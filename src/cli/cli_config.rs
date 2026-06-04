use color_eyre::eyre::{Context, Ok, Result};
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RepoConfig {
    pub name: String,
    pub git_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ToolConfig {
    pub tool_name: String,
    pub package_name: String,
    pub package_source: String,
    pub category: String,
    pub desc: String,
    pub command: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CliConfig {
    #[serde(rename = "tools")]
    tools_config: Vec<ToolConfig>,
    #[serde(rename = "repos")]
    repos_config: Vec<RepoConfig>,
}

impl CliConfig {
    pub fn load() -> Result<Self> {
        let cfg = Config::builder()
            .add_source(File::with_name("config/repos"))
            .add_source(File::with_name("config/tools"))
            .build()
            .context("Load config failed")?;
        let cli_cfg: Self = cfg.try_deserialize().context("config deserialize failed")?;
        Ok(cli_cfg)
    }

    pub fn get_tools_config(&self) -> &Vec<ToolConfig> {
        &self.tools_config
    }

    pub fn get_repos_config(&self) -> &Vec<RepoConfig> {
        &self.repos_config
    }
}
