use color_eyre::eyre::Result;
use tracing::info;

use crate::cli::{cli_config::CliConfig, sys::Sys};

pub fn handle_cmd(cfg: &CliConfig) -> Result<()> {
    let tools_cfg = cfg.get_tools_config();
    let repos_cfg = cfg.get_repos_config();
    tools_cfg.iter().for_each(|t| info!("{}", t.tool_name));
    repos_cfg.iter().for_each(|r| info!("{}", r.name));
    let _pacman_qs_info = Sys::get_pacman_qs();
    Ok(())
}
