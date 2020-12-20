/// Get a substring between start (inclusive) and end (exclusive) indices of a `&str`
pub(crate) fn substring(input: &str, start: usize, end: usize) -> String {
    assert!(end > start);
    input.chars().skip(start).take(end - start).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substring() {
        assert_eq!(String::from(""), substring("", 0, 1));
        assert_eq!(String::from("2"), substring("2 3 4", 0, 1));
        assert_eq!(String::from("bcd"), substring("abcde", 1, 4));
        assert_eq!(String::from("beef"), substring("beef", 0, 4));
        assert_eq!(String::from(""), substring("apple", 5, 6));
        assert_eq!(String::from("apple"), substring("apple", 0, 999));
    }
}
