macro_rules! indent {
    ($level:literal, $fmt:literal $(, $args:expr)*) => {{
        let mut result = String::new();
        let prefix = " ".repeat($level);
        let text = format!($fmt $(, $args)*);

        for line in text.split_inclusive('\n') {
            result.push_str(&prefix);
            result.push_str(line);
        }

        result
    }};
}

pub(crate) use indent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident() {
        assert_eq!(indent!(0, "foo\nbar"), "foo\nbar");
        assert_eq!(indent!(2, "foo\nbar"), "  foo\n  bar");
        assert_eq!(indent!(2, "foo\nbar\n{}", "bazz"), "  foo\n  bar\n  bazz");
    }
}
