use itf::value::{Map, Record, Value};
use std::fmt::{Display, Formatter, Result};

pub(crate) trait ValueDisplay {
    fn display(&self) -> impl Display;
}

impl ValueDisplay for Value {
    fn display(&self) -> impl Display {
        AsQuintValue(self)
    }
}

struct AsQuintValue<'a>(&'a Value);

impl Display for AsQuintValue<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.0 {
            Value::Bool(value) => write!(f, "{}", value),
            Value::Number(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "\"{}\"", value),
            Value::BigInt(value) => write!(f, "{}", value),
            Value::List(values) => write_col(f, "List(", values, ")"),
            Value::Tuple(values) => write_col(f, "(", values.iter(), ")"),
            Value::Set(values) => write_col(f, "Set(", values.iter(), ")"),
            Value::Map(values) => write_map(f, values),
            Value::Record(values) => write_rec(f, values),
            Value::Unserializable(value) => write!(f, "{:?}", value),
        }
    }
}

impl ValueDisplay for Record {
    fn display(&self) -> impl Display {
        AsQuintRecord(self)
    }
}

struct AsQuintRecord<'a>(&'a Record);

impl Display for AsQuintRecord<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write_rec(f, self.0)
    }
}

fn write_col<'a, Iter>(f: &mut Formatter, open: &str, elems: Iter, close: &str) -> Result
where
    Iter: IntoIterator<Item = &'a Value>,
{
    write!(f, "{}", open)?;
    write_elems(f, elems)?;
    write!(f, "{}", close)
}

fn write_elems<'a, Iter>(f: &mut Formatter, elements: Iter) -> Result
where
    Iter: IntoIterator<Item = &'a Value>,
{
    let mut iter = elements.into_iter();
    if let Some(first) = iter.next() {
        write!(f, "{}", first.display())?;
        for value in iter {
            write!(f, ", {}", value.display())?;
        }
    }
    Ok(())
}

fn write_map(f: &mut Formatter, map: &Map<Value, Value>) -> Result {
    write!(f, "Map(")?;
    let mut iter = map.iter();
    if let Some((key, value)) = iter.next() {
        write!(f, "{} -> {}", key.display(), value.display())?;
        for (key, value) in iter {
            write!(f, ", {} -> {}", key.display(), value.display())?;
        }
    }
    write!(f, ")")
}

fn write_rec(f: &mut Formatter, rec: &Record) -> Result {
    if rec.len() == 2
        && let Some(Value::String(tag)) = rec.get("tag")
        && let Some(value) = rec.get("value")
    {
        // Special case for tagged values
        write!(f, "{}", tag)?;
        if let tuple @ Value::Tuple(values) = value {
            if !values.is_empty() {
                write!(f, "{}", tuple.display())?;
            }
        } else {
            write!(f, "({})", value.display())?;
        }
    } else {
        // General data record
        write!(f, "{{ ")?;
        let mut iter = rec.iter();
        if let Some((key, value)) = iter.next() {
            write!(f, "{}: {}", key, value.display())?;
            for (key, value) in iter {
                write!(f, ", {}: {}", key, value.display())?;
            }
        }
        write!(f, " }}")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use itf::value::BigInt;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn test_display_scalars() {
        assert_display(Value::Bool(true), "true");
        assert_display(Value::Number(42), "42");
        assert_display(Value::String("foo".to_string()), "\"foo\"");
        assert_display(Value::BigInt(BigInt::new(42)), "42");
    }

    #[test]
    fn test_display_list() {
        let values = vec![Value::Number(42), Value::Bool(true)];
        assert_display(Value::List(values), "List(42, true)")
    }

    #[test]
    fn test_display_tuple() {
        let values = vec![Value::Number(42), Value::Bool(true)];
        assert_display(Value::Tuple(values.into()), "(42, true)")
    }

    #[test]
    fn test_display_set() {
        let mut values = BTreeSet::new();
        values.insert(Value::Number(42));
        values.insert(Value::Bool(true));
        assert_display(Value::Set(values.into()), "Set(true, 42)")
    }

    #[test]
    fn test_display_map() {
        let mut values = BTreeMap::new();
        values.insert(Value::String("num".to_string()), Value::Number(42));
        values.insert(Value::String("bool".to_string()), Value::Bool(false));
        assert_display(
            Value::Map(values.into()),
            "Map(\"bool\" -> false, \"num\" -> 42)",
        )
    }

    #[test]
    fn test_display_rec() {
        let mut values = Record::new();
        values.insert("num".to_string(), Value::Number(42));
        values.insert("bool".to_string(), Value::Bool(false));
        assert_display(Value::Record(values), "{ bool: false, num: 42 }")
    }

    #[test]
    fn test_display_tagged_rec() {
        let mut values = Record::new();
        values.insert("tag".to_string(), Value::String("Foo".to_string()));
        values.insert("value".to_string(), Value::Tuple(vec![].into()));
        assert_display(Value::Record(values), "Foo");

        let mut values = Record::new();
        values.insert("tag".to_string(), Value::String("Foo".to_string()));
        values.insert(
            "value".to_string(),
            Value::Tuple(vec![Value::Number(42)].into()),
        );
        assert_display(Value::Record(values), "Foo(42)");
    }

    fn assert_display(value: Value, expected: &str) {
        assert_eq!(format!("{}", value.display()), expected);
    }
}
