use color_eyre::eyre::{Ok, Result};
use config::Config;
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

const REPOS_TOML: &str = include_str!("../../config/repos.toml");
const TOOLS_TOML: &str = include_str!("../../config/tools.toml");

impl CliConfig {
    pub fn load() -> Result<Self> {
        let cfg = Config::builder()
            .add_source(config::File::from_str(REPOS_TOML, config::FileFormat::Toml))
            .add_source(config::File::from_str(TOOLS_TOML, config::FileFormat::Toml))
            .build()?;

        Ok(cfg.try_deserialize()?)
    }

    pub fn get_tools_config(&self) -> &Vec<ToolConfig> {
        &self.tools_config
    }

    pub fn get_repos_config(&self) -> &Vec<RepoConfig> {
        &self.repos_config
    }
}
