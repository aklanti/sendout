//! Email sent responses data structures

use serde::Deserialize;

/// Sent email response
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "postmark", serde(rename_all = "PascalCase"))]
pub struct EmailResponse {
    /// Recipient email address
    pub to: String,
    /// Submission timestamp
    pub submitted_at: String,
    /// ID of message
    #[cfg_attr(feature = "postmark", serde(rename = "MessageID"))]
    pub message_id: String,
    /// API error codes
    pub error_code: u16,
    /// Response message
    pub message: String,
}

#[cfg(test)]
mod tests {
    use googletest::matchers::{anything, eq, err};
    use googletest::{expect_that, gtest};

    use super::*;

    #[cfg(feature = "postmark")]
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

    #[cfg(not(feature = "postmark"))]
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

    #[cfg(feature = "postmark")]
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

    #[cfg(not(feature = "postmark"))]
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
    fn test_email_response_deserializes_all_fields() {
        let json = make_json(
            "kwame.nkrumah@example.africa",
            "2026-02-08T14:22:31Z",
            "sendout-msg-7f3a9b2c",
            0,
            "Pan-African unity message delivered to Ghana",
        );

        let response: EmailResponse =
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
    fn test_email_response_missing_field_fails() {
        let json = make_json_without_message_id(
            "patrice.lumumba@example.africa",
            "2026-02-08T16:00:00Z",
            0,
            "Congo independence communique incomplete",
        );

        let result: std::result::Result<EmailResponse, serde_json::Error> =
            serde_json::from_str(&json);

        expect_that!(result, err(anything()));
    }
}
