mod run;
mod test;
mod utils;

pub use run::RunConfig;
pub use test::TestConfig;

use crate::trace::iter::Traces;
use anyhow::{bail, Context, Result};
use std::{path::Path, process::Command};
use tempdir::TempDir;

const DEFAULT_TRACES: usize = 100;

pub trait QuintCommand {
    fn to_command(&self, tmpdir: &Path) -> Command;
}

pub fn generate<Cmd: QuintCommand>(cmd: &Cmd) -> Result<Traces> {
    let tmpdir = TempDir::new("quint-connect")?;
    let mut cmd = cmd.to_command(tmpdir.path());
    let output = cmd.output().context("Failed to execute Quint")?;

    if !output.status.success() {
        bail!("Quint returned non-zero code. Please check your spec.")
    }

    Traces::new(tmpdir)
}
