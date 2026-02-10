//! Email sending configuration data

use std::env::VarError;

use secrecy::SecretString;

use super::error::Error;

/// Configuration for the email sending service
#[must_use]
#[derive(Debug, serde::Deserialize)]
pub struct ServiceConfig {
    /// API endpoint for the email service
    pub base_url: String,
    /// Secret API token for authentication
    ///
    /// This token is used when making API requests
    /// When using Postmark, it corresponds to X-Postmark-Server-Token
    pub server_token: SecretString,
    /// Token used for requests that require account level privileges
    ///
    /// For Postmark, it corresponds to X-Postmark-Account-Token
    pub account_token: Option<SecretString>,
    /// The verified sender email address
    ///
    /// This email must be a sender verified by your email service provider
    /// Emails will appears to come from this address
    pub from_email: String,
}

impl ServiceConfig {
    /// Account API token
    pub const SENDOUT_ACCOUNT_TOKEN: &str = "SENDOUT_ACCOUNT_TOKEN";
    /// Email service API
    pub const SENDOUT_BASE_URL: &str = "SENDOUT_BASE_URL";
    /// Sender email address
    pub const SENDOUT_FROM_EMAIL: &str = "SENDOUT_FROM_EMAIL";
    /// Server API token
    pub const SENDOUT_SERVER_TOKEN: &str = "SENDOUT_SERVER_TOKEN";

    /// Creates [`ServiceConfig`] from environment variables
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use sendout::config::ServiceConfig;
    /// let email_config = ServiceConfig::from_env()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(name = "ServiceConfig::from_env", err(Debug))
    )]
    pub fn from_env() -> Result<Self, Error> {
        let base_url = std::env::var(Self::SENDOUT_BASE_URL).map_err(|_err| {
            #[cfg(feature = "tracing")]
            tracing::error!(%_err);
            Error::ConfigError(format!("{} not set", Self::SENDOUT_BASE_URL))
        })?;
        let server_token = std::env::var(Self::SENDOUT_SERVER_TOKEN)
            .map_err(|_err| {
                #[cfg(feature = "tracing")]
                tracing::error!(%_err);
                Error::ConfigError(format!("{} not set", Self::SENDOUT_SERVER_TOKEN))
            })
            .map(SecretString::from)?;
        let from_email = std::env::var(Self::SENDOUT_FROM_EMAIL).map_err(|_err| {
            #[cfg(feature = "tracing")]
            tracing::error!(%_err);
            Error::ConfigError(format!("{} not set", Self::SENDOUT_FROM_EMAIL))
        })?;

        let account_token = match std::env::var(Self::SENDOUT_ACCOUNT_TOKEN) {
            Ok(token) => Some(SecretString::from(token)),
            Err(VarError::NotPresent) => None,
            Err(_err) => {
                #[cfg(feature = "tracing")]
                tracing::error!(%_err);
                let error =
                    Error::ConfigError(format!("{} not set", Self::SENDOUT_ACCOUNT_TOKEN));
                return Err(error);
            }
        };

        Ok(Self {
            account_token,
            server_token,
            base_url,
            from_email,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceConfig;
    use secrecy::{ExposeSecret, SecretString};

    #[test]
    fn email_config() {
        let config = ServiceConfig {
            server_token: SecretString::from(String::from("test-token")),
            from_email: "from@test.com".into(),
            base_url: "http://localhost:6666".into(),
            account_token: Some(SecretString::from(String::from("test-account-token"))),
        };

        assert_eq!(config.server_token.expose_secret(), "test-token");
        assert_eq!(config.from_email, "from@test.com");
        assert_eq!(config.base_url, "http://localhost:6666");
    }
}
