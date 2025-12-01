mod seed;

pub use crate::trace::generator::{RunConfig, TestConfig};
pub use seed::gen_random_seed;

use crate::{
    Driver, State, Step,
    logger::*,
    trace::{
        generator::{Config as GenConfig, generate_traces},
        iter::Traces,
    },
    value::ValueDisplay,
};
use anyhow::{Result, bail, ensure};
use itf::Value;
use similar::TextDiff;

/// Configuration for running model-based tests, combining test metadata with
/// trace generation settings.
pub struct Config<C: GenConfig> {
    pub test_name: String,
    pub gen_config: C,
}

/// Run the test configuration using the given test driver.
pub fn run_test<C: GenConfig>(driver: impl Driver, config: Config<C>) -> Result<()> {
    title!("Running model based tests for {}", config.test_name);
    info!(
        "Generating {} traces using `{}` as random seed ...",
        config.gen_config.n_traces(),
        config.gen_config.seed()
    );

    let traces = generate_traces(&config.gen_config)?;
    let result = replay_traces(driver, traces);

    if result.is_ok() {
        success!("[OK] {}", config.test_name);
    } else {
        error!("[FAIL] {} ", config.test_name);
        error!(
            "Reproduce this error with `QUINT_SEED={}`\n",
            config.gen_config.seed()
        );
    }

    result
}

fn replay_traces<D: Driver>(mut driver: D, traces: Traces) -> Result<()> {
    info!("Replaying traces ...");

    let ann = D::config();
    let mut iter = traces.peekable();
    ensure!(
        iter.peek().is_some(),
        "Trace generation produced zero traces.\n\
         Please check your specification and/or your test configuration."
    );

    for (trace, t) in iter.zip(1..) {
        trace!(1, "[Trace {}]", t);

        for (state, s) in trace?.states.into_iter().zip(0..) {
            trace!(2, "\nDeriving step from:\n{}\n", state.value.display());
            let Value::Record(state) = state.value else {
                bail!("Expected current state to be a Record")
            };

            let step = Step::new(state, &ann)?;
            trace!(1, "[Step {}]\n{}\n", s, step);
            ensure!(
                !step.action_taken.is_empty(),
                "An anonymous action was found!\n\
                 Please make sure all actions in the specification are properly named.\n\
                 Check the crate docs for tips and tricks on action name heuristics."
            );

            driver.step(&step)?;
            check_state(&driver, step)?;
        }
    }

    Ok(())
}

fn check_state<D: Driver>(driver: &D, step: Step) -> Result<()> {
    trace!(2, "Extracting state from:\n{}\n", step.state.display());
    let spec_state = D::State::from_spec(step.state)?;
    let driver_state = D::State::from_driver(driver)?;

    if spec_state != driver_state {
        let left = format!("{:#?}", spec_state);
        let right = format!("{:#?}", driver_state);
        let diff = TextDiff::from_lines(&left, &right);

        error!("Specification and implementation states diverge");
        trace!(
            1,
            "{}",
            diff.unified_diff()
                .context_radius(256) // large enought?
                .header("specification", "implementation")
                .missing_newline_hint(false)
        );

        bail!("State invariant failed")
    }

    Ok(())
}
