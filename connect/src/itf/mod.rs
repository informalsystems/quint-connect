pub mod option;

pub use itf::*;

pub(crate) fn read_from<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut value = Some(value);

    for segment in path {
        if let Some(Value::Record(rec)) = value {
            value = rec.get(segment)
        } else {
            return None;
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use value::Record;

    #[test]
    fn test_read_from_record() {
        let mut record = Record::new();
        record.insert("foo".to_string(), Value::Number(42));

        let record = Value::Record(record);
        let value = read_from(&record, &["foo"]).unwrap();
        assert_eq!(value, &Value::Number(42));
    }

    #[test]
    fn test_read_from_nested_record() {
        let mut nested = Record::new();
        nested.insert("bar".to_string(), Value::Number(42));

        let mut record = Record::new();
        record.insert("foo".to_string(), Value::Record(nested));

        let record = Value::Record(record);
        let value = read_from(&record, &["foo", "bar"]).unwrap();
        assert_eq!(value, &Value::Number(42));
    }

    #[test]
    fn test_read_from_non_record() {
        let number = Value::Number(42);
        let value = read_from(&number, &["foo"]);
        assert_eq!(value, None);
    }

    #[test]
    fn test_read_from_invalid_path() {
        let mut nested = Record::new();
        nested.insert("bar".to_string(), Value::Number(42));

        let mut record = Record::new();
        record.insert("foo".to_string(), Value::Record(nested));

        let record = Value::Record(record);
        let value = read_from(&record, &["foo", "bazzz"]);
        assert_eq!(value, None);
    }
}
