use crate::cli::cli_config::CliConfig;
use std::process::Command;

use color_eyre::eyre::{ContextCompat, Ok, Result, bail};

#[derive(Debug)]
#[allow(dead_code)]
pub struct PacmanQsInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, clap::Subcommand)]
pub enum ToolsCommands {
    Install,
    List,
}

#[derive(Debug)]
pub struct Tools;

impl Tools {
    pub fn handle_cmd(_cfg: &CliConfig, _cmd: ToolsCommands) -> Result<()> {
        Ok(())
    }

    pub fn cmd_v(cmd: &String) -> Result<bool> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {}", cmd))
            .output()?;
        Ok(output.status.success())
    }

    pub fn get_pacman_qs() -> Result<Vec<PacmanQsInfo>> {
        let mut ret = Vec::new();
        let output = Command::new("pacman").arg("-Qs").output()?;
        if !output.status.success() {
            bail!(
                "command: pacman -Qs failed, {}",
                str::from_utf8(&output.stderr)?
            );
        }
        let output_stdout = str::from_utf8(&output.stdout)?;
        for line in output_stdout.lines() {
            if !line.starts_with("local/") {
                continue;
            }
            let (name, version) = line
                .strip_prefix("local/")
                .context(format!("strip_prefix failed: {}", line))?
                .split_once(" ")
                .context(format!("split_once failed: {}", line))?;
            ret.push(PacmanQsInfo {
                name: name.to_string(),
                version: version.to_string(),
            });
        }
        Ok(ret)
    }
}
