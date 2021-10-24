use rusoto_credential::AwsCredentials;

pub struct AwsProfileCredential {
    pub profile_name: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub token: Option<String>,
}

impl Default for AwsProfileCredential {
    fn default() -> Self {
        Self::new()
    }
}

impl AwsProfileCredential {
    pub fn new() -> AwsProfileCredential {
        AwsProfileCredential {
            profile_name: None,
            access_key: None,
            secret_key: None,
            token: None,
        }
    }

    pub fn new_with_profile_name(profile_name: String) -> AwsProfileCredential {
        AwsProfileCredential {
            profile_name: Some(profile_name),
            access_key: None,
            secret_key: None,
            token: None,
        }
    }

    pub fn into_aws_credential(self) -> Option<AwsCredentials> {
        if let (Some(access_key), Some(secret_key)) = (self.access_key, self.secret_key) {
            return Some(AwsCredentials::new(
                access_key, secret_key, self.token, None,
            ));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_aws_credential_should_return_aws_credential_when_both_access_key_and_secret_key_are_exist(
    ) {
        let access_key = "access_key";
        let secret_key = "secret_key";

        let aws_profile_credential = AwsProfileCredential {
            profile_name: None,
            access_key: Some(access_key.into()),
            secret_key: Some(secret_key.into()),
            token: None,
        };

        let result = aws_profile_credential.into_aws_credential().unwrap();
        let expected = AwsCredentials::new(access_key, secret_key, None, None);

        assert_eq!(expected.aws_access_key_id(), result.aws_access_key_id());
        assert_eq!(
            expected.aws_secret_access_key(),
            result.aws_secret_access_key()
        );
        assert_eq!(expected.token(), result.token());
    }

    #[test]
    fn into_aws_credential_should_return_none_when_either_or_both_access_key_and_secret_key_are_none(
    ) {
        let aws_profile_credential = AwsProfileCredential::new();

        assert!(aws_profile_credential.into_aws_credential().is_none());
    }
}
