mod run;
mod test;
mod utils;

pub use run::RunConfig;
pub use test::TestConfig;

use crate::trace::iter::Traces;
use anyhow::{Context, Result, anyhow};
use std::{path::Path, process::Command};
use tempfile::TempDir;

/// Default number of traces to generate when not specified.
const DEFAULT_TRACES: usize = 100;

/// Internal trait for configuring trace generation.
pub trait Config {
    fn seed(&self) -> &str;
    fn n_traces(&self) -> usize;
    fn to_command(&self, tmpdir: &Path) -> Command;
}

pub(crate) fn generate_traces<C: Config>(config: &C) -> Result<Traces> {
    let tmpdir = TempDir::with_prefix("quint-connect-")?;
    let mut cmd = config.to_command(tmpdir.path());
    let output = cmd.output().context("Failed to execute Quint command")?;

    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).context("Failed to decode stderr.")?;
        return Err(anyhow!("{}", stderr)).context("Quint returned non-zero code.");
    }

    Traces::new(tmpdir)
}
