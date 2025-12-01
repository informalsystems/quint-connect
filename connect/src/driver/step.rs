use crate::{
    driver::{Config, Path, nondet::NondetPicks},
    value::ValueDisplay,
};
use anyhow::{Context, Result, anyhow, bail};
use itf::value::{Record, Value};
use serde::Deserialize;
use std::fmt;

pub struct Step {
    #[doc(hidden)] // public for macro use
    pub action_taken: String,
    #[doc(hidden)] // public for macro use
    pub nondet_picks: NondetPicks,
    pub(crate) state: Value,
}

impl Step {
    pub(crate) fn new(state: Record, config: &Config) -> Result<Self> {
        if config.nondet.is_empty() {
            extract_from_mbt_vars(state, config.state)
        } else {
            extract_from_sum_type(state, config.nondet, config.state)
        }
    }
}

fn extract_from_mbt_vars(mut state: Record, state_path: Path) -> Result<Step> {
    Ok(Step {
        action_taken: extract_action_from_mbt_var(&mut state)?,
        nondet_picks: extract_nondet_from_mbt_var(&mut state)?,
        state: extract_vale_in_path(state, state_path)?,
    })
}

fn extract_from_sum_type(mut state: Record, sum_type_path: Path, state_path: Path) -> Result<Step> {
    let sum_type = find_record_in_path(&state, sum_type_path)?;
    let action_taken = extract_action_from_sum_type(sum_type)?;
    let nondet_picks = extract_nondet_from_sum_type(sum_type)?;

    // Remove unused mbt variables, if available.
    let _ = state.remove("mbt::actionTaken");
    let _ = state.remove("mbt::nondetPicks");

    let state = extract_vale_in_path(state, state_path)?;

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
            NondetPicks::new(value).context("Failed to extract nondet picks from trace")
        })
}

fn extract_vale_in_path(state: Record, path: &[&str]) -> Result<Value> {
    let mut value = Value::Record(state);
    for segment in path {
        let Value::Record(mut rec) = value else {
            bail!(
                "Can not read {:?} from non-record value in path: {:?}\n\
                 Current value: {}",
                segment,
                path,
                value.display(),
            )
        };
        let Some(next) = rec.remove(segment) else {
            bail!(
                "Can not find a value at {:?} in path: {:?}\n\
                 Current value: {}",
                segment,
                path,
                Value::Record(rec).display()
            )
        };
        value = next
    }
    Ok(value)
}

fn find_record_in_path<'a>(state: &'a Record, path: &[&str]) -> Result<&'a Record> {
    let mut rec = state;
    for segment in path {
        let Some(Value::Record(next)) = rec.get(segment) else {
            bail!(
                "Can not find a Record at {:?} in path: {:?}\n\
                 Current state: {}",
                segment,
                path,
                state.display()
            )
        };
        rec = next;
    }
    Ok(rec)
}

fn extract_action_from_sum_type(ty: &Record) -> Result<String> {
    let Some(Value::String(action)) = ty.get("tag") else {
        bail!(
            "Expected action to be a sum type variant.\n\
             Value found: {}",
            ty.display()
        )
    };
    Ok(action.clone())
}

fn extract_nondet_from_sum_type(ty: &Record) -> Result<NondetPicks> {
    match ty.get("value") {
        Some(Value::Tuple(t)) if t.is_empty() => Ok(NondetPicks::empty()),
        Some(Value::Record(rec)) => Ok(rec.clone().into()),
        _ => bail!(
            "Expected nondet picks to be a sum type variant value as a record.\n\
             Value found: {}",
            ty.display()
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
    use itf::Value;

    #[test]
    fn test_extract_vale_in_path_empty_path() {
        let mut rec = Record::new();
        rec.insert("key".to_string(), Value::String("value".to_string()));

        let result = extract_vale_in_path(rec.clone(), &[]).unwrap();
        assert_eq!(result, Value::Record(rec));
    }

    #[test]
    fn test_extract_vale_in_path_single_level() {
        let mut rec = Record::new();
        rec.insert("key".to_string(), Value::String("value".to_string()));

        let result = extract_vale_in_path(rec, &["key"]).unwrap();
        assert_eq!(result, Value::String("value".to_string()));
    }

    #[test]
    fn test_extract_vale_in_path_nested() {
        let mut inner = Record::new();
        inner.insert("inner_key".to_string(), Value::Number(42));

        let mut outer = Record::new();
        outer.insert("outer_key".to_string(), Value::Record(inner));

        let result = extract_vale_in_path(outer, &["outer_key", "inner_key"]).unwrap();
        assert_eq!(result, Value::Number(42));
    }

    #[test]
    #[should_panic(expected = "Can not find a value at")]
    fn test_extract_vale_in_path_missing_key() {
        let mut rec = Record::new();
        rec.insert("key".to_string(), Value::String("value".to_string()));

        extract_vale_in_path(rec, &["missing"]).unwrap();
    }

    #[test]
    #[should_panic(expected = "non-record value in path")]
    fn test_extract_vale_in_path_non_record() {
        let mut rec = Record::new();
        rec.insert("key".to_string(), Value::String("value".to_string()));

        extract_vale_in_path(rec, &["key", "nested"]).unwrap();
    }

    #[test]
    fn test_find_record_in_path_empty_path() {
        let rec = Record::new();
        let result = find_record_in_path(&rec, &[]).unwrap();
        assert_eq!(result, &rec);
    }

    #[test]
    fn test_find_record_in_path_single_level() {
        let inner = Record::new();
        let mut outer = Record::new();
        outer.insert("inner".to_string(), Value::Record(inner.clone()));

        let result = find_record_in_path(&outer, &["inner"]).unwrap();
        assert_eq!(result, &inner);
    }

    #[test]
    fn test_find_record_in_path_nested() {
        let innermost = Record::new();
        let mut middle = Record::new();
        middle.insert("innermost".to_string(), Value::Record(innermost.clone()));

        let mut outer = Record::new();
        outer.insert("middle".to_string(), Value::Record(middle));

        let result = find_record_in_path(&outer, &["middle", "innermost"]).unwrap();
        assert_eq!(result, &innermost);
    }

    #[test]
    #[should_panic(expected = "Can not find a Record")]
    fn test_find_record_in_path_missing_key() {
        let rec = Record::new();
        find_record_in_path(&rec, &["missing"]).unwrap();
    }

    #[test]
    #[should_panic(expected = "Can not find a Record")]
    fn test_find_record_in_path_non_record() {
        let mut rec = Record::new();
        rec.insert("key".to_string(), Value::String("value".to_string()));

        find_record_in_path(&rec, &["key"]).unwrap();
    }

    #[test]
    fn test_extract_action_from_mbt_var_success() {
        let mut rec = Record::new();
        rec.insert(
            "mbt::actionTaken".to_string(),
            Value::String("TestAction".to_string()),
        );

        let result = extract_action_from_mbt_var(&mut rec).unwrap();
        assert_eq!(result, "TestAction");
        assert!(!rec.contains_key("mbt::actionTaken"));
    }

    #[test]
    #[should_panic(expected = "Missing `mbt::actionTaken`")]
    fn test_extract_action_from_mbt_var_missing() {
        let mut rec = Record::new();
        extract_action_from_mbt_var(&mut rec).unwrap();
    }

    #[test]
    #[should_panic(expected = "Failed to decode `mbt::actionTaken`")]
    fn test_extract_action_from_mbt_var_wrong_type() {
        let mut rec = Record::new();
        rec.insert("mbt::actionTaken".to_string(), Value::Number(42));

        extract_action_from_mbt_var(&mut rec).unwrap();
    }

    #[test]
    fn test_extract_nondet_from_mbt_var_success() {
        let mut rec = Record::new();
        let nondet_rec = Record::new();
        rec.insert("mbt::nondetPicks".to_string(), Value::Record(nondet_rec));

        let result = extract_nondet_from_mbt_var(&mut rec);
        assert!(result.is_ok());
        assert!(!rec.contains_key("mbt::nondetPicks"));
    }

    #[test]
    #[should_panic(expected = "Missing `mbt::nondetPicks`")]
    fn test_extract_nondet_from_mbt_var_missing() {
        let mut rec = Record::new();
        extract_nondet_from_mbt_var(&mut rec).unwrap();
    }

    #[test]
    fn test_extract_action_from_sum_type_success() {
        let mut rec = Record::new();
        rec.insert("tag".to_string(), Value::String("ActionName".to_string()));

        let result = extract_action_from_sum_type(&rec).unwrap();
        assert_eq!(result, "ActionName");
    }

    #[test]
    #[should_panic(expected = "Expected action to be a sum type variant.")]
    fn test_extract_action_from_sum_type_missing_tag() {
        let rec = Record::new();
        extract_action_from_sum_type(&rec).unwrap();
    }

    #[test]
    #[should_panic(expected = "Expected action to be a sum type variant.")]
    fn test_extract_action_from_sum_type_wrong_type() {
        let mut rec = Record::new();
        rec.insert("tag".to_string(), Value::Number(42));

        extract_action_from_sum_type(&rec).unwrap();
    }

    #[test]
    fn test_extract_nondet_from_sum_type_empty_tuple() {
        let mut rec = Record::new();
        rec.insert("value".to_string(), Value::Tuple(vec![].into()));

        let result = extract_nondet_from_sum_type(&rec).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_nondet_from_sum_type_record() {
        let nondet_rec = Record::new();
        let mut rec = Record::new();
        rec.insert("value".to_string(), Value::Record(nondet_rec));

        let result = extract_nondet_from_sum_type(&rec);
        assert!(result.is_ok());
    }

    #[test]
    #[should_panic(expected = "Expected nondet picks to be a sum type variant value as a record.")]
    fn test_extract_nondet_from_sum_type_invalid() {
        let mut rec = Record::new();
        rec.insert("value".to_string(), Value::String("invalid".to_string()));

        extract_nondet_from_sum_type(&rec).unwrap();
    }

    #[test]
    fn test_extract_from_mbt_vars_success() {
        let mut rec = Record::new();
        rec.insert(
            "mbt::actionTaken".to_string(),
            Value::String("TestAction".to_string()),
        );

        let mut nondet_rec = Record::new();
        nondet_rec.insert("pick1".to_string(), Value::Number(1));
        rec.insert("mbt::nondetPicks".to_string(), Value::Record(nondet_rec));

        rec.insert(
            "state_var".to_string(),
            Value::String("state_value".to_string()),
        );

        let result = extract_from_mbt_vars(rec, &["state_var"]).unwrap();
        assert_eq!(result.action_taken, "TestAction");
        assert_eq!(result.state, Value::String("state_value".to_string()));
    }

    #[test]
    #[should_panic(expected = "Missing `mbt::actionTaken`")]
    fn test_extract_from_mbt_vars_missing_action() {
        let mut rec = Record::new();
        let nondet_rec = Record::new();
        rec.insert("mbt::nondetPicks".to_string(), Value::Record(nondet_rec));

        extract_from_mbt_vars(rec, &[]).unwrap();
    }

    #[test]
    fn test_extract_from_sum_type_success() {
        let mut sum_rec = Record::new();
        sum_rec.insert("tag".to_string(), Value::String("TestAction".to_string()));
        sum_rec.insert("value".to_string(), Value::Tuple(vec![].into()));

        let mut rec = Record::new();
        rec.insert("sum_type".to_string(), Value::Record(sum_rec));
        rec.insert(
            "state_var".to_string(),
            Value::String("state_value".to_string()),
        );

        let result = extract_from_sum_type(rec, &["sum_type"], &["state_var"]).unwrap();
        assert_eq!(result.action_taken, "TestAction");
        assert_eq!(result.state, Value::String("state_value".to_string()));
        assert!(result.nondet_picks.is_empty());
    }

    #[test]
    fn test_extract_from_sum_type_removes_mbt_vars() {
        let mut sum_rec = Record::new();
        sum_rec.insert("tag".to_string(), Value::String("TestAction".to_string()));
        sum_rec.insert("value".to_string(), Value::Tuple(vec![].into()));

        let mut rec = Record::new();
        rec.insert("sum_type".to_string(), Value::Record(sum_rec));
        rec.insert(
            "mbt::actionTaken".to_string(),
            Value::String("OldAction".to_string()),
        );
        rec.insert("mbt::nondetPicks".to_string(), Value::Record(Record::new()));
        rec.insert(
            "state_var".to_string(),
            Value::String("state_value".to_string()),
        );

        let result = extract_from_sum_type(rec, &["sum_type"], &["state_var"]).unwrap();
        assert_eq!(result.action_taken, "TestAction");
    }
}
