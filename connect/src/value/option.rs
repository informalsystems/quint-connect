use itf::Value;

/// Internal trait for converting Quint types to Rust Option types.
pub(crate) trait ValueOption {
    fn into_option(self) -> Option<Value>;
}

impl ValueOption for Value {
    fn into_option(self) -> Option<Value> {
        match self {
            Value::Record(mut rec) => match rec.get("tag") {
                Some(Value::String(tag)) if tag == "Some" => rec.remove("value"),
                Some(Value::String(tag)) if tag == "None" => None,
                _ => Some(Value::Record(rec)),
            },
            other => Some(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itf::value::Record;

    #[test]
    fn test_some_with_value() {
        let mut rec = Record::new();
        rec.insert("tag".to_string(), Value::String("Some".to_string()));
        rec.insert("value".to_string(), Value::Number(42));

        let result = Value::Record(rec).into_option();
        assert_eq!(result, Some(Value::Number(42)));
    }

    #[test]
    fn test_none() {
        let mut rec = Record::new();
        rec.insert("tag".to_string(), Value::String("None".to_string()));

        let result = Value::Record(rec).into_option();
        assert_eq!(result, None);
    }

    #[test]
    fn test_non_option_record() {
        let mut rec = Record::new();
        rec.insert("foo".to_string(), Value::Number(42));
        rec.insert("bar".to_string(), Value::Bool(true));

        let original = Value::Record(rec.clone());
        let result = original.into_option();
        assert_eq!(result, Some(Value::Record(rec)));
    }

    #[test]
    fn test_record_with_non_string_tag() {
        let mut rec = Record::new();
        rec.insert("tag".to_string(), Value::Number(42));
        rec.insert("value".to_string(), Value::String("test".to_string()));

        let original = Value::Record(rec.clone());
        let result = original.into_option();
        assert_eq!(result, Some(Value::Record(rec)));
    }
}
