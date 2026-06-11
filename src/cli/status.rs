use color_eyre::eyre::Result;

use crate::cli::{cli_config::CliConfig, common::Common, repos::Repos, tools::Tools};

#[derive(Debug)]
pub struct Status;

impl Status {
    pub fn handle_cmd(cfg: &CliConfig) -> Result<()> {
        let tools_cfg = cfg.get_tools_config();
        let repos_cfg = cfg.get_repos_config();
        let pacman_qs_info = Tools::get_pacman_qs()?;
        let mut tools_install = 0;
        let tools_total = tools_cfg.len();
        let mut repos_install = 0;
        let repos_total = repos_cfg.len();

        for t in tools_cfg {
            if let Ok(v) = Tools::cmd_v(&t.command)
                && v
            {
                tools_install += 1;
                continue;
            }

            for info in &pacman_qs_info {
                if info.name == t.package_name {
                    tools_install += 1;
                    break;
                }
            }
        }

        for r in repos_cfg {
            if let Ok(v) = Repos::repo_is_exist(&r.name)
                && v
            {
                repos_install += 1;
            }
        }

        Common::print_emoji_title("📊", "Status Overview");
        Common::print_success(&format!("Tools: {}/{}", tools_install, tools_total));
        Common::print_success(&format!("Repositories: {}/{}", repos_install, repos_total));

        Ok(())
    }
}
