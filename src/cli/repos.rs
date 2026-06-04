use std::process::Command;

use color_eyre::eyre::{ContextCompat, Result, bail};
use tokio::task::spawn_blocking;

use crate::cli::cli_config::{CliConfig, RepoConfig};

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
        let repos_path = dirs::home_dir()
            .context("get home_dir failed")?
            .join(".test_moecly_conf");
        Ok(repos_path.join(repo_name).exists())
    }

    fn init_all(cfg: &CliConfig) -> Result<bool> {
        let repos = cfg.get_repos_config();
        repos.iter().for_each(|r| {
            let tmpr = RepoConfig {
                name: r.name.clone(),
                git_url: r.git_url.clone(),
            };
            spawn_blocking(move || {
                let ret = Self::init(&tmpr);
                match ret {
                    Ok(s) => {
                        println!("git clone {} {}, {}", tmpr.git_url, tmpr.name, s);
                    }
                    Err(e) => {
                        println!("git clone {} {}, {}", tmpr.git_url, tmpr.name, e);
                    }
                }
            });
        });

        Ok(true)
    }

    fn init(repo: &RepoConfig) -> Result<String> {
        let output = Command::new("git")
            .arg("clone")
            .arg(repo.git_url.as_str())
            .arg(repo.name.as_str())
            .current_dir(
                dirs::home_dir()
                    .context("get home_dir failed")?
                    .join(".test_moecly_conf"),
            )
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
