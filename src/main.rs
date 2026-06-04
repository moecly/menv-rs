use color_eyre::eyre::{Context, Ok, Result};
use tracing_subscriber::{EnvFilter, fmt};

use crate::cli::cli_main;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("info".into()))
        .with_file(true)
        .with_line_number(true)
        .init();
    cli_main().context("cli main failed")?;
    Ok(())
}
