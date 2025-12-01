use crate::trace::generator::{Config, DEFAULT_TRACES, utils::opt_arg};
use std::{path::Path, process::Command};

/// Configuration for generating traces using `quint run` in simulation mode.
pub struct RunConfig {
    pub spec: String,
    pub main: Option<String>,
    pub init: Option<String>,
    pub step: Option<String>,
    pub max_samples: Option<usize>,
    pub max_steps: Option<usize>,
    pub seed: String,
}

impl Config for RunConfig {
    fn seed(&self) -> &str {
        self.seed.as_str()
    }

    fn n_traces(&self) -> usize {
        self.max_samples.unwrap_or(DEFAULT_TRACES)
    }

    fn to_command(&self, tmpdir: &Path) -> Command {
        let n_traces = self.n_traces().to_string();
        let mut cmd = Command::new("quint");
        cmd.arg("run")
            .arg(Path::new(&self.spec))
            .arg("--seed")
            .arg(&self.seed)
            .arg("--max-samples")
            .arg(&n_traces)
            .arg("--n-traces")
            .arg(n_traces)
            .arg("--out-itf")
            .arg(tmpdir.join("run_{seq}.itf.json"))
            .arg("--mbt")
            .arg("--verbosity")
            .arg("0");

        opt_arg(&mut cmd, "--main", self.main.as_ref());
        opt_arg(&mut cmd, "--init", self.init.as_ref());
        opt_arg(&mut cmd, "--step", self.step.as_ref());
        opt_arg(
            &mut cmd,
            "--max-steps",
            self.max_steps.map(|n| n.to_string()),
        );
        cmd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_config() -> RunConfig {
        RunConfig {
            spec: "foo.qnt".to_string(),
            seed: "42".to_string(),
            main: None,
            init: None,
            step: None,
            max_samples: None,
            max_steps: None,
        }
    }

    #[test]
    fn test_basic_config() {
        let config = basic_config();

        assert_eq!(
            to_string(config),
            "quint run foo.qnt \
             --seed 42 \
             --max-samples 100 \
             --n-traces 100 \
             --out-itf tmpdir/run_{seq}.itf.json \
             --mbt \
             --verbosity 0"
        );
    }

    #[test]
    fn test_main_module() {
        let mut config = basic_config();
        config.main = Some("simulation".to_string());

        assert_eq!(
            to_string(config),
            "quint run foo.qnt \
             --seed 42 \
             --max-samples 100 \
             --n-traces 100 \
             --out-itf tmpdir/run_{seq}.itf.json \
             --mbt \
             --verbosity 0 \
             --main simulation"
        );
    }

    #[test]
    fn test_init_action() {
        let mut config = basic_config();
        config.init = Some("my_init".to_string());

        assert_eq!(
            to_string(config),
            "quint run foo.qnt \
             --seed 42 \
             --max-samples 100 \
             --n-traces 100 \
             --out-itf tmpdir/run_{seq}.itf.json \
             --mbt \
             --verbosity 0 \
             --init my_init"
        );
    }

    #[test]
    fn test_step_action() {
        let mut config = basic_config();
        config.step = Some("my_step".to_string());

        assert_eq!(
            to_string(config),
            "quint run foo.qnt \
             --seed 42 \
             --max-samples 100 \
             --n-traces 100 \
             --out-itf tmpdir/run_{seq}.itf.json \
             --mbt \
             --verbosity 0 \
             --step my_step"
        );
    }

    #[test]
    fn test_max_samples() {
        let mut config = basic_config();
        config.max_samples = Some(42);

        assert_eq!(
            to_string(config),
            "quint run foo.qnt \
             --seed 42 \
             --max-samples 42 \
             --n-traces 42 \
             --out-itf tmpdir/run_{seq}.itf.json \
             --mbt \
             --verbosity 0"
        );
    }

    #[test]
    fn test_max_steps() {
        let mut config = basic_config();
        config.max_steps = Some(32);

        assert_eq!(
            to_string(config),
            "quint run foo.qnt \
             --seed 42 \
             --max-samples 100 \
             --n-traces 100 \
             --out-itf tmpdir/run_{seq}.itf.json \
             --mbt \
             --verbosity 0 \
             --max-steps 32"
        );
    }

    fn to_string(config: RunConfig) -> String {
        let dir = Path::new("tmpdir");
        let cmd = config.to_command(dir);
        format!("{:?}", cmd).replace("\"", "")
    }
}
