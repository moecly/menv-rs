use color_eyre::eyre::Result;

use crate::cli::cli_config::CliConfig;

mod boot_default;
mod hibernate;
mod locale;
mod paru;
mod service;
mod swapfile;
mod trim;

#[derive(Debug, clap::Subcommand)]
pub enum ScriptsCommand {
    BootDefault,
    Hibernate,
    Locale,
    Paru,
    Service,
    Swapfile,
    Trim,
}

#[derive(Debug)]
pub struct Scripts;

impl Scripts {
    pub fn handle_cmd(_cfg: &CliConfig, cmd: ScriptsCommand) -> Result<()> {
        match cmd {
            ScriptsCommand::BootDefault => boot_default::run()?,
            ScriptsCommand::Hibernate => hibernate::run()?,
            ScriptsCommand::Locale => locale::run()?,
            ScriptsCommand::Paru => paru::run()?,
            ScriptsCommand::Service => service::run()?,
            ScriptsCommand::Swapfile => swapfile::run()?,
            ScriptsCommand::Trim => trim::run()?,
        }
        Ok(())
    }
}
