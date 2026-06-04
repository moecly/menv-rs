#[derive(Debug, clap::Subcommand)]
pub enum ReposCommands {
    Init,
    List,
    Pull,
    Link,
}

