//! Email service traits
//!
//! Defines the main traits for the services a provider may support
use async_trait::async_trait;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::Error;

/// Trait for sending an email with a provider
#[async_trait]
pub trait EmailService<Email, Response>: Send + Sync
where
    Email: Serialize,
    Response: DeserializeOwned,
{
    /// Send an email
    async fn send_email(&self, email: Email) -> Result<Response, Error>;
}

cfg_test_util! {
    use std::sync::{Arc, Mutex};

    /// A list of sent emails
    type Outbox<Email> = Arc<Mutex<Vec<Email>>>;

    /// Mock sender that records sent emails
    pub struct MockEmailSender<Email> {
        /// The error to return when failure is expected
        pub failure_error: Option<Error>,
        /// Records every email sent so you can assert on them
        pub outbox: Outbox<Email>,
    }

    #[async_trait]
    impl<Email> EmailService<Email, ()> for MockEmailSender<Email>
    where
        Email: Serialize + Send + Sync,
    {
        async fn send_email(&self, email: Email) -> Result<(), Error> {
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
        /// Any attempt to send an email always returns the specified error.
        pub fn with_error(error: Error) -> Self {
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
        async fn send_email_successfully() {
            let sender = MockEmailSender::new();
            let res = sender.send_email("hi").await;
            expect_that!(res, ok(anything()));
            expect_that!(sender.total_emails_sent(), eq(1));
            insta::assert_yaml_snapshot!(sender.sent_emails());
        }

        #[tokio::test]
        #[gtest]
        async fn send_email_fails() {
            let sender = MockEmailSender::with_error(Error::SendFailed("test error".into()));
            let res = sender.send_email("hi").await;
            expect_that!(res, err(anything()));
            expect_that!(sender.total_emails_sent(), eq(0));
            insta::assert_yaml_snapshot!(sender.sent_emails());
        }
    }
}
