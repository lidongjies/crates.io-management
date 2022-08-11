// https://doc.rust-lang.org/cargo/reference/config.html#configuration-format

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct BuildConfig {
    pub jobs: Option<i8>,      // number of parallel jobs, defaults to # of CPUs
    pub rustc: Option<String>, // the rust compiler tool
    #[serde(rename(serialize = "rustc-wrapper"))]
    #[serde(alias = "rustc-wrapper")]
    pub rustc_wrapper: Option<String>, // run this wrapper instead of `rustc`
    #[serde(rename(serialize = "rustc_workspace_wrapper"))]
    #[serde(alias = "rustc_workspace_wrapper")]
    pub rustc_workspace_wrapper: Option<String>, // run this wrapper instead of `rustc` for workspace members
    pub rustdoc: Option<String>, // the doc generator tool
    pub target: Option<String>,  // build for the target triple (ignored by `cargo install`)
    #[serde(rename(serialize = "target_dir"))]
    #[serde(alias = "target_dir")]
    pub target_dir: Option<String>, // path of where to place all generated artifacts
    pub rustflags: Option<Vec<String>>, // custom flags to pass to all compiler invocations
    pub rustdocflags: Option<Vec<String>>, // custom flags to pass to rustdoc
    pub incremental: Option<bool>, // whether or not to enable incremental compilation
    #[serde(rename(serialize = "dep-info-basedir"))]
    #[serde(alias = "dep-info-basedir")]
    pub dep_info_basedir: Option<String>, // path for the base directory for targets in depfiles
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DocConfig {
    pub browser: Option<String>, // browser to use with `cargo doc --open`, overrides the `BROWSER` environment variable
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EnvConfig {
    pub value: String,          // Set ENV_VAR_NAME=value for any process run by Cargo
    pub force: Option<bool>,    // Set even if already present in environment
    pub relative: Option<bool>, // Value is relative to .cargo directory containing `config.toml`, make absolute
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FeatureIncompatReportConfig {
    pub frequency: Option<String>, // when to display a notification about a future incompat report
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CargoNewConfig {
    pub vcs: Option<String>, // VCS to use ('git', 'hg', 'pijul', 'fossil', 'none')
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HttpConfig {
    pub debug: Option<bool>,         //  HTTP debugging
    pub proxy: Option<String>,       // HTTP proxy in libcurl format
    pub ssl_version: Option<String>, // TLS version to use
    #[serde(rename(serialize = "ssl-version.max"))]
    #[serde(alias = "ssl-version.max")]
    pub ssl_version_max: Option<String>, // maximum TLS version
    #[serde(rename(serialize = "ssl-version.min"))]
    #[serde(alias = "ssl-version.min")]
    pub ssl_version_min: Option<String>, // minimum TLS version
    pub timeout: Option<u32>,        // timeout for each HTTP request, in seconds
    #[serde(rename(serialize = "low-speed-limit"))]
    #[serde(alias = "low-speed-limit")]
    pub low_speed_limit: Option<u32>, // network timeout threshold (bytes/sec)
    pub cainfo: Option<String>,      // path to Certificate Authority (CA) bundle
    #[serde(rename(serialize = "check-revoke"))]
    #[serde(alias = "check-revoke")]
    pub check_revoke: Option<bool>, // check for SSL certificate revocation
    pub multiplexing: Option<bool>,  //  HTTP/2 multiplexing
    #[serde(rename(serialize = "user-agent"))]
    #[serde(alias = "user-agent")]
    pub user_agent: Option<String>, // the user-agent header
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstallConfig {
    pub root: Option<String>, // `cargo install` destination directory
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NetConfig {
    pub retry: Option<i8>, // network retries
    #[serde(rename(serialize = "git-fetch-with-ci"))]
    #[serde(alias = "git-fetch-with-ci")]
    pub git_fetch_with_ci: Option<bool>, // use the `git` executable for git operations
    pub offline: Option<bool>, // do not access the network
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PatchConfig {} // Same keys as for [patch] in Cargo.toml

#[derive(Deserialize, Serialize, Debug)]
pub struct ProfileConfig {
    pub opt_level: Option<i8>, // Optimization level.
    pub debug: Option<bool>,
    #[serde(rename(serialize = "split-debuginfo"))]
    #[serde(alias = "split-debuginfo")] // Include debug info.
    pub split_debuginfo: Option<String>, // Include debug info.
    #[serde(rename(serialize = "debug-assertions"))]
    #[serde(alias = "debug-assertions")]
    pub debug_assertions: Option<bool>, // Enables debug assertions.
    #[serde(rename(serialize = "overflow-checks"))]
    #[serde(alias = "overflow-checks")]
    pub overflow_checks: Option<bool>, // Enables runtime integer overflow checks.
    pub lto: Option<bool>,         // Sets link-time optimization.
    pub panic: Option<String>,     //  The panic strategy.
    pub incremental: Option<bool>, // Incremental compilation.
    #[serde(rename(serialize = "codegen-units"))]
    #[serde(alias = "codegen-units")]
    pub codegen_units: Option<i8>, // Number of code generation units.
    pub rpath: Option<String>,     // Sets the rpath linking option.
    #[serde(rename(serialize = "build-override"))]
    #[serde(alias = "build-override")]
    pub build_override: Option<ProfileConfigPartial>, // Overrides build-script settings. Same keys for a normal profile.
    pub package: Option<HashMap<String, ProfileConfigPartial>>, // Override profile for a package. Same keys for a normal profile (minus `panic`, `lto`, and `rpath`).
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProfileConfigPartial {
    #[serde(rename(serialize = "opt-level"))]
    #[serde(alias = "opt-level")]
    pub opt_level: Option<i8>,
    pub debug: Option<bool>,
    #[serde(rename(serialize = "split-debuginfo"))]
    #[serde(alias = "split-debuginfo")]
    pub split_debuginfo: Option<String>,
    #[serde(rename(serialize = "debug-assertions"))]
    #[serde(alias = "debug-assertions")]
    pub debug_assertions: Option<bool>,
    #[serde(rename(serialize = "overflow-checks"))]
    #[serde(alias = "overflow-checks")]
    pub overflow_checks: Option<bool>,
    pub lto: Option<bool>,
    pub panic: Option<String>,
    pub incremental: Option<bool>,
    #[serde(rename(serialize = "codegen-units"))]
    #[serde(alias = "codegen-units")]
    pub codegen_units: Option<i8>,
    pub rpath: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RgistriesConfig {
    pub index: Option<String>, // URL of the registry index
    pub token: Option<String>, // authentication token for the registry
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegistryConfig {
    pub default: Option<String>, // name of the default registry
    pub token: Option<String>,   // authentication token for crates.io
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SourceConfig {
    #[serde(rename(serialize = "replace-with"))]
    #[serde(alias = "replace-with")]
    pub replace_with: Option<String>, // path to a directory source
    pub directory: Option<String>, // URL to a registry source
    pub registry: Option<String>,  // replace this source with the given named source
    #[serde(rename(serialize = "local-registry"))]
    #[serde(alias = "local-registry")]
    pub local_registry: Option<String>, // path to a local registry source
    pub git: Option<String>,       // URL of a git repository source
    pub branch: Option<String>,    // branch name for the git repository
    pub tag: Option<String>,       // tag name for the git repository
    pub rev: Option<String>,       // revision for the git repository
    pub home: Option<String>,      // home page for source
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TargetConfig {
    pub linker: Option<String>,         // linker to use
    pub runner: Option<String>,         // wrapper to run executables
    pub rustflags: Option<Vec<String>>, // custom flags for `rustc`
    pub links: Option<LinksConfig>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LinksConfig {
    pub rustc_link_lib: Option<Vec<String>>,
    pub rustc_link_search: Option<Vec<String>>,
    pub rustc_flags: Option<Vec<String>>,
    pub rustc_cfg: Option<Vec<String>>,
    pub rustc_env: Option<Vec<String>>,
    pub rustc_cdylib_link_arg: Option<Vec<String>>,
    pub metadata_key1: Option<String>,
    pub metadata_key2: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProgressConfig {
    pub when: Option<String>, // whether cargo shows progress bar
    pub width: Option<i8>,    // width of progress bar
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TermConfig {
    pub quiet: Option<bool>,   // whether cargo output is quiet
    pub verbose: Option<bool>, // whether cargo provides verbose output
    pub color: Option<String>, // whether cargo colorizes output
    pub progress: Option<ProgressConfig>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    // pub paths: Option<Vec<String>>,
    // pub alias: Option<HashMap<String, String>>,
    // pub build: Option<BuildConfig>,
    // pub doc: Option<DocConfig>,
    // pub env: Option<HashMap<String, EnvConfig>>,
    // pub feature_incompat_report: Option<FeatureIncompatReportConfig>,
    // pub cargo_new: Option<CargoNewConfig>,
    // pub http: Option<HttpConfig>,
    // pub install: Option<InstallConfig>,
    // pub net: Option<NetConfig>,
    // pub patch: Option<HashMap<String, PatchConfig>>,
    // pub profile: Option<HashMap<String, ProfileConfig>>,
    // pub registries: Option<HashMap<String, RgistriesConfig>>,
    // pub registry: Option<RegistryConfig>,
    pub source: Option<HashMap<String, SourceConfig>>,
    // pub target: Option<HashMap<String, TargetConfig>>,
    // pub term: Option<TermConfig>,
}
