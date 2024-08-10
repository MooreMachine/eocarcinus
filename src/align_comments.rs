const COMMENT_DELIMITER: &str = "//";

pub(crate) fn align_comments(file: String) {
    let split = split_text_and_comments(&file);

    let transformed = pad_text(split);

    for line in transformed {
        println!("{}", line);
    }
}

fn pad_text(split: Vec<(&str, Option<&str>)>) -> Vec<String> {
    let mut transformed = Vec::new();
    let mut buffer = Vec::new();
    let mut longest = 0;

    for (text, comment) in split {
        if let Some(comment) = comment {
            if text.len() > longest {
                longest = text.len();
            }
            buffer.push((text, comment));
        } else {
            for (a, b) in &buffer {
                let mut a = a.to_string();
                let padding = longest - a.len();
                a.push_str(&str::repeat(" ", padding));
                transformed.push(format!("{} {} {}", a, COMMENT_DELIMITER, b));
            }
            transformed.push(text.to_string());
            buffer.clear();
            longest = 0;
        }
    }

    if !buffer.is_empty() {
        for (a, b) in &buffer {
            let mut a = a.to_string();
            let padding = longest - a.len();
            a.push_str(&str::repeat(" ", padding));
            transformed.push(format!("{} {} {}", a, COMMENT_DELIMITER, b));
        }
    }
    transformed
}

fn split_text_and_comments(file: &str) -> Vec<(&str, Option<&str>)> {
    let mut split: Vec<(&str, Option<&str>)> = Vec::new();

    for line in file.lines() {
        if line.contains(COMMENT_DELIMITER) {
            let (text, comment) = line.split_once(COMMENT_DELIMITER).unwrap();
            let text = remove_trailing_whitespace(text);
            let comment = remove_leading_whitespace(comment);

            split.push((text, Some(comment)));
        } else {
            let text = remove_trailing_whitespace(line);
            split.push((text, None));
        }
    }
    split
}

fn remove_trailing_whitespace(s: &str) -> &str {
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

fn remove_leading_whitespace(s: &str) -> &str {
    let count = s.find(|c: char| !c.is_whitespace()).unwrap_or_default();
    s.split_at(count).1
}
