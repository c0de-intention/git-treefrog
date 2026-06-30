use super::Component;
use crate::git_repo::GitRepo;
use crate::git_worktree::GitWorktree;
use crate::{action::Action, config::Config};
use color_eyre::eyre::Context;
use crossterm::ExecutableCommand;
use crossterm::event::{KeyCode, KeyEvent};
use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Padding, Row, Table, TableState};
use std::io::stdout;
use std::process::Command;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Default)]
pub struct WorktreeList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    table_items: Vec<GitWorktree>,
    table_state: TableState,
}

impl WorktreeList {
    pub fn new() -> Self {
        Self::default()
    }

    fn run_editor(&self, path: &str) -> color_eyre::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Command::new("zed").arg(path).status()?;
        Ok(())
    }
}

impl Component for WorktreeList {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> color_eyre::Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> color_eyre::Result<()> {
        self.config = config;
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Enter if self.table_state.selected().is_some() => {
                if let Some(current_path_index) = self.table_state.selected() {
                    let path = &self.table_items[current_path_index];
                    return Ok(Some(Action::WorktreeOpenEditor(path.path.to_string())));
                }
                color_eyre::eyre::bail!("expected to have selected path");
            }
            KeyCode::Up if self.table_state.selected().is_some() => {
                Ok(Some(Action::WorktreePrevious))
            }
            KeyCode::Char('k') if self.table_state.selected().is_some() => {
                Ok(Some(Action::WorktreePrevious))
            }
            KeyCode::Down if self.table_state.selected().is_some() => {
                Ok(Some(Action::WorktreeNext))
            }
            KeyCode::Char('j') if self.table_state.selected().is_some() => {
                Ok(Some(Action::WorktreeNext))
            }
            _ => Ok(None),
        }
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            Action::WorktreeUpdate => {
                let repo = GitRepo::new();
                let worktrees = repo.get_worktrees().context("Unable to get worktrees")?;
                if !&worktrees.is_empty() && self.table_state.selected().is_none() {
                    self.table_state.select(Some(0))
                }
                for worktree in worktrees {
                    self.table_items.push(worktree);
                }
            }
            Action::WorktreePrevious => {
                self.table_state.select_previous();
            }
            Action::WorktreeNext => {
                self.table_state.select_next();
            }
            Action::WorktreeOpenEditor(path) => {
                self.run_editor(&path)?;
                let action_tx = self.command_tx.clone();
                if let Some(action_tx) = action_tx {
                    action_tx.send(Action::Quit)?;
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let header = Row::new(["Branch", "Path"])
            .style(Style::new().bold())
            .bottom_margin(1);

        let rows: Vec<Row> = self
            .table_items
            .iter()
            .map(|item| Row::new([item.branch.clone(), item.path.to_string()]))
            .collect();

        let widths = [Constraint::Percentage(20), Constraint::Percentage(80)];
        let table = Table::new(rows, widths)
            .block(
                Block::bordered()
                    .padding(Padding {
                        left: 1,
                        right: 1,
                        top: 0,
                        bottom: 0,
                    })
                    .title("Worktree selector"),
            )
            .header(header)
            .column_spacing(1)
            .style(Color::White)
            .row_highlight_style(Style::new().on_black().bold())
            .column_highlight_style(Color::Gray)
            .highlight_symbol("🐸 ");

        frame.render_stateful_widget(table, area, &mut self.table_state);
        Ok(())
    }
}
