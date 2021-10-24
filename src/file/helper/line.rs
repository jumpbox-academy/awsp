use regex::Regex;

/// check if given line is described profile
pub fn is_profile(line: &str) -> bool {
    let profile_regex = new_profile_regex();

    profile_regex.is_match(line)
}

pub fn get_profile_name_from(line: &str) -> Option<String> {
    let profile_regex = new_profile_regex();
    let caps = profile_regex.captures(&line).unwrap();

    caps.get(2).map(|value| value.as_str().to_string())
}

fn new_profile_regex() -> Regex {
    Regex::new(r"^\[(profile )?([^\]]+)\]$").expect("Failed to compile regex")
}

pub fn is_comment_or_empty(line: &str) -> bool {
    line.is_empty() || is_comment(line)
}

pub fn is_comment(to_check: &str) -> bool {
    to_check.starts_with('#')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_profile_should_return_true_when_given_line_is_appropriate_profile_line() {
        let line = "[someProfile]";

        assert_eq!(true, is_profile(line));
    }

    #[test]
    fn is_profile_should_return_false_when_given_line_is_not_appropriate_profile() {
        let line = "some random text]][[";

        assert_eq!(false, is_profile(line));
    }

    #[test]
    fn is_comment_should_return_true_when_given_line_is_start_with_sharp() {
        let line = "# some comment";

        assert_eq!(true, is_comment(line));
    }

    #[test]
    fn is_comment_should_return_false_when_given_line_is_not_start_with_sharp() {
        let line = "some one line text in file";

        assert_eq!(false, is_comment(line));
    }
}
