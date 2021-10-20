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
