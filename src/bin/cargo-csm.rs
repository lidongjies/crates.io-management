use anyhow::Result;
use cargo_csm::commands::list::ListCommand;
use clap::Parser;
use lazy_static::lazy_static;

#[tokio::main]
async fn main() -> Result<()> {
    CSMApp::parse().run().await?;
    Ok(())
}

#[derive(Parser)]
#[clap(name = "cargo-csm", version = version())]
enum CSMApp {
    List(ListCommand),
    // Current(CurrentCommand),
    // Add(AddCommand),
    // Del(DelCommand),
    // Home(HomeCommand),
    // Test(TestCommand),
    // Use(UseCommand),
}

impl CSMApp {
    async fn run(self) -> Result<()> {
        match self {
            Self::List(cmd) => cmd.run().await,
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
