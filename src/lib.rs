//! # Sendout
//!
//! Provides an abstraction over sending emails using APIs
pub mod config;
pub mod error;
#[macro_use]
pub mod macros;

use async_trait::async_trait;

use self::error::SendoutError;

/// Trait for sending emails
///
/// This trait defines the mechanism for sending emails.
#[async_trait]
pub trait Sendout<Email>: Send + Sync {
    /// Send an email
    async fn send(&self, email: Email) -> Result<(), SendoutError>;
}

cfg_test_util! {
    use std::sync::{Arc, Mutex};

    /// A list to sent emails
    type Outbox<Email> = Arc<Mutex<Vec<Email>>>;

    /// Mock sender that records sent emails
    pub struct MockEmailSender<Email> {
        /// The error to return when failure is expected
        pub failure_error: Option<SendoutError>,
        /// Records sent emails
        pub outbox: Outbox<Email>,
    }

    #[async_trait]
    impl<Email> Sendout<Email> for MockEmailSender<Email>
    where
        Email: Send + Sync,
    {
        async fn send(&self, email: Email) -> Result<(), SendoutError> {
            if let Some(err) = &self.failure_error {
                return Err(err.clone());
            }
            self.outbox
                .lock()
                .map(|mut guard| guard.push(email))
                .expect("unpoisoned mutex");
            Ok(())
        }
    }

    impl<Email> Default for MockEmailSender<Email> {
        fn default() -> Self {
            Self {
                failure_error: None,
                outbox: Arc::new(Mutex::new(vec![])),
            }
        }
    }

    impl<Email> MockEmailSender<Email>
    where
        Email: Clone,
    {
        /// Creates new `MockEmailSender` that succeeds.
        pub fn new() -> Self {
            Self::default()
        }

        /// Creates new `MockEmailSender` that fails with the given error.
        ///
        /// Any attempt to send an email always return the specified error.
        pub fn with_error(error: SendoutError) -> Self {
            Self {
                failure_error: Some(error),
                ..MockEmailSender::default()
            }
        }

        /// Returns the list of "sent" emails.
        pub fn sent_emails(&self) -> Vec<Email> {
            self.outbox
                .lock()
                .map(|guard| guard.clone())
                .expect("to access the outbox")
        }

        /// Returns the number of emails "sent" successfully
        pub fn total_emails_sent(&self) -> usize {
            self.outbox.lock().expect("unpoisoned mutex").len()
        }

    }
}

cfg_test! {
    mod tests {
        use googletest::{gtest, expect_that};
        use googletest::matchers::{eq, err, ok, anything};

        use super::*;

        #[tokio::test]
        #[gtest]
        async fn test_send_email_successfully() {
            let sender = MockEmailSender::new();
            let res = sender.send("hi").await;
            expect_that!(res, ok(anything()));
            expect_that!(sender.total_emails_sent(), eq(1));
            insta::assert_yaml_snapshot!(sender.sent_emails());
        }

        #[tokio::test]
        #[gtest]
        async fn test_send_email_fails() {
            let sender = MockEmailSender::with_error(SendoutError::SendFailed("test error".into()));
            let res = sender.send("hi").await;
            expect_that!(res, err(anything()));
            expect_that!(sender.total_emails_sent(), eq(0));
            insta::assert_yaml_snapshot!(sender.sent_emails());
        }
    }
}
