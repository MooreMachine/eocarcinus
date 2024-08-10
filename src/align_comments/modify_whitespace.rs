pub(super) fn remove_trailing_whitespace(s: &str) -> &str {
    let characters: Vec<char> = s.chars().collect();
    let mut count = 0;
    for c in characters.iter().rev() {
        if !c.is_whitespace() {
            break;
        }
        count += 1;
    }
    let trailing_whitespace_index = s.len() - count;
    let (text, _trailing_whitespace) = s.split_at(trailing_whitespace_index);
    text
}

pub(super) fn remove_leading_whitespace(s: &str) -> &str {
    let count = s.find(|c: char| !c.is_whitespace()).unwrap_or_default();
    s.split_at(count).1
}

#[cfg(test)]
mod tests {
    use super::remove_leading_whitespace;

    #[test]
    fn leading_whitespace() {
        let leading = "    ";
        let message = "Hello, world!";
        let input = format!("{}{}", leading, message);

        let result = remove_leading_whitespace(&input);

        assert_eq!(result, message);
    }

    #[test]
    fn remove_leading_leave_trailing() {
        let leading = "    ";
        let trailing = "   ";
        let message = "Hello, world!";
        let input = format!("{}{}{}", leading, message, trailing);

        let result = remove_leading_whitespace(&input);

        assert!(!result.starts_with(char::is_whitespace));
        assert_eq!(result, format!("{}{}", message, trailing));
    }
}
