use crate::itf::Value;

pub trait ValueReader {
    fn get<'a>(&'a self, path: &[&str]) -> Option<&'a Value>;
}

impl ValueReader for Value {
    fn get<'a>(&'a self, path: &[&str]) -> Option<&'a Value> {
        let mut value = Some(self);

        for segment in path {
            if let Some(Value::Record(rec)) = value {
                value = rec.get(segment)
            } else {
                return None;
            }
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::itf::value::Record;

    #[test]
    fn test_read_from_record() {
        let mut record = Record::new();
        record.insert("foo".to_string(), Value::Number(42));

        let record = Value::Record(record);
        let value = record.get(&["foo"]).unwrap();
        assert_eq!(value, &Value::Number(42));
    }

    #[test]
    fn test_read_from_nested_record() {
        let mut nested = Record::new();
        nested.insert("bar".to_string(), Value::Number(42));

        let mut record = Record::new();
        record.insert("foo".to_string(), Value::Record(nested));

        let record = Value::Record(record);
        let value = record.get(&["foo", "bar"]).unwrap();
        assert_eq!(value, &Value::Number(42));
    }

    #[test]
    fn test_read_from_non_record() {
        let number = Value::Number(42);
        let value = number.get(&["foo"]);
        assert_eq!(value, None);
    }

    #[test]
    fn test_read_from_invalid_path() {
        let mut nested = Record::new();
        nested.insert("bar".to_string(), Value::Number(42));

        let mut record = Record::new();
        record.insert("foo".to_string(), Value::Record(nested));

        let record = Value::Record(record);
        let value = record.get(&["foo", "bazzz"]);
        assert_eq!(value, None);
    }
}
