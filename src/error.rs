//! Error that can come up when sending a request to a service provider

/// Errors that can occurs when talking a service provider
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    /// Configuration error preventing the email from being sent out
    #[error("email configuration error: {0}")]
    ConfigError(String),

    /// The email failed to send due to a network error
    ///
    /// This includes connection failures, timeouts, and non-success HTTP
    /// responses from the email service except rate limiting.
    #[error("failed to send email: {0}")]
    SendFailed(String),

    /// Rate limit hit when interacting with a service provider
    ///
    /// Back off and retry after a bit
    #[error("rate limit exceeded")]
    RateLimitExceeded,

    /// The recipient email address is invalid or rejected
    ///
    /// This may occur if the email address format is invalid or if the
    /// email service rejects the recipient for policy reasons.
    #[error("invalid recipient: {0}")]
    InvalidRecipient(String),
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        if error.is_connect() {
            Error::SendFailed("connection failed".into())
        } else if error.is_timeout() {
            Error::SendFailed("connection timeout".into())
        } else {
            Error::SendFailed(error.to_string())
        }
    }
}
