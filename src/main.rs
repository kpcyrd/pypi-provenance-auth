mod args;
mod attestation;
mod certificate;
mod errors;

use crate::args::Args;
use crate::errors::*;
use clap::Parser;
use env_logger::Env;
use std::ffi::OsStr;
use std::fs;
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    let args = Args::parse();

    let log_level = match args.verbose {
        0 => "warn,pypi_provenance_auth=info",
        1 => "info,pypi_provenance_auth=debug",
        2 => "debug",
        _ => "trace",
    };
    env_logger::init_from_env(Env::default().default_filter_or(log_level));

    // read data
    let data = fs::read(&args.attestation_file).with_context(|| {
        anyhow!(
            "Failed to read attestation file: {:?}",
            args.attestation_file
        )
    })?;
    let attestation = attestation::parse(&data)?;

    // determine current commit
    let commit = if let Some(commit) = args.commit {
        commit
    } else {
        let output = Command::new("git")
            .args([
                OsStr::new("-C"),
                args.git_path.as_os_str(),
                OsStr::new("rev-parse"),
                OsStr::new("HEAD"),
            ])
            .stdout(Stdio::piped())
            .spawn()
            .context("Failed to run git binary")?
            .wait_with_output()
            .context("Failed to wait for git child")?;
        if !output.status.success() {
            bail!("Git did not exit successfully");
        }
        let mut output =
            String::from_utf8(output.stdout).context("Git output contains invalid utf8")?;
        output.truncate(output.find('\n').unwrap_or(output.len()));
        output
    };
    debug!("Using git commit: {commit:?}");

    // TODO: write more code
    attestation.commit;

    Ok(())
}
