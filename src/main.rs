use color_eyre::eyre::{Ok, Result};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("info".into()))
        .with_file(true)
        .with_line_number(true)
        .init();
    info!("helloworld");
    Ok(())
}
