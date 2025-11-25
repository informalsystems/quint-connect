use crate::itf::Value;

pub trait OptionValue {
    fn into_option(self) -> Option<Value>;
}

impl OptionValue for Value {
    fn into_option(self) -> Option<Value> {
        if let Value::Record(mut record) = self
            && let Some(Value::String(tag)) = record.get("tag")
            && tag == "Some"
        {
            return record.remove("value");
        }
        None
    }
}
