//! Email sending configuration data

use secrecy::SecretString;

use super::error::EmailError;

/// Configuration for the email sending service
#[must_use]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct EmailConfig {
    /// API endpoint for the email service
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

impl EmailConfig {
    /// Server API token
    pub const SENDOUT_API_TOKEN: &str = "SENDOUT_API_TOKEN";
    /// Email service API
    pub const SENDOUT_BASE_URL: &str = "SENDOUT_BASE_URL";
    /// Sender email address
    pub const SENDOUT_FROM_EMAIL: &str = "SENDOUT_FROM_EMAIL";

    /// Creates [`EmailConfig`] from environment variables
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sendout::config::EmailConfig;
    /// let email_config = EmailConfig::from_env()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(name = "EmailConfig::from_env", err(Debug))
    )]
    pub fn from_env() -> Result<Self, EmailError> {
        let base_url = std::env::var(Self::SENDOUT_BASE_URL).map_err(|_err| {
            #[cfg(feature = "tracing")]
            tracing::error!(%_err);
            EmailError::ConfigError(format!("{} not set", Self::SENDOUT_BASE_URL))
        })?;
        let api_token = std::env::var(Self::SENDOUT_API_TOKEN).map_err(|_err| {
            #[cfg(feature = "tracing")]
            tracing::error!(%_err);
            EmailError::ConfigError(format!("{} not set", Self::SENDOUT_API_TOKEN))
        })?;
        let from_email = std::env::var(Self::SENDOUT_FROM_EMAIL).map_err(|_err| {
            #[cfg(feature = "tracing")]
            tracing::error!(%_err);
            EmailError::ConfigError(format!("{} not set", Self::SENDOUT_FROM_EMAIL))
        })?;

        Ok(Self {
            api_token: SecretString::from(api_token),
            base_url,
            from_email,
        })
    }
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
