use anyhow::Result;
use cargo_csm::commands::{
    add::AddCommand, current::CurrentCommand, delete::DeleteCommand, list::ListCommand,
    set::SetCommand,
};
use clap::Parser;
use lazy_static::lazy_static;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    CSMApp::parse().run().await?;
    Ok(())
}

#[derive(Parser)]
#[clap(name = "cargo-csm", version = version(), about, author)]
enum CSMApp {
    /// List all the sources
    List(ListCommand),
    /// Change crates-io replace-with to custom source
    Use(SetCommand),
    /// Show current source name
    Current(CurrentCommand),
    /// Add one custom source
    Add(AddCommand),
    /// Delete one custom source
    Del(DeleteCommand),
    // Test(TestCommand),
}

impl CSMApp {
    async fn run(self) -> Result<()> {
        match self {
            Self::List(cmd) => cmd.run().await,
            Self::Use(cmd) => cmd.run().await,
            Self::Current(cmd) => cmd.run().await,
            Self::Add(cmd) => cmd.run().await,
            Self::Del(cmd) => cmd.run().await,
        }
    }
}

lazy_static! {
    pub static ref VERSION: String = build_info();
}

fn version() -> &'static str {
    &VERSION
}

fn build_info() -> String {
    format!(
        "{} ({} {})",
        env!("VERGEN_BUILD_SEMVER"),
        env!("VERGEN_GIT_SHA_SHORT"),
        env!("VERGEN_GIT_COMMIT_DATE")
    )
}
