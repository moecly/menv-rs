use color_eyre::eyre::{ContextCompat, Ok, Result};

use crate::cli::cli_config::CliConfig;

#[derive(Debug, clap::Subcommand)]
pub enum ReposCommands {
    Init,
    List,
    Pull,
    Link,
}

#[derive(Debug)]
pub struct Repos;

impl Repos {
    pub fn handle_cmd(_cfg: &CliConfig, _cmd: ReposCommands) -> Result<()> {
        Ok(())
    }

    pub fn repo_is_exist(repo_name: &String) -> Result<bool> {
        let repos_path = dirs::home_dir()
            .context("get home_dir failed")?
            .join(".moecly_conf");
        Ok(repos_path.join(repo_name).exists())
    }
}
