mod modify_whitespace;

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
            let text = text.trim_end();
            let comment = comment.trim_start();

            split.push((text, Some(comment)));
        } else {
            let text = line.trim_end();
            split.push((text, None));
        }
    }
    split
}
