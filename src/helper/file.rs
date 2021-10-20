use regex::Regex;

pub mod config;
pub mod credential;

pub fn is_profile(line: &str) -> bool {
    let profile_regex = new_profile_regex();

    profile_regex.is_match(line)
}

pub fn new_profile_regex() -> Regex {
    Regex::new(r"^\[(profile )?([^\]]+)\]$").expect("Failed to compile regex")
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
}
