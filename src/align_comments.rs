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
            align_consecutive_comments(&buffer, longest, &mut transformed);
            transformed.push(text.to_string());
            buffer.clear();
            longest = 0;
        }
    }

    if !buffer.is_empty() {
        align_consecutive_comments(&buffer, longest, &mut transformed);
    }
    transformed
}

fn align_consecutive_comments(
    buffer: &Vec<(&str, &str)>,
    longest: usize,
    transformed: &mut Vec<String>,
) {
    for (text, comment) in buffer {
        let padding = longest - text.len();
        transformed.push(format!(
            "{}{} {} {}",
            text,
            &str::repeat(" ", padding),
            COMMENT_DELIMITER,
            comment
        ));
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_line() {
        let text = "Hello, world!";
        let comment = "this is a comment";
        let input = format!("{} {} {}", text, COMMENT_DELIMITER, comment);

        let result = split_text_and_comments(&input);

        assert_eq!(result.first().unwrap().0, text);
        assert_eq!(result.first().unwrap().1.unwrap(), comment);
    }

    #[test]
    fn multi_line() {
        let text = "Hello, world!";
        let comment = "this is a comment";
        let input = format!("{}\n{} {}", text, COMMENT_DELIMITER, comment);

        let result = split_text_and_comments(&input);

        let mut it = result.iter();
        assert_eq!(*it.next().unwrap(), (text, None));
        assert_eq!(*it.next().unwrap(), ("", Some(comment)));
    }

    #[test]
    fn indented_line() {
        let text = "    Hello, world";
        let comment = "    this is a comment";
        let input = format!("{} {} {}", text, COMMENT_DELIMITER, comment);

        let result = split_text_and_comments(&input);

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
        let split = split_text_and_comments(&sample);

        let result = pad_text(split);

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
        let split = split_text_and_comments(&sample);

        let result = pad_text(split);

        let mut it = result.iter();
        let expected_with_comment = format!("{} {} {}", text, COMMENT_DELIMITER, comment); // The additional space has been removed
        assert_eq!(*it.next().unwrap(), expected_with_comment);
        assert_eq!(*it.next().unwrap(), only_text);
    }
}
