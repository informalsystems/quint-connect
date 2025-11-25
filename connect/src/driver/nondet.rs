use crate::itf::{
    option::OptionValue,
    value::{Record, Value},
};
use anyhow::{Result, bail};
use serde::de::DeserializeOwned;

pub struct NondetPicks(Record);

impl NondetPicks {
    pub(crate) fn new(value: Value) -> Result<Self> {
        let Value::Record(record) = value else {
            bail!("Expected nondet picks to be a `Value::Record`")
        };

        let mut nondets = Record::new();
        for (key, value) in record {
            if let Some(value) = value.into_option() {
                nondets.insert(key, value);
            }
        }

        Ok(Self(nondets))
    }

    pub fn get<'a>(&'a self, var: &str) -> Option<NondetPick<'a>> {
        self.0.get(var).map(NondetPick)
    }
}

pub struct NondetPick<'a>(&'a Value);

impl<'a> NondetPick<'a> {
    pub fn try_into<T>(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(T::deserialize(self.0.clone())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Expected nondet picks to be a `Value::Record`")]
    fn test_fail_to_build_nondet_picks() {
        let value = Value::Number(42);
        NondetPicks::new(value).unwrap();
    }

    #[test]
    fn test_get_nondet_pick() {
        let mut option = Record::new();
        option.insert("tag".to_string(), Value::String("Some".to_string()));
        option.insert("value".to_string(), Value::Number(42));

        let mut record = Record::new();
        record.insert("foo".to_string(), Value::Record(option));

        let nondets = NondetPicks::new(Value::Record(record)).unwrap();
        let nondet = nondets.get("foo");

        assert!(nondet.is_some(), "failed to find nondet value")
    }

    #[test]
    fn test_decode_nondet_pick() {
        let nondet = Value::Number(42);
        let nondet = NondetPick(&nondet);
        let value: i64 = nondet.try_into().unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    #[should_panic(expected = "invalid type: integer `42`, expected a boolean")]
    fn test_failed_to_decode_nondet_pick() {
        let nondet = Value::Number(42);
        let nondet = NondetPick(&nondet);
        nondet.try_into::<bool>().unwrap();
    }
}
