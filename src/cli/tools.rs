use crate::cli::{cli_config::CliConfig, common::Common};
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
    pub fn handle_cmd(cfg: &CliConfig, cmd: ToolsCommands) -> Result<()> {
        match cmd {
            ToolsCommands::Install => {
                Self::install_all(cfg)?;
            }
            ToolsCommands::List => {
                Self::list_all(cfg)?;
            }
        }
        Ok(())
    }

    pub fn install_all(cfg: &CliConfig) -> Result<()> {
        let mut pacman_need_install_tools = Vec::new();
        let mut paru_need_install_tools = Vec::new();
        let tools = cfg.get_tools_config();
        let pacman_qs_info = Self::get_pacman_qs()?;
        let mut success = 0;
        let mut failed = 0;
        let total = tools.len();

        Common::print_emoji_title("🔧", "Checking and Installing Tools");

        for t in tools {
            Common::print_progress(&t.package_name);
            if Self::cmd_v(&t.command)? {
                Common::print_progress_done(&t.package_name);
                success += 1;
                continue;
            }

            let mut find_in_pacman = false;
            for info in &pacman_qs_info {
                if t.package_name == info.name {
                    find_in_pacman = true;
                    break;
                }
            }

            if find_in_pacman {
                Common::print_progress_done(&t.package_name);
                success += 1;
                continue;
            }

            match t.package_source.as_str() {
                "pacman" => {
                    Common::print_progress_failed(&t.package_name, "need install");
                    pacman_need_install_tools.push(&t.package_name);
                }
                "paru" => {
                    Common::print_progress_failed(&t.package_name, "need install");
                    paru_need_install_tools.push(&t.package_name);
                }
                _ => {
                    Common::print_progress_failed(&t.package_name, "unknown source");
                    failed += 1;
                }
            }
        }

        if !pacman_need_install_tools.is_empty() {
            let mut pacman_tools_str = String::new();
            pacman_need_install_tools.iter().for_each(|tool| {
                if !pacman_tools_str.is_empty() {
                    pacman_tools_str.push(' ');
                }
                pacman_tools_str.push_str(tool);
            });
            let output = Command::new("sudo")
                .arg("pacman")
                .arg("-S")
                .arg("--needed")
                .arg("--noconfirm")
                .arg(pacman_tools_str)
                .output()?;
            if output.status.success() {
                Common::print_success(&format!(
                    "Installed {} packages",
                    pacman_need_install_tools.len()
                ));
                success += pacman_need_install_tools.len();
            } else {
                let error_msg = str::from_utf8(&output.stderr)?;
                Common::print_error(&format!("Installation failed: {}", error_msg));
                failed += pacman_need_install_tools.len();
                // bail!("failed to install: {}", error_msg);
            }
        }

        if !paru_need_install_tools.is_empty() {
            let mut paru_tools_str = String::new();
            paru_need_install_tools.iter().for_each(|tool| {
                if !paru_tools_str.is_empty() {
                    paru_tools_str.push(' ');
                }
                paru_tools_str.push_str(tool);
            });
            let output = Command::new("sudo")
                .arg("paru")
                .arg("-S")
                .arg("--needed")
                .arg("--noconfirm")
                .arg(paru_tools_str)
                .output()?;

            if output.status.success() {
                Common::print_success(&format!(
                    "Installed {} packages",
                    paru_need_install_tools.len()
                ));
                success += paru_need_install_tools.len();
            } else {
                let error_msg = str::from_utf8(&output.stderr)?;
                Common::print_error(&format!("Installation failed: {}", error_msg));
                failed += paru_need_install_tools.len();
                // bail!("failed to install: {}", error_msg);
            }
        }

        Common::print_summary(success, failed, total);
        Ok(())
    }

    pub fn list_all(cfg: &CliConfig) -> Result<()> {
        let tools = cfg.get_tools_config();
        let pacman_qs_info = Self::get_pacman_qs()?;
        let mut success = 0;
        let mut failed = 0;
        let total = tools.len();

        Common::print_emoji_title("📋", "Tools List");
        for (idx, tool) in tools.iter().enumerate() {
            let is_installed = Self::cmd_v(&tool.command)?;
            let in_pacman = if !is_installed {
                pacman_qs_info.iter().any(|i| i.name == tool.package_name)
            } else {
                false
            };

            if is_installed || in_pacman {
                success += 1;
                Common::print_success(&format!(
                    "{}. [{}] {} ({})",
                    idx + 1,
                    tool.package_source,
                    tool.package_name,
                    tool.desc
                ));
            } else {
                failed += 1;
                Common::print_error(&format!(
                    "{}. [{}] {} ({})",
                    idx + 1,
                    tool.package_source,
                    tool.package_name,
                    tool.desc
                ));
            }
        }
        Common::print_summary(success, failed, total);
        Ok(())
    }

    pub fn cmd_v(cmd: &str) -> Result<bool> {
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
