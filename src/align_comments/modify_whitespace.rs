pub(crate) fn remove_trailing_whitespace(s: &str) -> &str {
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

pub(crate) fn remove_leading_whitespace(s: &str) -> &str {
    let count = s.find(|c: char| !c.is_whitespace()).unwrap_or_default();
    s.split_at(count).1
}
