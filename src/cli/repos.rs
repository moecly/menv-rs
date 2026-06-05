use std::{process::Command, sync::Arc};

use color_eyre::eyre::{ContextCompat, Result, bail};
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
    pub async fn handle_cmd(cfg: &CliConfig, cmd: ReposCommands) -> Result<()> {
        match cmd {
            ReposCommands::Init => {
                Self::init_all(cfg).await?;
            }
            ReposCommands::List => {
                Self::list_all(cfg)?;
            }
            ReposCommands::Pull => {
                Self::pull_all(cfg).await?;
            }
            ReposCommands::Link => {
                Self::link_all(cfg).await?;
            }
        }
        Ok(())
    }

    pub fn repo_is_exist(repo_name: &String) -> Result<bool> {
        let repos_path = Common::get_cfg_path()?;
        Ok(repos_path.join(repo_name).exists())
    }

    async fn init_all(cfg: &CliConfig) -> Result<bool> {
        let repos = cfg.get_repos_config();
        let success = Arc::new(Mutex::new(0));
        let failed = Arc::new(Mutex::new(0));
        let total = repos.len();
        let mut handles = Vec::new();
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
            handles.push(spawn_blocking(move || {
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
            }));
        });

        for handle in handles {
            handle.await?;
        }

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

    async fn pull_all(cfg: &CliConfig) -> Result<()> {
        let repos = cfg.get_repos_config();
        let success = Arc::new(Mutex::new(0));
        let failed = Arc::new(Mutex::new(0));
        let total = repos.len();
        let mut handles = Vec::new();
        Common::print_emoji_title("⬇️", "Repository Pull");
        repos.iter().for_each(|r| {
            let tmpr = RepoConfig {
                name: r.name.clone(),
                git_url: r.git_url.clone(),
            };
            let success = Arc::clone(&success);
            let failed = Arc::clone(&failed);
            handles.push(spawn_blocking(move || match Self::pull(&tmpr) {
                Ok(_) => {
                    Common::print_progress_done(&tmpr.name);
                    *success.lock() += 1;
                }
                Err(e) => {
                    Common::print_progress_failed(&tmpr.name, &e.to_string());
                    *failed.lock() += 1;
                }
            }));
        });

        for handle in handles {
            handle.await?;
        }

        Common::print_summary(*success.lock(), *failed.lock(), total);
        Ok(())
    }

    fn pull(repo: &RepoConfig) -> Result<String> {
        let work_dir = Common::get_cfg_path()?.join(&repo.name);
        if !work_dir.exists() {
            bail!(
                "repo: {} not exists",
                work_dir.to_str().context("pathbuf to str failed")?
            );
        }

        let output = Command::new("git")
            .arg("pull")
            .current_dir(work_dir)
            .output()?;

        if !output.status.success() {
            bail!(
                "failed to git pull: {}, {}",
                repo.name,
                str::from_utf8(&output.stderr)?
            );
        }

        Ok(str::from_utf8(&output.stderr)?.to_string())
    }

    async fn link_all(cfg: &CliConfig) -> Result<()> {
        let repos = cfg.get_repos_config();
        let success = Arc::new(Mutex::new(0));
        let failed = Arc::new(Mutex::new(0));
        let total = repos.len();
        let mut handles = Vec::new();
        Common::print_emoji_title("🔗", "Repository Link");
        repos.iter().for_each(|r| {
            let tmpr = RepoConfig {
                name: r.name.clone(),
                git_url: r.git_url.clone(),
            };
            let success = Arc::clone(&success);
            let failed = Arc::clone(&failed);
            handles.push(spawn_blocking(move || match Self::link(&tmpr) {
                Ok(_) => {
                    Common::print_progress_done(&tmpr.name);
                    *success.lock() += 1;
                }
                Err(e) => {
                    Common::print_progress_failed(&tmpr.name, &e.to_string());
                    *failed.lock() += 1;
                }
            }));
        });

        for handle in handles {
            handle.await?;
        }

        Common::print_summary(*success.lock(), *failed.lock(), total);
        Ok(())
    }

    fn link(repo: &RepoConfig) -> Result<()> {
        let work_dir = Common::get_cfg_path()?.join(&repo.name);
        if !work_dir.exists() {
            bail!(
                "repo: {} not exists",
                work_dir.to_str().context("pathbuf to str failed")?
            );
        }

        let output = Command::new("sh")
            .arg("link.sh")
            .current_dir(work_dir)
            .output()?;

        if !output.status.success() {
            bail!(
                "failed to git pull: {}, {}",
                repo.name,
                str::from_utf8(&output.stderr)?
            );
        }
        Ok(())
    }

    fn list_all(cfg: &CliConfig) -> Result<()> {
        let repos = cfg.get_repos_config();
        let total = repos.len();
        let mut cloned_count = 0;

        Common::print_emoji_title("📋", "Repository List");
        for (idx, r) in repos.iter().enumerate() {
            if Self::repo_is_exist(&r.name)? {
                cloned_count += 1;
                Common::print_success(&format!("{}. {} ({})", idx + 1, r.name, r.git_url));
            } else {
                Common::print_error(&format!("{}. {} ({})", idx + 1, r.name, r.git_url));
            }
        }

        Common::print_summary(cloned_count, total - cloned_count, total);

        Ok(())
    }
}
