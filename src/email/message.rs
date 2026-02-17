//! The message your build and hand off to a provider
use std::collections::HashMap;

#[cfg(feature = "garde")]
use garde::Validate;
use serde::Serialize;
use serde_with::formats::CommaSeparator;
use serde_with::{StringWithSeparator, serde_as};

/// An email to be sent
#[serde_as]
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
#[cfg_attr(feature = "garde", derive(Validate))]
pub struct EmailMessage {
    /// The sender email address
    #[cfg_attr(feature = "garde", garde(email))]
    pub from: String,
    /// Recipient email address
    #[cfg_attr(feature = "garde", garde(dive))]
    pub to: Recipients,
    /// Email subject
    #[cfg_attr(feature = "garde", garde(skip))]
    pub subject: String,
    /// The email body
    #[cfg_attr(feature = "garde", garde(skip))]
    #[serde(flatten)]
    pub body: Body,
    /// Cc recipients
    #[cfg_attr(feature = "garde", garde(dive))]
    pub cc: Option<Recipients>,
    /// Bcc recipients
    #[cfg_attr(feature = "garde", garde(dive))]
    pub bcc: Option<Recipients>,
    /// Email tag that allows you to categorize outgoing emails
    /// and get detailed statistics
    #[cfg_attr(feature = "garde", garde(length(graphemes, min = 1)))]
    pub tag: Option<String>,
    /// Reply-To address override
    #[cfg_attr(feature = "garde", garde(dive))]
    pub reply_to: Option<Recipients>,
    /// List of custom headers to include
    #[cfg_attr(feature = "garde", garde(length(min = 1)))]
    pub headers: Option<Vec<Header>>,
    /// Custom metadata key/value pairs
    #[cfg_attr(feature = "garde", garde(length(min = 1)))]
    pub metadata: Option<HashMap<String, String>>,
    /// List of attachments
    #[cfg_attr(feature = "garde", garde(dive))]
    pub attachments: Option<Vec<Attachment>>,
    /// Message stream to send through
    #[cfg_attr(feature = "garde", garde(length(graphemes, min = 1)))]
    pub message_stream: Option<String>,
}

/// Email message body
#[derive(Debug, Clone, Serialize)]
pub enum Body {
    /// Plain text email message
    Text(String),
    /// HTML email message
    Html(String),
}

/// A custom header to attach to the email
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "garde", derive(Validate))]
pub struct Header {
    /// Name of the header
    #[cfg_attr(feature = "garde", garde(length(graphemes, min = 1)))]
    pub name: String,
    /// Value of the header
    #[cfg_attr(feature = "garde", garde(length(graphemes, min = 1)))]
    pub value: String,
}

/// A list of recipients serialized as comma separated string
#[serde_as]
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "garde", derive(Validate))]
#[cfg_attr(feature = "garde", garde(transparent))]
pub struct Recipients(
    #[cfg_attr(feature = "garde", garde(length(min = 1), inner(email)))]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    Vec<String>,
);

impl Recipients {
    /// Consumes self and returns the inner list of email addresses
    pub fn into_inner(self) -> Vec<String> {
        self.0
    }
}

impl From<Vec<String>> for Recipients {
    fn from(emails: Vec<String>) -> Self {
        Self(emails)
    }
}

impl From<Vec<&str>> for Recipients {
    fn from(emails: Vec<&str>) -> Self {
        Self(emails.iter().map(ToString::to_string).collect())
    }
}

/// An attachment to the email
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "garde", derive(Validate))]
pub struct Attachment {
    /// Name of the attached file
    #[cfg_attr(feature = "garde", garde(skip))]
    pub name: String,
    #[cfg_attr(feature = "garde", garde(skip))]
    /// Base64-encoded file content
    pub content: String,
    /// The content type of the attached file
    #[cfg_attr(feature = "garde", garde(skip))]
    pub content_type: String,
}

#[cfg(test)]
mod tests {
    use googletest::matchers::{eq, none, some};
    use googletest::{expect_that, gtest};
    use serde_json::Value;

    use super::*;

    #[gtest]
    fn email_email_message_serializes_required_fields() {
        let email_message = EmailMessage {
            from: "wangari.maathai@example.africa".to_owned(),
            to: vec!["kwame.nkrumah@example.africa"].into(),
            subject: "Green Belt Movement Monthly Update".to_owned(),
            body: Body::Text("We planted 10,000 trees across Kenya this month.".to_owned()),
            cc: None,
            bcc: None,
            tag: None,
            reply_to: None,
            headers: None,
            metadata: None,
            attachments: None,
            message_stream: None,
        };

        let json: Value = serde_json::to_value(&email_message).expect("serialization to succeed");

        expect_that!(
            json.get("from").and_then(|v| v.as_str()),
            some(eq("wangari.maathai@example.africa"))
        );
        expect_that!(
            json.get("to").and_then(|v| v.as_str()),
            some(eq("kwame.nkrumah@example.africa"))
        );
        expect_that!(
            json.get("subject").and_then(|v| v.as_str()),
            some(eq("Green Belt Movement Monthly Update"))
        );
        expect_that!(
            json.get("Text").and_then(|v| v.as_str()),
            some(eq("We planted 10,000 trees across Kenya this month."))
        );
    }

    #[gtest]
    fn email_email_message_omits_none_optional_fields() {
        let email_message = EmailMessage {
            from: "thomas.sankara@example.africa".to_owned(),
            to: vec!["patrice.lumumba@example.africa"].into(),
            subject: "Self-Sufficiency Progress Report".to_owned(),
            body: Body::Text("Burkina Faso grows stronger through our own efforts.".to_owned()),
            cc: None,
            bcc: None,
            tag: None,
            reply_to: None,
            headers: None,
            metadata: None,
            attachments: None,
            message_stream: None,
        };

        let json: Value = serde_json::to_value(&email_message).expect("serialization to succeed");

        expect_that!(json.get("cc"), none());
        expect_that!(json.get("bcc"), none());
        expect_that!(json.get("tag"), none());
        expect_that!(json.get("reply_to"), none());
        expect_that!(json.get("headers"), none());
        expect_that!(json.get("metadata"), none());
        expect_that!(json.get("attachments"), none());
        expect_that!(json.get("message_stream"), none());
    }

    #[gtest]
    fn email_email_message_includes_optional_fields_when_present() {
        let mut metadata = HashMap::new();
        metadata.insert("literary_genre".to_owned(), "african-fiction".to_owned());

        let email_message = EmailMessage {
            from: "chimamanda.adichie@example.africa".to_owned(),
            to: vec!["yaa.asantewaa@example.africa"].into(),
            subject: "New Novel Draft Ready for Review".to_owned(),
            body: Body::Text("The story of our ancestors deserves to be told.".to_owned()),
            cc: Some(vec!["steve.biko@example.africa"].into()),
            bcc: Some(vec!["miriam.makeba@example.africa"].into()),
            tag: Some("african-literature".to_owned()),
            reply_to: Some(vec!["gbehanzin@example.africa"].into()),
            headers: Some(vec![Header {
                name: "X-Manuscript-Id".to_owned(),
                value: "half-of-a-yellow-sun-draft".to_owned(),
            }]),
            metadata: Some(metadata),
            attachments: Some(vec![Attachment {
                name: "manuscript-chapter-one.pdf".to_owned(),
                content: "JVBERi0xLjQKJcfs".to_owned(),
                content_type: "application/pdf".to_owned(),
            }]),
            message_stream: Some("literary-submissions".to_owned()),
        };

        let json: Value = serde_json::to_value(&email_message).expect("serialization to succeed");
        expect_that!(
            json.get("cc").and_then(|c| c.as_str()),
            some(eq("steve.biko@example.africa"))
        );
        expect_that!(
            json.get("bcc").and_then(|b| b.as_str()),
            some(eq("miriam.makeba@example.africa"))
        );
        expect_that!(
            json.get("tag").and_then(|t| t.as_str()),
            some(eq("african-literature"))
        );
        expect_that!(
            json.get("reply_to").and_then(|r| r.as_str()),
            some(eq("gbehanzin@example.africa"))
        );
        expect_that!(
            json.get("headers")
                .and_then(|h| h.as_array())
                .map(|h| h.len()),
            some(eq(1))
        );
        expect_that!(
            json.get("metadata")
                .and_then(|m| m.get("literary_genre"))
                .and_then(|m| m.as_str()),
            some(eq("african-fiction"))
        );
        expect_that!(
            json.get("attachments")
                .and_then(|a| a.as_array())
                .map(|a| a.len()),
            some(eq(1))
        );
        expect_that!(
            json.get("message_stream").and_then(|m| m.as_str()),
            some(eq("literary-submissions"))
        );
    }

    #[gtest]
    fn body_serializes_as_expected() {
        let text_body =
            Body::Text("The Green Belt Movement has planted one million trees.".to_owned());
        let text_json: Value = serde_json::to_value(&text_body).expect("serialization to succeed");
        expect_that!(
            text_json.get("Text").and_then(|t| t.as_str()),
            some(eq("The Green Belt Movement has planted one million trees."))
        );
        expect_that!(text_json.get("Html"), none());

        let html_body = Body::Html("<h1>Pan-African Unity Conference</h1>".to_owned());
        let html_json: Value = serde_json::to_value(&html_body).expect("serialization to succeed");
        expect_that!(
            html_json.get("Html").and_then(|h| h.as_str()),
            some(eq("<h1>Pan-African Unity Conference</h1>"))
        );
        expect_that!(html_json.get("Text"), none());
    }

    #[gtest]
    fn header_serializes_name_and_value() {
        let header = Header {
            name: "X-Movement-Id".to_owned(),
            value: "green-belt-kenya-1977".to_owned(),
        };
        let json: Value = serde_json::to_value(&header).expect("serialization to succeed");

        expect_that!(
            json.get("name").and_then(|n| n.as_str()),
            some(eq("X-Movement-Id"))
        );
        expect_that!(
            json.get("value").and_then(|v| v.as_str()),
            some(eq("green-belt-kenya-1977"))
        );
    }

    #[gtest]
    fn recipients_single_email_serializes() {
        let recipients = Recipients::from(vec!["steve.biko@example.africa"]);
        let json: Value = serde_json::to_value(&recipients).expect("serialization to succeed");

        expect_that!(json.as_str(), some(eq("steve.biko@example.africa")));
    }

    #[gtest]
    fn recipients_multiple_emails_comma_separated() {
        let recipients = Recipients::from(vec![
            "wangari.maathai@example.africa",
            "thomas.sankara@example.africa",
            "miriam.makeba@example.africa",
        ]);
        let json: Value = serde_json::to_value(&recipients).expect("serialization to succeed");

        expect_that!(
            json.as_str(),
            some(eq(
                "wangari.maathai@example.africa,thomas.sankara@example.africa,miriam.makeba@example.africa"
            ))
        );
    }

    #[gtest]
    fn attachment_serializes_all_fields() {
        let attachment = Attachment {
            name: "reforestation-report.xlsx".to_owned(),
            content: "UEsDBBQAAAAIAA==".to_owned(),
            content_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                .to_owned(),
        };
        let json: Value = serde_json::to_value(&attachment).expect("serialization to succeed");

        expect_that!(
            json.get("name").and_then(|n| n.as_str()),
            some(eq("reforestation-report.xlsx"))
        );
        expect_that!(
            json.get("content").and_then(|c| c.as_str()),
            some(eq("UEsDBBQAAAAIAA=="))
        );
        expect_that!(
            json.get("content_type").and_then(|c| c.as_str()),
            some(eq(
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            ))
        );
    }

    #[cfg(feature = "garde")]
    mod validation_tests {
        use garde::Validate;
        use googletest::matchers::{anything, err, ok};

        use super::*;

        #[gtest]
        fn email_email_message_valid_from_email() {
            let email_message = EmailMessage {
                from: "wangari.maathai@example.africa".to_owned(),
                to: vec!["patrice.lumumba@example.africa"].into(),
                subject: "Environmental Restoration Initiative".to_owned(),
                body: Body::Text(
                    "Every tree we plant is a step toward healing our land.".to_owned(),
                ),
                cc: None,
                bcc: None,
                tag: None,
                reply_to: None,
                headers: None,
                metadata: None,
                attachments: None,
                message_stream: None,
            };

            expect_that!(email_message.validate(), ok(anything()));
        }

        #[gtest]
        fn email_email_message_invalid_from_email_fails() {
            let email_message = EmailMessage {
                from: "this-is-not-an-email-address".to_owned(),
                to: vec!["thomas.sankara@example.africa"].into(),
                subject: "Revolutionary Economic Reforms".to_owned(),
                body: Body::Text("The people of Burkina Faso demand self-reliance.".to_owned()),
                cc: None,
                bcc: None,
                tag: None,
                reply_to: None,
                headers: None,
                metadata: None,
                attachments: None,
                message_stream: None,
            };

            expect_that!(email_message.validate(), err(anything()));
        }

        #[gtest]
        fn email_email_message_validates_nested_to_recipients() {
            let email_message = EmailMessage {
                from: "chimamanda.adichie@example.africa".to_owned(),
                to: vec!["broken-recipient-format"].into(),
                subject: "The Danger of a Single Story".to_owned(),
                body: Body::Text("Our narratives shape how the world sees Africa.".to_owned()),
                cc: None,
                bcc: None,
                tag: None,
                reply_to: None,
                headers: None,
                metadata: None,
                attachments: None,
                message_stream: None,
            };

            expect_that!(email_message.validate(), err(anything()));
        }

        #[gtest]
        fn recipients_validation() {
            let valid = Recipients::from(vec!["patrice.lumumba@example.africa"]);
            expect_that!(valid.validate(), ok(anything()));

            let invalid = Recipients::from(vec!["completely-invalid"]);
            expect_that!(invalid.validate(), err(anything()));
        }
    }

    #[cfg(feature = "bon")]
    mod builder_tests {
        use super::*;

        #[gtest]
        fn email_email_message_builder_with_required_fields() {
            let email_message = EmailMessage::builder()
                .from("patrice.lumumba@example.africa".to_owned())
                .to(vec!["kwame.nkrumah@example.africa"].into())
                .subject("Congo's Path to Sovereignty".to_owned())
                .body(Body::Text(
                    "Independence is not a gift but a right of all peoples.".to_owned(),
                ))
                .build();

            expect_that!(
                email_message.from.as_str(),
                eq("patrice.lumumba@example.africa")
            );
            expect_that!(
                email_message.subject.as_str(),
                eq("Congo's Path to Sovereignty")
            );
            expect_that!(email_message.cc, none());
            expect_that!(email_message.bcc, none());
            expect_that!(email_message.tag, none());
        }

        #[gtest]
        fn email_email_message_builder_with_all_fields() {
            let mut metadata = HashMap::new();
            metadata.insert("heritage".to_owned(), "ashanti-kingdom".to_owned());

            let email_message = EmailMessage::builder()
                .from("chimamanda.adichie@example.africa".to_owned())
                .to(vec!["yaa.asantewaa@example.africa"].into())
                .subject("Celebrating African Women in Literature".to_owned())
                .body(Body::Html(
                    "<p>Your courage inspires generations of writers.</p>".to_owned(),
                ))
                .cc(vec!["steve.biko@example.africa"].into())
                .bcc(vec!["miriam.makeba@example.africa"].into())
                .tag("african-women-history".to_owned())
                .reply_to(vec!["gbehanzin@example.africa"].into())
                .headers(vec![Header {
                    name: "X-Literary-Tribute".to_owned(),
                    value: "queen-mother-yaa-asantewaa".to_owned(),
                }])
                .metadata(metadata)
                .attachments(vec![Attachment {
                    name: "war-of-the-golden-stool.json".to_owned(),
                    content: "eyJyZXNpc3RhbmNlIjogIjE5MDAifQ==".to_owned(),
                    content_type: "application/json".to_owned(),
                }])
                .message_stream("african-heritage".to_owned())
                .build();

            expect_that!(
                email_message.from.as_str(),
                eq("chimamanda.adichie@example.africa")
            );
            expect_that!(
                email_message.to.0.first().map(|t| t.as_str()),
                some(eq("yaa.asantewaa@example.africa"))
            );
            expect_that!(
                email_message.subject.as_str(),
                eq("Celebrating African Women in Literature")
            );
            expect_that!(
                email_message
                    .cc
                    .as_ref()
                    .and_then(|c| c.0.first())
                    .map(|c| c.as_str()),
                some(eq("steve.biko@example.africa"))
            );
            expect_that!(
                email_message
                    .bcc
                    .as_ref()
                    .and_then(|b| b.0.first())
                    .map(|b| b.as_str()),
                some(eq("miriam.makeba@example.africa"))
            );
            expect_that!(
                email_message.tag.as_deref(),
                some(eq("african-women-history"))
            );
            expect_that!(
                email_message
                    .reply_to
                    .as_ref()
                    .and_then(|r| r.0.first())
                    .map(|r| r.as_str()),
                some(eq("gbehanzin@example.africa"))
            );
            expect_that!(
                email_message
                    .headers
                    .as_ref()
                    .and_then(|h| h.first())
                    .map(|h| h.name.as_str()),
                some(eq("X-Literary-Tribute"))
            );
            expect_that!(
                email_message
                    .metadata
                    .as_ref()
                    .and_then(|m| m.get("heritage"))
                    .map(|m| m.as_str()),
                some(eq("ashanti-kingdom"))
            );
            expect_that!(
                email_message
                    .attachments
                    .as_ref()
                    .and_then(|a| a.first())
                    .map(|a| a.name.as_str()),
                some(eq("war-of-the-golden-stool.json"))
            );
            expect_that!(
                email_message.message_stream.as_deref(),
                some(eq("african-heritage"))
            );
        }
    }
}
