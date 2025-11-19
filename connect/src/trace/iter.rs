use crate::trace::Trace;
use anyhow::{Context, Result};
use std::{
    fs::{File, ReadDir},
    io::BufReader,
    path::Path,
};
use tempdir::TempDir;

pub(crate) struct Traces {
    iter: ReadDir,
    _tmpdir: TempDir,
}

impl Traces {
    pub fn new(tmpdir: TempDir) -> Result<Self> {
        let iter = std::fs::read_dir(tmpdir.path()).with_context(|| {
            format!("Failed to list trace files at: {}", tmpdir.path().display())
        })?;

        Ok(Self {
            iter,
            _tmpdir: tmpdir,
        })
    }
}

impl Iterator for Traces {
    type Item = Result<Trace>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(entry)) => Some(trace_from_file(&entry.path())),
            Some(Err(err)) => Some(Err(err.into())),
            None => None,
        }
    }
}

fn trace_from_file(path: &Path) -> Result<Trace> {
    let file = File::open(path)
        .with_context(|| format!("Can't open trace file at: {}", path.display()))?;

    let trace = serde_json::from_reader(BufReader::new(file))
        .with_context(|| format!("Failed to parse JSON trace file at: {}", path.display()))?;

    Ok(trace)
}
