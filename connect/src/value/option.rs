use itf::Value;

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
