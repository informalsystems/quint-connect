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
    pub(crate) fn from_mbt_state(mut state: Record) -> Result<Self> {
        Ok(Self {
            action_taken: extract_default_action_taken(&mut state)?,
            nondet_picks: extract_default_nondet_picks(&mut state)?,
            state,
        })
    }

    pub fn from_sum_type(mut state: Record, path: &[&str]) -> Result<Self> {
        let ty = find_sum_type(&state, path)?;
        let action_taken = extract_action_taken_from_sum_type(ty)?;
        let nondet_picks = extract_nondet_picks_from_sum_type(ty)?;

        // Remove unused mbt variables, if available.
        let _ = state.remove("mbt::actionTaken");
        let _ = state.remove("mbt::nondetPicks");

        Ok(Self {
            action_taken,
            nondet_picks,
            state,
        })
    }
}

fn extract_default_action_taken(state: &mut Record) -> Result<String> {
    state
        .remove("mbt::actionTaken")
        .ok_or(anyhow!("Missing `mbt::actionTaken` variable in the trace"))
        .and_then(|value| {
            String::deserialize(value).context("Failed to decode `mbt::actionTaken` variable")
        })
}

fn extract_default_nondet_picks(state: &mut Record) -> Result<NondetPicks> {
    state
        .remove("mbt::nondetPicks")
        .ok_or(anyhow!("Missing `mbt::nondetPicks` variable in the trace"))
        .and_then(|value| {
            NondetPicks::from_value(value).context("Failed to extract nondet picks from trace")
        })
}

fn find_sum_type<'a>(state: &'a Record, path: &[&str]) -> Result<&'a Record> {
    let mut rec = state;
    for segment in path {
        let Some(Value::Record(next)) = rec.get(segment) else {
            bail!("Can not find Quint sum type at {:?}", segment)
        };
        rec = next;
    }
    Ok(rec)
}

fn extract_action_taken_from_sum_type(ty: &Record) -> Result<String> {
    let Some(Value::String(action)) = ty.get("tag") else {
        bail!(
            "Expected action taken to be the sum type's `tag` string.\n\
             Type: {:#?}",
            ty
        )
    };
    Ok(action.clone())
}

fn extract_nondet_picks_from_sum_type(ty: &Record) -> Result<NondetPicks> {
    match ty.get("value") {
        Some(Value::Tuple(t)) if t.is_empty() => Ok(NondetPicks::empty()),
        Some(Value::Record(rec)) => Ok(NondetPicks::from_record(rec.clone())),
        _ => bail!(
            "Expected nondet picks to be the sum type's `value` as a single Record.\n\
             Type: {:#?}",
            ty
        ),
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action taken:")?;
        if self.action_taken.is_empty() {
            writeln!(f, " <anonymous>")?;
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
    use crate::itf::Value;

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

        let step = Step::from_mbt_state(state).unwrap();
        assert_eq!(step.action_taken, "init");
        assert_eq!(step.nondet_picks.get("n").unwrap(), &Value::Number(42));

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

        let step = Step::from_mbt_state(state).unwrap();
        assert_eq!(step.action_taken, "");
        assert!(step.nondet_picks.is_empty());

        assert_eq!(
            format!("{}", step),
            "Action taken: <anonymous>\n\
             Nondet picks: <none>\n\
             Next state: <none>"
        );
    }

    #[test]
    #[should_panic(expected = "Missing `mbt::actionTaken` variable in the trace")]
    fn test_no_action_taken() {
        let state = Record::new();
        Step::from_mbt_state(state).unwrap();
    }

    #[test]
    #[should_panic(expected = "Failed to decode `mbt::actionTaken` variable")]
    fn test_invalid_action_taken() {
        let mut state = Record::new();
        state.insert("mbt::actionTaken".to_string(), Value::Number(42));
        Step::from_mbt_state(state).unwrap();
    }

    #[test]
    #[should_panic(expected = "Missing `mbt::nondetPicks` variable in the trace")]
    fn test_no_nondet_picks() {
        let mut state = Record::new();
        state.insert(
            "mbt::actionTaken".to_string(),
            Value::String("init".to_string()),
        );
        Step::from_mbt_state(state).unwrap();
    }
}
