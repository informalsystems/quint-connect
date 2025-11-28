use crate::{
    driver::{Path, SpecAnnotations, nondet::NondetPicks},
    itf::{
        display::ValueDisplay,
        value::{Record, Value},
    },
};
use anyhow::{Context, Result, anyhow, bail};
use serde::Deserialize;
use std::fmt;

pub struct Step {
    #[doc(hidden)]
    pub action_taken: String,
    #[doc(hidden)]
    pub nondet_picks: NondetPicks,
    pub(crate) state: Value,
}

impl Step {
    pub(crate) fn new(state: Record, ann: &SpecAnnotations) -> Result<Self> {
        if ann.nondet_location.is_empty() {
            extract_from_mbt_vars(state, ann.state_location)
        } else {
            extract_from_sum_type(state, ann.nondet_location, ann.state_location)
        }
    }
}

fn extract_from_mbt_vars(mut state: Record, state_loc: Path) -> Result<Step> {
    Ok(Step {
        action_taken: extract_action_from_mbt_var(&mut state)?,
        nondet_picks: extract_nondet_from_mbt_var(&mut state)?,
        state: extract_vale_in_path(state, state_loc)?,
    })
}

fn extract_from_sum_type(mut state: Record, sum_type_loc: Path, state_loc: Path) -> Result<Step> {
    let sum_type = find_record_in_path(&state, sum_type_loc)?;
    let action_taken = extract_action_from_sum_type(sum_type)?;
    let nondet_picks = extract_nondet_from_sum_type(sum_type)?;

    // Remove unused mbt variables, if available.
    let _ = state.remove("mbt::actionTaken");
    let _ = state.remove("mbt::nondetPicks");

    let state = extract_vale_in_path(state, state_loc)?;

    Ok(Step {
        action_taken,
        nondet_picks,
        state,
    })
}

fn extract_action_from_mbt_var(state: &mut Record) -> Result<String> {
    state
        .remove("mbt::actionTaken")
        .ok_or(anyhow!("Missing `mbt::actionTaken` variable in the trace"))
        .and_then(|value| {
            String::deserialize(value).context("Failed to decode `mbt::actionTaken` variable")
        })
}

fn extract_nondet_from_mbt_var(state: &mut Record) -> Result<NondetPicks> {
    state
        .remove("mbt::nondetPicks")
        .ok_or(anyhow!("Missing `mbt::nondetPicks` variable in the trace"))
        .and_then(|value| {
            NondetPicks::from_value(value).context("Failed to extract nondet picks from trace")
        })
}

fn extract_vale_in_path(state: Record, path: &[&str]) -> Result<Value> {
    let mut value = Value::Record(state);
    for segment in path {
        let Value::Record(mut rec) = value else {
            bail!(
                "Can read {:?} from non-record value in path: {:?}",
                segment,
                path
            )
        };
        let Some(next) = rec.remove(segment) else {
            bail!("Can not find a value at {:?} in path: {:?}", segment, path)
        };
        value = next
    }
    Ok(value)
}

fn find_record_in_path<'a>(state: &'a Record, path: &[&str]) -> Result<&'a Record> {
    let mut rec = state;
    for segment in path {
        let Some(Value::Record(next)) = rec.get(segment) else {
            bail!("Can not find a Record at {:?} in path: {:?}", segment, path)
        };
        rec = next;
    }
    Ok(rec)
}

fn extract_action_from_sum_type(ty: &Record) -> Result<String> {
    let Some(Value::String(action)) = ty.get("tag") else {
        bail!(
            "Expected action to be the sum type's `tag` string.\n\
             Type found: {:#?}",
            ty
        )
    };
    Ok(action.clone())
}

fn extract_nondet_from_sum_type(ty: &Record) -> Result<NondetPicks> {
    match ty.get("value") {
        Some(Value::Tuple(t)) if t.is_empty() => Ok(NondetPicks::empty()),
        Some(Value::Record(rec)) => Ok(NondetPicks::from_record(rec.clone())),
        _ => bail!(
            "Expected nondet picks to be the sum type's `value` as a single Record.\n\
             Type found: {:#?}",
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
        match &self.state {
            Value::Record(rec) => {
                if rec.is_empty() {
                    write!(f, " <none>")?;
                } else {
                    for (key, value) in rec.iter() {
                        write!(f, "\n+ {}: {}", key, value.display())?;
                    }
                }
            }
            Value::Map(map) => {
                if map.is_empty() {
                    write!(f, " <none>")?;
                } else {
                    for (key, value) in map.iter() {
                        write!(f, "\n+ {}: {}", key.display(), value.display())?;
                    }
                }
            }
            other => write!(f, " {}", other.display())?,
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

        let step = extract_from_mbt_vars(state, &[]).unwrap();
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

        let step = extract_from_mbt_vars(state, &[]).unwrap();
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
        extract_from_mbt_vars(state, &[]).unwrap();
    }

    #[test]
    #[should_panic(expected = "Failed to decode `mbt::actionTaken` variable")]
    fn test_invalid_action_taken() {
        let mut state = Record::new();
        state.insert("mbt::actionTaken".to_string(), Value::Number(42));
        extract_from_mbt_vars(state, &[]).unwrap();
    }

    #[test]
    #[should_panic(expected = "Missing `mbt::nondetPicks` variable in the trace")]
    fn test_no_nondet_picks() {
        let mut state = Record::new();
        state.insert(
            "mbt::actionTaken".to_string(),
            Value::String("init".to_string()),
        );
        extract_from_mbt_vars(state, &[]).unwrap();
    }
}
