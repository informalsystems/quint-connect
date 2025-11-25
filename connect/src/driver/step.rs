use crate::{
    driver::nondet::NondetPicks,
    itf::value::{Record, Value},
};
use anyhow::{Context, Result, anyhow, bail};
use serde::Deserialize;

pub struct Step {
    pub action_taken: String,
    pub nondet_picks: NondetPicks,
    state: Record,
}

impl Step {
    pub(crate) fn new(state: Value) -> Result<Self> {
        let Value::Record(mut state) = state else {
            bail!("Expected state to be a `Value::Record`")
        };

        let action_taken = state
            .remove("mbt::actionTaken")
            .ok_or(anyhow!("Missing `mbt::actionTaken` variable in the trace"))
            .and_then(|value| {
                String::deserialize(value).context("Failed to decode `mbt::actionTaken` variable")
            })?;

        let nondet_picks = state
            .remove("mbt::nondetPicks")
            .ok_or(anyhow!("Missing `mbt::nondetPicks` variable in the trace"))
            .and_then(|value| {
                NondetPicks::new(value).context("Failed to extract nondet picks from trace")
            })?;

        Ok(Self {
            action_taken,
            nondet_picks,
            state,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_step() {
        let action = Value::String("init".to_string());

        let mut nondet = Record::new();
        nondet.insert("tag".to_string(), Value::String("Some".to_string()));
        nondet.insert("value".to_string(), Value::Number(42));

        let mut nondets = Record::new();
        nondets.insert("n".to_string(), Value::Record(nondet));

        let mut state = Record::new();
        state.insert("mbt::actionTaken".to_string(), action);
        state.insert("mbt::nondetPicks".to_string(), Value::Record(nondets));

        let step = Step::new(Value::Record(state)).unwrap();
        assert_eq!(step.action_taken, "init");
        assert_eq!(
            step.nondet_picks
                .get("n")
                .unwrap()
                .try_into::<i64>()
                .unwrap(),
            42
        );
    }

    #[test]
    #[should_panic(expected = "Expected state to be a `Value::Record`")]
    fn test_invalid_state_value() {
        let state = Value::Number(42);
        Step::new(state).unwrap();
    }

    #[test]
    #[should_panic(expected = "Missing `mbt::actionTaken` variable in the trace")]
    fn test_no_action_taken() {
        let state = Record::new();
        let state = Value::Record(state);
        Step::new(state).unwrap();
    }

    #[test]
    #[should_panic(expected = "Failed to decode `mbt::actionTaken` variable")]
    fn test_invalid_action_taken() {
        let mut state = Record::new();
        state.insert("mbt::actionTaken".to_string(), Value::Number(42));
        let state = Value::Record(state);
        Step::new(state).unwrap();
    }

    #[test]
    #[should_panic(expected = "Missing `mbt::nondetPicks` variable in the trace")]
    fn test_no_nondet_picks() {
        let mut state = Record::new();
        state.insert(
            "mbt::actionTaken".to_string(),
            Value::String("init".to_string()),
        );
        let state = Value::Record(state);
        Step::new(state).unwrap();
    }
}
