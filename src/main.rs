use clap::Parser;
use cli::Cli;

use crate::app::App;

mod action;
mod app;
mod cli;
mod components;
mod config;
mod errors;
mod git_path;
mod git_repo;
mod git_worktree;
mod logging;
mod tui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    crate::errors::init()?;
    crate::logging::init()?;

    let args = Cli::parse();
    //TODO: add path to args list
    let mut app = App::new(args.tick_rate, args.frame_rate, None)?;
    app.run().await?;
    Ok(())
}
