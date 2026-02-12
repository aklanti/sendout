//! Postmark-specific response types

use bytes::Bytes;
use http::Response;
use serde::Deserialize;

use crate::email::EmailDelivery;
use crate::error::Error;

/// Postmark email response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PostmarkEmailResponse {
    /// Recipient email address
    pub to: String,
    /// Submission timestamp
    pub submitted_at: String,
    /// Postmark message ID
    #[serde(rename = "MessageID")]
    pub message_id: String,
    /// API error code
    pub error_code: u16,
    /// Human-readable response message
    pub message: String,
}

impl TryFrom<Response<Bytes>> for PostmarkEmailResponse {
    type Error = Error;

    fn try_from(response: Response<Bytes>) -> Result<Self, Self::Error> {
        serde_json::from_slice(response.body())
            .map_err(|err| Error::SendFailed(format!("failed to parse response: {err}")))
    }
}

impl From<PostmarkEmailResponse> for EmailDelivery {
    fn from(res: PostmarkEmailResponse) -> Self {
        Self {
            to: res.to,
            submitted_at: res.submitted_at,
            message_id: res.message_id,
            error_code: res.error_code,
            message: res.message,
        }
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
            r#"{{"To": "{to}", "SubmittedAt": "{submitted_at}", "MessageID": "{message_id}", "ErrorCode": {error_code}, "Message": "{message}"}}"#
        )
    }

    fn make_json_without_message_id(
        to: &str,
        submitted_at: &str,
        error_code: u16,
        message: &str,
    ) -> String {
        format!(
            r#"{{"To": "{to}", "SubmittedAt": "{submitted_at}", "ErrorCode": {error_code}, "Message": "{message}"}}"#
        )
    }

    #[gtest]
    fn postmark_response_deserializes_all_fields() {
        let json = make_json(
            "kwame.nkrumah@example.africa",
            "2026-02-08T14:22:31Z",
            "sendout-msg-7f3a9b2c",
            0,
            "Pan-African unity message delivered to Ghana",
        );

        let response: PostmarkEmailResponse =
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
    fn postmark_response_missing_field_fails() {
        let json = make_json_without_message_id(
            "patrice.lumumba@example.africa",
            "2026-02-08T16:00:00Z",
            0,
            "Congo independence communique incomplete",
        );

        let result: std::result::Result<PostmarkEmailResponse, serde_json::Error> =
            serde_json::from_str(&json);

        expect_that!(result, err(anything()));
    }

    #[gtest]
    fn postmark_response_converts_to_email_response() {
        let postmark = PostmarkEmailResponse {
            to: "wangari.maathai@example.africa".to_owned(),
            submitted_at: "2026-02-09T10:00:00Z".to_owned(),
            message_id: "sendout-msg-abc123".to_owned(),
            error_code: 0,
            message: "OK".to_owned(),
        };

        let response: EmailDelivery = postmark.into();

        expect_that!(response.to, eq("wangari.maathai@example.africa"));
        expect_that!(response.submitted_at, eq("2026-02-09T10:00:00Z"));
        expect_that!(response.message_id, eq("sendout-msg-abc123"));
        expect_that!(response.error_code, eq(0));
        expect_that!(response.message, eq("OK"));
    }

    #[gtest]
    fn postmark_response_try_from_http_response() {
        let json = make_json(
            "steve.biko@example.africa",
            "2026-02-09T08:30:00Z",
            "sendout-msg-def456",
            0,
            "Message accepted",
        );

        let http_response = http::Response::builder()
            .status(200)
            .body(bytes::Bytes::from(json))
            .expect("valid response");

        let result = PostmarkEmailResponse::try_from(http_response);
        assert!(result.is_ok());

        let response = result.expect("successful parse");
        expect_that!(response.to, eq("steve.biko@example.africa"));
        expect_that!(response.message_id, eq("sendout-msg-def456"));
    }

    #[gtest]
    fn postmark_response_try_from_invalid_body_fails() {
        let http_response = http::Response::builder()
            .status(200)
            .body(bytes::Bytes::from("not json"))
            .expect("valid response");

        let result = PostmarkEmailResponse::try_from(http_response);
        expect_that!(result, err(anything()));
    }
}
