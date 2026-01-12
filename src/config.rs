//! Email sending configuration data

use secrecy::SecretString;

/// Configuration for the email sending service
#[must_use]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct EmailConfig {
    /// API endpoint for the email service provider
    pub base_url: String,
    /// Secret API token for authentication
    ///
    /// This token is used when making API requests
    pub api_token: SecretString,

    /// The verified sender email address
    ///
    /// This email must be a sender verified by your email service provider
    /// Emails will appears to come from this address
    pub from_email: String,
}

#[cfg(test)]
mod tests {
    use super::EmailConfig;
    use secrecy::{ExposeSecret, SecretString};

    #[test]
    fn test_email_config() {
        let config = EmailConfig {
            api_token: SecretString::from(String::from("test-token")),
            from_email: "from@test.com".into(),
            base_url: "http://localhost:6666".into(),
        };

        assert_eq!(config.api_token.expose_secret(), "test-token");
        assert_eq!(config.from_email, "from@test.com");
        assert_eq!(config.base_url, "http://localhost:6666");
    }
}
