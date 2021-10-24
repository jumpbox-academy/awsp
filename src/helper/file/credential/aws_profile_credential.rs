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
