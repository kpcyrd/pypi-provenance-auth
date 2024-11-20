use clap::ArgAction;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct Args {
    /// Increase logging output (can be used multiple times)
    #[arg(short, long, global = true, action(ArgAction::Count))]
    pub verbose: u8,
    /// Assume this git commit instead of auto-detecting
    #[arg(long)]
    pub commit: Option<String>,
    /// Verify the attestation subject matches this expected string
    #[arg(long)]
    pub subject: String,
    /// Verify the repository embedded in the attestation matches this expected value
    #[arg(long)]
    pub repository: Option<String>,
    /// Path to file containing attestation json data
    #[arg(long)]
    pub attestation_file: PathBuf,
    /// When auto-detecting the git commit, run git in this path
    #[arg(short = 'C', long, default_value = ".")]
    pub git_path: PathBuf,
}
