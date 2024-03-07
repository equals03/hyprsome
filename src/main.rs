use std::ops::RangeInclusive;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod commands;
mod hyprland_ipc;

use hyprland_ipc::{client, workspace};

pub const MAX_GROUP_WS: i32 = 10;

const INDEX_RANGE: RangeInclusive<i32> = 0..=(MAX_GROUP_WS - 1);
fn index_in_range(s: &str) -> Result<i32, String> {
    let index: i32 = s.parse().map_err(|_| {
        format!(
            "`{s}` isn't a number within the range of {}-{}",
            INDEX_RANGE.start(),
            INDEX_RANGE.end()
        )
    })?;
    if INDEX_RANGE.contains(&index) {
        Ok(index)
    } else {
        Err(format!(
            "index not in range {}-{}",
            INDEX_RANGE.start(),
            INDEX_RANGE.end()
        ))
    }
}
#[derive(Parser)]
#[command(name = "hyprsome")]
#[command(author = "sopa0")]
#[command(version = "0.1.11")]
#[command(about = "Makes hyprland workspaces behave like awesome", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Clone, Debug)]
struct Index {
    #[clap(value_name = "index", help = "The workspace index to work with", value_parser = index_in_range)]
    index: i32,
}

#[derive(Args, Clone, Debug)]
struct IndexWithSilent {
    #[clap(value_name = "index", help = "The workspace index to work with", value_parser = index_in_range)]
    index: i32,

    #[clap(short, long, help = "Is the action silent?")]
    silent: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Initialize the workspace groups for all the outputs")]
    Init(Index),
    #[clap(about = "Switch to the workspace on the current output")]
    Workspace(Index),
    #[clap(about = "Switch to the workspace on all available outputs")]
    WorkspaceAll(Index),

    #[clap(about = "Move the focused container to another workspace")]
    MoveToWorkspace(IndexWithSilent),

    #[clap(about = "Move the current workspace to the correct workspace on the specified monitor")]
    MoveCurrentWorkspaceToMonitor(Index),

    #[clap(
        about = "Rearrange already opened workspaces to the correct outputs, useful when plugging new monitors"
    )]
    RearrangeWorkspaces,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init(action) => commands::init_workspaces(action.index),
        Commands::RearrangeWorkspaces => commands::rearrange_workspaces(),
        Commands::MoveToWorkspace(action) => {
            commands::move_to_workspace(action.index, action.silent)
        }

        Commands::MoveCurrentWorkspaceToMonitor(action) => {
            commands::move_current_workspace_to_monitor(action.index, false)
        }
        Commands::Workspace(action) => commands::workspace(action.index),
        Commands::WorkspaceAll(action) => commands::workspace_all(action.index),
    }
}
