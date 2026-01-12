//! Email sending configuration data

use secrecy::SecretBox;

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
    pub api_token: SecretBox<String>,

    /// The verified sender email address
    ///
    /// This email must be a sender verified by your email service provider
    /// Emails will appears to come from this address
    pub from_email: String,
}
