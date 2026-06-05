use std::{process::Command, sync::Arc};

use color_eyre::eyre::{Result, bail};
use parking_lot::Mutex;
use tokio::task::spawn_blocking;

use crate::cli::{
    cli_config::{CliConfig, RepoConfig},
    common::Common,
};

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
    pub fn handle_cmd(cfg: &CliConfig, cmd: ReposCommands) -> Result<()> {
        match cmd {
            ReposCommands::Init => {
                Self::init_all(cfg)?;
            }
            ReposCommands::List => {}
            ReposCommands::Pull => {}
            ReposCommands::Link => {}
        }
        Ok(())
    }

    pub fn repo_is_exist(repo_name: &String) -> Result<bool> {
        let repos_path = Common::get_cfg_path()?;
        Ok(repos_path.join(repo_name).exists())
    }

    fn init_all(cfg: &CliConfig) -> Result<bool> {
        let repos = cfg.get_repos_config();
        let success = Arc::new(Mutex::new(0));
        let failed = Arc::new(Mutex::new(0));
        let total = repos.len();
        Common::print_emoji_title("📦", "Repository Initialization");
        repos.iter().for_each(|r| {
            let tmpr = RepoConfig {
                name: r.name.clone(),
                git_url: r.git_url.clone(),
            };

            let success = Arc::clone(&success);
            let failed = Arc::clone(&failed);
            if let Ok(v) = Repos::repo_is_exist(&tmpr.name)
                && v
            {
                Common::print_progress_done(&tmpr.name);
                *success.lock() += 1;
                return;
            }
            spawn_blocking(move || {
                let ret = Self::init(&tmpr);
                match ret {
                    Ok(_) => {
                        Common::print_progress_done(&tmpr.name);
                        *success.lock() += 1;
                    }
                    Err(e) => {
                        Common::print_progress_failed(&tmpr.name, &e.to_string());
                        *failed.lock() += 1;
                    }
                }
            });
        });
        Common::print_summary(*success.lock(), *failed.lock(), total);
        Ok(true)
    }

    fn init(repo: &RepoConfig) -> Result<String> {
        let output = Command::new("git")
            .arg("clone")
            .arg(repo.git_url.as_str())
            .arg(repo.name.as_str())
            .current_dir(Common::get_cfg_path()?)
            .output()?;
        if !output.status.success() {
            bail!(
                "failed to git clone: {}, {}",
                repo.name,
                str::from_utf8(&output.stderr)?
            );
        }

        Ok(str::from_utf8(&output.stderr)?.to_string())
    }
}
