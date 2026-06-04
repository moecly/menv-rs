use clap::Parser;
use color_eyre::eyre::{Ok, Result};

use crate::cli::{repos::ReposCommands, tools::ToolsCommands};

mod repos;
mod status;
mod tools;

#[derive(Debug, clap::Parser)]
#[command(name = "menv", about = "a dotfiles manager tool")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    Status,
    Repos {
        #[command(subcommand)]
        repos_cmd: ReposCommands,
    },
    Tools {
        #[command(subcommand)]
        tools_cmd: ToolsCommands,
    },
}

pub fn cli_main() -> Result<()> {
    let cli = Cli::parse();
    match cli.commands {
        Commands::Status => {}
        Commands::Repos { repos_cmd: _ } => {}
        Commands::Tools { tools_cmd: _ } => {}
    }
    Ok(())
}
