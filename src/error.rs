//! Error module

use thiserror::Error;

/// Errors that can occurs when sending an email
///
/// It represents all possible failures that can occur when attempting
/// to send an email via the email service.
#[derive(Debug, Clone, Error)]
pub enum SendoutError {
    /// Configuration error preventing the email from being sent out
    #[error("email configuration error: {0}")]
    ConfigError(String),

    /// The email failed to send due to a network error
    ///
    /// This includes connection failures, timeouts, and non-success HTTP
    /// responses from the email service except rate limiting.
    #[error("failed to send email: {0}")]
    SendFailed(String),

    /// The email service rate limit has been exhausted
    ///
    /// This error occurs when too many requests are made in short period
    #[error("rate limit exceeded")]
    RateLimitExceeded,

    /// The recipient email address is invalid or rejected
    ///
    /// This may occur if the email address format is invalid or if the
    /// email service rejects the recipient for policy reasons.
    #[error("invalid recipient: {0}")]
    InvalidRecipient(String),
}

#[cfg(test)]
mod tests {
    use super::SendoutError;

    #[test]
    fn test_email_error_variants() {
        let config_err = SendoutError::ConfigError("missing token".into());
        assert!(matches!(config_err, SendoutError::ConfigError(_)));

        let send_err = SendoutError::SendFailed("connection failed".into());
        assert!(matches!(send_err, SendoutError::SendFailed(_)));

        let rate_err = SendoutError::RateLimitExceeded;
        assert!(matches!(rate_err, SendoutError::RateLimitExceeded));

        let recipient_err = SendoutError::InvalidRecipient("bad@".into());
        assert!(matches!(recipient_err, SendoutError::InvalidRecipient(_)));
    }
}
