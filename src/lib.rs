use dirs::home_dir;
use regex::Regex;
use rusoto_credential::{AwsCredentials, CredentialsError};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::{collections::HashMap, env::var};

const AWS_CONFIG_FILE: &str = "AWS_CONFIG_FILE";

fn non_empty_env_var(name: &str) -> Option<String> {
    match var(name) {
        Ok(value) => {
            if value.is_empty() {
                None
            } else {
                Some(value)
            }
        }
        Err(_) => None,
    }
}

/// Default config file location:
/// 1: if set and not empty, use the value from environment variable ```AWS_CONFIG_FILE```
/// 2. otherwise return `~/.aws/config` (Linux/Mac) resp. `%USERPROFILE%\.aws\config` (Windows)
pub fn default_config_location() -> Result<PathBuf, CredentialsError> {
    let env = non_empty_env_var(AWS_CONFIG_FILE);
    match env {
        Some(path) => Ok(PathBuf::from(path)),
        None => hardcoded_config_location(),
    }
}

fn hardcoded_config_location() -> Result<PathBuf, CredentialsError> {
    match home_dir() {
        Some(mut home_path) => {
            home_path.push(".aws");
            home_path.push("config");
            Ok(home_path)
        }
        None => Err(CredentialsError::new("Failed to determine home directory.")),
    }
}

fn new_profile_regex() -> Regex {
    Regex::new(r"^\[(profile )?([^\]]+)\]$").expect("Failed to compile regex")
}

pub fn parse_config_file(file_path: &Path) -> Option<HashMap<String, HashMap<String, String>>> {
    match fs::metadata(file_path) {
        Err(_) => return None,
        Ok(metadata) => {
            if !metadata.is_file() {
                return None;
            }
        }
    };
    let profile_regex = new_profile_regex();
    let file = File::open(file_path).expect("expected file");
    let file_lines = BufReader::new(&file);
    let result: (HashMap<String, HashMap<String, String>>, Option<String>) = file_lines
        .lines()
        .filter_map(|line| {
            line.ok()
                .map(|l| l.trim_matches(' ').to_owned())
                .into_iter()
                .find(|l| !l.starts_with('#') || !l.is_empty())
        })
        .fold(Default::default(), |(mut result, profile), line| {
            if profile_regex.is_match(&line) {
                let caps = profile_regex.captures(&line).unwrap();
                let next_profile = caps.get(2).map(|value| value.as_str().to_string());
                (result, next_profile)
            } else {
                match &line
                    .splitn(2, '=')
                    .map(|value| value.trim_matches(' '))
                    .collect::<Vec<&str>>()[..]
                {
                    [key, value] if !key.is_empty() && !value.is_empty() => {
                        if let Some(current) = profile.clone() {
                            let values = result.entry(current).or_insert_with(HashMap::new);
                            (*values).insert(key.to_string(), value.to_string());
                        }
                        (result, profile)
                    }
                    _ => (result, profile),
                }
            }
        });
    Some(result.0)
}

pub fn parse_credentials_file(
    file_path: &Path,
) -> Result<HashMap<String, AwsCredentials>, CredentialsError> {
    match fs::metadata(file_path) {
        Err(_) => {
            return Err(CredentialsError::new(format!(
                "Couldn't stat credentials file: [ {:?} ]. Non existant, or no permission.",
                file_path
            )))
        }
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(CredentialsError::new(format!(
                    "Credentials file: [ {:?} ] is not a file.",
                    file_path
                )));
            }
        }
    };

    let file = File::open(file_path)?;

    let profile_regex = new_profile_regex();
    let mut profiles: HashMap<String, AwsCredentials> = HashMap::new();
    let mut access_key: Option<String> = None;
    let mut secret_key: Option<String> = None;
    let mut token: Option<String> = None;
    let mut profile_name: Option<String> = None;

    let file_lines = BufReader::new(&file);
    for (line_no, line) in file_lines.lines().enumerate() {
        let unwrapped_line: String =
            line.unwrap_or_else(|_| panic!("Failed to read credentials file, line: {}", line_no));

        // skip empty lines
        if unwrapped_line.is_empty() {
            continue;
        }

        // skip comments
        if unwrapped_line.starts_with('#') {
            continue;
        }

        // handle the opening of named profile blocks
        if profile_regex.is_match(&unwrapped_line) {
            if profile_name.is_some() && access_key.is_some() && secret_key.is_some() {
                let creds =
                    AwsCredentials::new(access_key.unwrap(), secret_key.unwrap(), token, None);
                profiles.insert(profile_name.unwrap(), creds);
            }

            access_key = None;
            secret_key = None;
            token = None;

            let caps = profile_regex.captures(&unwrapped_line).unwrap();
            profile_name = Some(caps.get(2).unwrap().as_str().to_string());
            continue;
        }

        // otherwise look for key=value pairs we care about
        let lower_case_line = unwrapped_line.to_ascii_lowercase().to_string();

        if lower_case_line.contains("aws_access_key_id") && access_key.is_none() {
            let v: Vec<&str> = unwrapped_line.split('=').collect();
            if !v.is_empty() {
                access_key = Some(v[1].trim_matches(' ').to_string());
            }
        } else if lower_case_line.contains("aws_secret_access_key") && secret_key.is_none() {
            let v: Vec<&str> = unwrapped_line.split('=').collect();
            if !v.is_empty() {
                secret_key = Some(v[1].trim_matches(' ').to_string());
            }
        } else if lower_case_line.contains("aws_session_token") && token.is_none() {
            let v: Vec<&str> = unwrapped_line.split('=').collect();
            if !v.is_empty() {
                token = Some(v[1].trim_matches(' ').to_string());
            }
        } else if lower_case_line.contains("aws_security_token") {
            if token.is_none() {
                let v: Vec<&str> = unwrapped_line.split('=').collect();
                if !v.is_empty() {
                    token = Some(v[1].trim_matches(' ').to_string());
                }
            }
        } else {
            // Ignore unrecognized fields
            continue;
        }
    }

    if profile_name.is_some() && access_key.is_some() && secret_key.is_some() {
        let creds = AwsCredentials::new(access_key.unwrap(), secret_key.unwrap(), token, None);
        profiles.insert(profile_name.unwrap(), creds);
    }

    if profiles.is_empty() {
        return Err(CredentialsError::new("No credentials found."));
    }

    Ok(profiles)
}

#[cfg(test)]
mod tests {

    use std::path::Path;

    const DEFAULT: &str = "default";
    const REGION: &str = "region";

    #[test]
    fn parse_config_file_default_profile() {
        let result = super::parse_config_file(Path::new("tests/sample-data/default_config"));
        assert!(result.is_some());
        let profiles = result.unwrap();
        assert_eq!(profiles.len(), 1);
        let default_profile = profiles
            .get(DEFAULT)
            .expect("No Default profile in default_profile_credentials");
        assert_eq!(default_profile.get(REGION), Some(&"us-east-2".to_string()));
        assert_eq!(default_profile.get("output"), Some(&"json".to_string()));
    }

    #[test]
    fn parse_config_file_multiple_profiles() {
        let result =
            super::parse_config_file(Path::new("tests/sample-data/multiple_profile_config"));
        assert!(result.is_some());

        let profiles = result.unwrap();
        assert_eq!(profiles.len(), 3);

        let foo_profile = profiles
            .get("foo")
            .expect("No foo profile in multiple_profile_credentials");
        assert_eq!(foo_profile.get(REGION), Some(&"us-east-3".to_string()));
        assert_eq!(foo_profile.get("output"), Some(&"json".to_string()));

        let bar_profile = profiles
            .get("bar")
            .expect("No bar profile in multiple_profile_credentials");
        assert_eq!(bar_profile.get(REGION), Some(&"us-east-4".to_string()));
        assert_eq!(bar_profile.get("output"), Some(&"json".to_string()));
    }

    #[test]
    fn parse_config_file_credential_process() {
        let result =
            super::parse_config_file(Path::new("tests/sample-data/credential_process_config"));
        assert!(result.is_some());
        let profiles = result.unwrap();
        assert_eq!(profiles.len(), 2);
        let default_profile = profiles
            .get(DEFAULT)
            .expect("No Default profile in default_profile_credentials");
        assert_eq!(default_profile.get(REGION), Some(&"us-east-1".to_string()));
        assert_eq!(
            default_profile.get("credential_process"),
            Some(&"cat tests/sample-data/credential_process_sample_response".to_string())
        );
    }

    #[test]
    fn parse_credentials_file_default_profile() {
        let result = super::parse_credentials_file(Path::new(
            "tests/sample-data/default_profile_credentials",
        ));
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 1);

        let default_profile = profiles
            .get(DEFAULT)
            .expect("No Default profile in default_profile_credentials");
        assert_eq!(default_profile.aws_access_key_id(), "foo");
        assert_eq!(default_profile.aws_secret_access_key(), "bar");
    }

    #[test]
    fn parse_credentials_file_multiple_profiles() {
        let result = super::parse_credentials_file(Path::new(
            "tests/sample-data/multiple_profile_credentials",
        ));
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 2);

        let foo_profile = profiles
            .get("foo")
            .expect("No foo profile in multiple_profile_credentials");
        assert_eq!(foo_profile.aws_access_key_id(), "foo_access_key");
        assert_eq!(foo_profile.aws_secret_access_key(), "foo_secret_key");

        let bar_profile = profiles
            .get("bar")
            .expect("No bar profile in multiple_profile_credentials");
        assert_eq!(bar_profile.aws_access_key_id(), "bar_access_key");
        assert_eq!(bar_profile.aws_secret_access_key(), "bar_secret_key");
    }

    #[test]
    fn parse_all_values_credentials_file() {
        let result =
            super::parse_credentials_file(Path::new("tests/sample-data/full_profile_credentials"));
        assert!(result.is_ok());

        let profiles = result.ok().unwrap();
        assert_eq!(profiles.len(), 1);

        let default_profile = profiles
            .get(DEFAULT)
            .expect("No default profile in full_profile_credentials");
        assert_eq!(default_profile.aws_access_key_id(), "foo");
        assert_eq!(default_profile.aws_secret_access_key(), "bar");
    }
}