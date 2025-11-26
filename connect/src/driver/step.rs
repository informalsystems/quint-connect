use crate::{
    driver::nondet::NondetPicks,
    itf::{
        display::ValueDisplay,
        value::{Record, Value},
    },
};
use anyhow::{Context, Result, anyhow, bail};
use serde::Deserialize;
use std::fmt;

pub struct Step {
    pub action_taken: String,
    pub nondet_picks: NondetPicks,
    pub(crate) state: Record,
}

impl Step {
    pub(crate) fn new(state: Value) -> Result<Self> {
        let Value::Record(mut state) = state else {
            bail!("Expected state to be a `Value::Record`")
        };

        Ok(Self {
            action_taken: extract_action_taken(&mut state)?,
            nondet_picks: extract_nondet_picks(&mut state)?,
            state,
        })
    }
}

fn extract_nondet_picks(state: &mut Record) -> Result<NondetPicks> {
    state
        .remove("mbt::nondetPicks")
        .ok_or(anyhow!("Missing `mbt::nondetPicks` variable in the trace"))
        .and_then(|value| {
            NondetPicks::new(value).context("Failed to extract nondet picks from trace")
        })
}

fn extract_action_taken(state: &mut Record) -> Result<String> {
    state
        .remove("mbt::actionTaken")
        .ok_or(anyhow!("Missing `mbt::actionTaken` variable in the trace"))
        .and_then(|value| {
            String::deserialize(value).context("Failed to decode `mbt::actionTaken` variable")
        })
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action taken:")?;
        if self.action_taken.is_empty() {
            writeln!(f, " <stuttered>")?;
        } else {
            writeln!(f, " {}", self.action_taken)?;
        }

        write!(f, "Nondet picks:")?;
        if self.nondet_picks.is_empty() {
            writeln!(f, " <none>")?;
        } else {
            writeln!(f, "\n{}", self.nondet_picks)?;
        }

        write!(f, "Next state:")?;
        if self.state.is_empty() {
            write!(f, " <none>")?;
        } else {
            for (key, value) in self.state.iter() {
                write!(f, "\n+ {}: {}", key, value.display())?;
            }
        }
        Ok(())
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

        let mut procs = Record::new();
        procs.insert("p1".to_string(), Value::Number(42));
        procs.insert("p2".to_string(), Value::Number(44));

        let mut state = Record::new();
        state.insert("mbt::actionTaken".to_string(), action);
        state.insert("mbt::nondetPicks".to_string(), Value::Record(nondets));
        state.insert("procs".to_string(), Value::Record(procs));

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

        assert_eq!(
            format!("{}", step),
            "Action taken: init\n\
             Nondet picks:\n\
             + n: 42\n\
             Next state:\n\
             + procs: { p1: 42, p2: 44 }"
        );
    }

    #[test]
    fn test_empty_step() {
        let action = Value::String("".to_string());

        let mut state = Record::new();
        state.insert("mbt::actionTaken".to_string(), action);
        state.insert("mbt::nondetPicks".to_string(), Value::Record(Record::new()));

        let step = Step::new(Value::Record(state)).unwrap();
        assert_eq!(step.action_taken, "");
        assert!(step.nondet_picks.is_empty());

        assert_eq!(
            format!("{}", step),
            "Action taken: <stuttered>\n\
             Nondet picks: <none>\n\
             Next state: <none>"
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
