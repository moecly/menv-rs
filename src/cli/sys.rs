use std::process::Command;

use color_eyre::eyre::{Ok, Result, bail};
use tracing::info;

#[derive(Debug)]
#[allow(dead_code)]
pub struct PacmanQsInfo {
    pub name: String,
    pub version: String,
    pub desc: String,
}

#[derive(Debug)]
pub struct Sys;

impl Sys {
    pub fn get_pacman_qs() -> Result<Vec<PacmanQsInfo>> {
        let ret = Vec::new();
        let output = Command::new("pacman").arg("-Qs").output()?;
        if !output.status.success() {
            bail!("command: pacman -Qs failed, {}", str::from_utf8(&output.stderr)?);
        }
        info!("{}", str::from_utf8(&output.stdout)?);
        Ok(ret)
    }
}
