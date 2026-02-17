//! Email sent responses data structures
use bytes::Bytes;
use http::Response;
use serde::Deserialize;

use crate::error::Error;

/// What the provider hands back after receiving an email
#[derive(Debug, Clone, Deserialize)]
pub struct EmailDelivery {
    /// Recipient email address
    pub to: String,
    /// Submission timestamp
    pub submitted_at: String,
    /// Unique ID assigned to this message by the provider
    pub message_id: String,
    /// API error code
    pub error_code: u16,
    /// Human readable status message
    pub message: String,
}

impl TryFrom<Response<Bytes>> for EmailDelivery {
    type Error = Error;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(name = "Response::try_from", err(Debug))
    )]
    fn try_from(response: Response<Bytes>) -> Result<Self, Self::Error> {
        serde_json::from_slice(response.body()).map_err(|err| {
            #[cfg(feature = "tracing")]
            tracing::error!(?err);
            Error::SendFailed(err.to_string())
        })
    }
}
#[cfg(test)]
mod tests {
    use googletest::matchers::{anything, eq, err};
    use googletest::{expect_that, gtest};

    use super::*;

    fn make_json(
        to: &str,
        submitted_at: &str,
        message_id: &str,
        error_code: u16,
        message: &str,
    ) -> String {
        format!(
            r#"{{"to": "{to}", "submitted_at": "{submitted_at}", "message_id": "{message_id}", "error_code": {error_code}, "message": "{message}"}}"#
        )
    }

    fn make_json_without_message_id(
        to: &str,
        submitted_at: &str,
        error_code: u16,
        message: &str,
    ) -> String {
        format!(
            r#"{{"to": "{to}", "submitted_at": "{submitted_at}", "error_code": {error_code}, "message": "{message}"}}"#
        )
    }

    #[gtest]
    fn email_response_deserializes_all_fields() {
        let json = make_json(
            "kwame.nkrumah@example.africa",
            "2026-02-08T14:22:31Z",
            "sendout-msg-7f3a9b2c",
            0,
            "Pan-African unity message delivered to Ghana",
        );

        let response: EmailDelivery =
            serde_json::from_str(&json).expect("deserialization to succeed");

        expect_that!(response.to, eq("kwame.nkrumah@example.africa"));
        expect_that!(response.submitted_at, eq("2026-02-08T14:22:31Z"));
        expect_that!(response.message_id, eq("sendout-msg-7f3a9b2c"));
        expect_that!(response.error_code, eq(0));
        expect_that!(
            response.message,
            eq("Pan-African unity message delivered to Ghana")
        );
    }

    #[gtest]
    fn email_response_missing_field_fails() {
        let json = make_json_without_message_id(
            "patrice.lumumba@example.africa",
            "2026-02-08T16:00:00Z",
            0,
            "Congo independence communique incomplete",
        );

        let result: std::result::Result<EmailDelivery, serde_json::Error> =
            serde_json::from_str(&json);

        expect_that!(result, err(anything()));
    }
}
