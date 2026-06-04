use color_eyre::eyre::Result;

use crate::cli::cli_config::CliConfig;

#[derive(Debug, clap::Subcommand)]
pub enum ReposCommands {
    Init,
    List,
    Pull,
    Link,
}

pub fn handle_cmd(_cfg: &CliConfig, _cmd: ReposCommands) -> Result<()> {
    Ok(())
}
