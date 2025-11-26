use crate::{
    Driver, Step,
    logger::*,
    trace::{
        generator::{Config as GenConfig, generate_traces},
        iter::Traces,
    },
};
use anyhow::Result;

pub use crate::trace::generator::{RunConfig, TestConfig};

pub fn random_seed() -> String {
    "0x42".to_string() // FIXME
}

pub struct Config<C: GenConfig> {
    pub test_name: String,
    pub gen_config: C,
}

pub fn run_test<C: GenConfig>(driver: impl Driver, config: Config<C>) -> Result<()> {
    title!("Running model based tests for {}", config.test_name);
    info!(
        "Generating {} traces using `{}` as random seed ...",
        config.gen_config.n_traces(),
        config.gen_config.seed()
    );

    let traces = generate_traces(&config.gen_config)?;
    replay_traces(driver, traces)?;

    success!("[OK] {}", config.test_name);
    Ok(())
}

fn replay_traces<D: Driver>(mut driver: D, traces: Traces) -> Result<()> {
    info!("Replaying generated traces ...");

    for (trace, t) in traces.zip(1..) {
        trace!("[Trace {}]", t);
        for (state, s) in trace?.states.into_iter().zip(1..) {
            let step = Step::new(state.value)?;
            trace!("[Step {}]\n{}\n", s, step);
            if step.action_taken.is_empty() {
                continue; // stuttered?
            }
            driver.step(&step)?;
        }
    }

    Ok(())
}
