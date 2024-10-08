pub(crate) fn align_comments(file: String, delimiter: &str) -> Vec<String> {
    let split = split_text_and_comments(&file, delimiter);

    pad_text(&split, delimiter)
}

fn pad_text(split: &[(&str, Option<&str>)], delimiter: &str) -> Vec<String> {
    let mut transformed = Vec::new();
    let mut buffer: Vec<(&str, &str)> = Vec::new();
    let mut longest = 0;

    for (text, comment) in split {
        if let Some(comment) = comment {
            if text.len() > longest {
                longest = text.len();
            }
            buffer.push((text, comment));
        } else {
            align_consecutive_comments(&buffer, longest, delimiter, &mut transformed);
            transformed.push(text.to_string());
            buffer.clear();
            longest = 0;
        }
    }

    if !buffer.is_empty() {
        align_consecutive_comments(&buffer, longest, delimiter, &mut transformed);
    }
    transformed
}

fn align_consecutive_comments(
    buffer: &[(&str, &str)],
    longest: usize,
    delimiter: &str,
    transformed: &mut Vec<String>,
) {
    for (text, comment) in buffer {
        let padding = longest - text.len();
        transformed.push(format!(
            "{}{} {} {}",
            text,
            &str::repeat(" ", padding),
            delimiter,
            comment
        ));
    }
}

fn split_text_and_comments<'a>(file: &'a str, delimiter: &str) -> Vec<(&'a str, Option<&'a str>)> {
    let mut split: Vec<(&str, Option<&str>)> = Vec::new();

    for line in file.lines() {
        if line.contains(delimiter) {
            let (text, comment) = line.split_once(delimiter).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    const COMMENT_DELIMITER: &str = "//";

    #[test]
    fn simple_line() {
        let text = "Hello, world!";
        let comment = "this is a comment";
        let input = format!("{} {} {}", text, COMMENT_DELIMITER, comment);

        let result = split_text_and_comments(&input, COMMENT_DELIMITER);

        assert_eq!(result.first().unwrap().0, text);
        assert_eq!(result.first().unwrap().1.unwrap(), comment);
    }

    #[test]
    fn multi_line() {
        let text = "Hello, world!";
        let comment = "this is a comment";
        let input = format!("{}\n{} {}", text, COMMENT_DELIMITER, comment);

        let result = split_text_and_comments(&input, COMMENT_DELIMITER);

        let mut it = result.iter();
        assert_eq!(*it.next().unwrap(), (text, None));
        assert_eq!(*it.next().unwrap(), ("", Some(comment)));
    }

    #[test]
    fn indented_line() {
        let text = "    Hello, world";
        let comment = "    this is a comment";
        let input = format!("{} {} {}", text, COMMENT_DELIMITER, comment);

        let result = split_text_and_comments(&input, COMMENT_DELIMITER);

        assert_eq!(result.first().unwrap().0, text);
        assert_eq!(result.first().unwrap().1.unwrap(), comment.trim());
    }

    #[test]
    fn two_lines_with_comments() {
        let hello = "hello";
        let this_is_a_comment = "this_is_a_comment";
        let you = "you";
        let this_is_also_a_comment = "this_is_also_a_comment";
        let input_a = format!("{hello} // {this_is_a_comment}");
        let input_b = format!("{you} // {this_is_also_a_comment}");
        let sample = format!("{}\n{}", input_a, input_b);
        let split = split_text_and_comments(&sample, COMMENT_DELIMITER);

        let result = pad_text(&split, COMMENT_DELIMITER);

        let mut it = result.iter();
        let expected_a = format!("{hello} // {this_is_a_comment}");
        let expected_b = format!("{you}   // {this_is_also_a_comment}");
        assert_eq!(*it.next().unwrap(), expected_a);
        assert_eq!(*it.next().unwrap(), expected_b);
    }

    #[test]
    fn one_with_comments_another_without() {
        let text = "Hello, world!";
        let comment = "this is a comment";
        let with_comment = format!("{}    {} {}", text, COMMENT_DELIMITER, comment);
        let only_text = "No comment here";
        let sample = format!("{}\n{}", with_comment, only_text);
        let split = split_text_and_comments(&sample, COMMENT_DELIMITER);

        let result = pad_text(&split, COMMENT_DELIMITER);

        let mut it = result.iter();
        let expected_with_comment = format!("{} {} {}", text, COMMENT_DELIMITER, comment); // The additional space has been removed
        assert_eq!(*it.next().unwrap(), expected_with_comment);
        assert_eq!(*it.next().unwrap(), only_text);
    }
}
