use color_eyre::eyre::Result;

use crate::cli::cli_config::CliConfig;

#[derive(Debug, clap::Subcommand)]
pub enum ToolsCommands {
    Install,
    List,
}

pub fn handle_cmd(_cfg: &CliConfig, _cmd: ToolsCommands) -> Result<()> {
    Ok(())
}
