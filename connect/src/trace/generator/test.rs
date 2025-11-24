use crate::trace::generator::{Config, DEFAULT_TRACES, utils::opt_arg};
use std::{path::Path, process::Command};

pub struct TestConfig {
    pub spec: String,
    pub main: Option<String>,
    pub test: String,
    pub max_samples: Option<usize>,
    pub seed: String,
}

impl Config for TestConfig {
    fn to_command(&self, tmpdir: &Path) -> Command {
        let n_traces = self.max_samples.unwrap_or(DEFAULT_TRACES).to_string();
        let mut cmd = Command::new("quint");
        cmd.arg("test")
            .arg(Path::new(&self.spec))
            .arg("--seed")
            .arg(&self.seed)
            .arg("--match")
            .arg(format!("^{}$", self.test))
            .arg("--max-samples")
            .arg(n_traces)
            .arg("--out-itf")
            .arg(tmpdir.join("test_{seq}.itf.json"))
            .arg("--mbt")
            .arg("--verbosity")
            .arg("0");

        opt_arg(&mut cmd, "--main", self.main.as_ref());
        cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_config() -> TestConfig {
        TestConfig {
            spec: "foo.qnt".to_string(),
            test: "happyTest".to_string(),
            seed: "42".to_string(),
            main: None,
            max_samples: None,
        }
    }

    #[test]
    fn test_basic_config() {
        let config = basic_config();

        assert_eq!(
            to_string(config),
            "quint test foo.qnt \
             --seed 42 \
             --match ^happyTest$ \
             --max-samples 100 \
             --out-itf tmpdir/test_{seq}.itf.json \
             --mbt \
             --verbosity 0"
        );
    }

    #[test]
    fn test_main_module() {
        let mut config = basic_config();
        config.main = Some("tests".to_string());

        assert_eq!(
            to_string(config),
            "quint test foo.qnt \
             --seed 42 \
             --match ^happyTest$ \
             --max-samples 100 \
             --out-itf tmpdir/test_{seq}.itf.json \
             --mbt \
             --verbosity 0 \
             --main tests"
        );
    }

    #[test]
    fn test_max_samples() {
        let mut config = basic_config();
        config.max_samples = Some(42);

        assert_eq!(
            to_string(config),
            "quint test foo.qnt \
             --seed 42 \
             --match ^happyTest$ \
             --max-samples 42 \
             --out-itf tmpdir/test_{seq}.itf.json \
             --mbt \
             --verbosity 0"
        );
    }

    fn to_string(config: TestConfig) -> String {
        let dir = Path::new("tmpdir");
        let cmd = config.to_command(dir);
        format!("{:?}", cmd).replace("\"", "")
    }
}
