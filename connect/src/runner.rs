use crate::{trace::generator::Config as GenConfig, Driver};
use anyhow::Result;

pub use crate::trace::generator::{RunConfig, TestConfig};

pub fn random_seed() -> String {
    "42".to_string() // FIXME
}

pub struct Config<C: GenConfig> {
    pub test_name: String,
    pub gen_config: C,
}

pub fn run_test<C: GenConfig>(_driver: impl Driver, _config: Config<C>) -> Result<()> {
    // TODO: implement this
    //let _traces = generate_traces(&config.gen_config)?;
    todo!()
}
