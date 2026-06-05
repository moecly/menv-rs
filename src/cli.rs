use clap::Parser;
use color_eyre::eyre::{Ok, Result};

use crate::cli::{cli_config::CliConfig, repos::ReposCommands, tools::ToolsCommands};

mod cli_config;
mod common;
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
    let cli_cfg = CliConfig::load()?;
    match cli.commands {
        Commands::Status => {
            status::Status::handle_cmd(&cli_cfg)?;
        }
        Commands::Repos { repos_cmd } => repos::Repos::handle_cmd(&cli_cfg, repos_cmd)?,
        Commands::Tools { tools_cmd } => {
            tools::Tools::handle_cmd(&cli_cfg, tools_cmd)?;
        }
    }
    Ok(())
}
