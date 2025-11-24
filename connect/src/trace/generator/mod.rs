mod run;
mod test;
mod utils;

pub use run::RunConfig;
pub use test::TestConfig;

use crate::trace::iter::Traces;
use anyhow::{Context, Result, bail};
use std::{path::Path, process::Command};
use tempdir::TempDir;

const DEFAULT_TRACES: usize = 100;

pub trait Config {
    fn to_command(&self, tmpdir: &Path) -> Command;
}

pub(crate) fn generate_traces<C: Config>(config: &C) -> Result<Traces> {
    let tmpdir = TempDir::new("quint-connect")?;
    let mut cmd = config.to_command(tmpdir.path());
    let output = cmd.output().context("Failed to execute Quint")?;

    if !output.status.success() {
        // TODO: log std error to help with debugging.
        bail!("Quint returned non-zero code. Please check your spec.")
    }

    Traces::new(tmpdir)
}
