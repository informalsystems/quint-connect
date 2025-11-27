use crate::itf::{
    display::ValueDisplay,
    option::OptionValue,
    value::{Record, Value},
};
use anyhow::{Result, bail};
use std::fmt;

pub struct NondetPicks(Record);

impl NondetPicks {
    pub(crate) fn from_value(value: Value) -> Result<Self> {
        let Value::Record(record) = value else {
            bail!("Expected nondet picks to be a `Value::Record`")
        };
        Ok(Self::from_record(record))
    }

    pub(crate) fn from_record(record: Record) -> Self {
        let mut nondets = Record::new();
        for (key, value) in record {
            if let Some(value) = value.into_option() {
                nondets.insert(key, value);
            }
        }

        Self(nondets)
    }

    pub(crate) fn empty() -> Self {
        Self(Record::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get<'a>(&'a self, var: &str) -> Option<&'a Value> {
        self.0.get(var)
    }
}

impl fmt::Display for NondetPicks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iter = self.0.iter();
        if let Some((key, value)) = iter.next() {
            write!(f, "+ {}: {}", key, value.display())?;
            for (key, value) in iter {
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
    #[should_panic(expected = "Expected nondet picks to be a `Value::Record`")]
    fn test_fail_to_build_nondet_picks() {
        let value = Value::Number(42);
        NondetPicks::from_value(value).unwrap();
    }

    #[test]
    fn test_get_nondet_pick() {
        let mut option = Record::new();
        option.insert("tag".to_string(), Value::String("Some".to_string()));
        option.insert("value".to_string(), Value::Number(42));

        let mut record = Record::new();
        record.insert("foo".to_string(), Value::Record(option));

        let nondets = NondetPicks::from_value(Value::Record(record)).unwrap();
        let nondet = nondets.get("foo");

        assert!(nondet.is_some(), "failed to find nondet value")
    }

    #[test]
    fn test_display_nondet_picks() {
        let empty = NondetPicks::from_value(Value::Record(Record::new())).unwrap();
        assert_eq!(format!("{}", empty), "");

        let mut option = Record::new();
        option.insert("tag".to_string(), Value::String("Some".to_string()));
        option.insert("value".to_string(), Value::Number(42));

        let mut record = Record::new();
        record.insert("foo".to_string(), Value::Record(option.clone()));
        record.insert("bar".to_string(), Value::Record(option));

        let non_empty = NondetPicks::from_value(Value::Record(record)).unwrap();
        assert_eq!(
            format!("{}", non_empty),
            "+ bar: 42\n\
             + foo: 42"
        );
    }
}
