mod common;
use crate::common::helpers::Fixture;
use crate::common::temp_directory::TempDirectory;
use git_treefrog::app::App;
use insta::assert_snapshot;
use ratatui::{Terminal, backend::TestBackend};
use std::path::Path;

#[tokio::test]
async fn renders_worktree_list() -> color_eyre::Result<()> {
    let temp = TempDirectory::new(Path::new("tmp/repo").to_path_buf())?;
    let mut fixture = Fixture::new(&temp.path);
    fixture.create_worktree("worktree1");
    fixture.create_worktree("worktree2");
    let mut app = App::new(4.0, 60.0, Some(temp.path.clone()))?;
    let mut terminal = Terminal::new(TestBackend::new(100, 20)).unwrap();
    terminal.draw(|frame| {
        app.draw(frame, frame.area()).unwrap();
    });
    assert_snapshot!(terminal.backend());
    Ok(())
}
