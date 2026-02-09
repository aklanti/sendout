//! Email data structure

use std::collections::HashMap;

#[cfg(feature = "garde")]
use garde::Validate;
use serde::Serialize;
use serde_with::formats::CommaSeparator;
use serde_with::{StringWithSeparator, serde_as};

/// Request for sending an email
#[serde_as]
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "bon", derive(bon::Builder))]
#[cfg_attr(feature = "garde", derive(Validate))]
pub struct EmailMessage {
    /// The sender email address
    #[cfg_attr(feature = "garde", garde(email))]
    pub r#from: String,
    /// Recipient email address
    #[cfg_attr(feature = "garde", garde(dive))]
    pub to: Recipients,
    /// Email subject
    #[cfg_attr(feature = "garde", garde(skip))]
    pub subject: String,
    /// Plain text email message
    #[cfg_attr(feature = "garde", garde(skip))]
    #[serde(flatten)]
    pub body: Body,
    /// Cc recipient email address
    #[cfg_attr(feature = "garde", garde(dive))]
    pub cc: Option<Recipients>,
    /// Bcc recipient email address
    #[cfg_attr(feature = "garde", garde(dive))]
    pub bcc: Option<Recipients>,
    /// Email tag that allows you to categorize outgoing emails
    /// and get detailed statistics
    #[cfg_attr(feature = "garde", garde(length(graphemes, min = 1)))]
    pub tag: Option<String>,
    /// Reply To override email address
    #[cfg_attr(feature = "garde", garde(dive))]
    pub rely_to: Option<Recipients>,
    /// List of custom headers to include
    #[cfg_attr(feature = "garde", garde(length(min = 1)))]
    pub headers: Option<Vec<Header>>,
    /// Custom metadata key/value pairs
    #[cfg_attr(feature = "garde", garde(length(min = 1)))]
    pub metadata: Option<HashMap<String, String>>,
    /// List of attachments
    #[cfg_attr(feature = "garde", garde(dive))]
    pub attachments: Option<Vec<Attachment>>,
    /// Set message stream ID that's used for sending
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

/// Custom Header
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

/// Email recipients
#[serde_as]
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "garde", derive(Validate))]
#[cfg_attr(feature = "garde", garde(transparent))]
pub struct Recipients(
    #[cfg_attr(feature = "garde", garde(length(min = 1), inner(email)))]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    Vec<String>,
);

/// An attachment to the email
#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "garde", derive(Validate))]
pub struct Attachment {
    /// Name of the attached file
    #[cfg_attr(feature = "garde", garde(skip))]
    pub name: String,
    #[cfg_attr(feature = "garde", garde(skip))]
    /// The content of the attached file
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
    fn test_email_request_serializes_required_fields() {
        let request = EmailMessage {
            r#from: "wangari.maathai@example.africa".to_owned(),
            to: Recipients(vec!["kwame.nkrumah@example.africa".to_owned()]),
            subject: "Green Belt Movement Monthly Update".to_owned(),
            body: Body::Text("We planted 10,000 trees across Kenya this month.".to_owned()),
            cc: None,
            bcc: None,
            tag: None,
            rely_to: None,
            headers: None,
            metadata: None,
            attachments: None,
            message_stream: None,
        };

        let json: Value = serde_json::to_value(&request).expect("serialization to succeed");

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
    fn test_email_request_omits_none_optional_fields() {
        let request = EmailMessage {
            r#from: "thomas.sankara@example.africa".to_owned(),
            to: Recipients(vec!["patrice.lumumba@example.africa".to_owned()]),
            subject: "Self-Sufficiency Progress Report".to_owned(),
            body: Body::Text("Burkina Faso grows stronger through our own efforts.".to_owned()),
            cc: None,
            bcc: None,
            tag: None,
            rely_to: None,
            headers: None,
            metadata: None,
            attachments: None,
            message_stream: None,
        };

        let json: Value = serde_json::to_value(&request).expect("serialization to succeed");

        expect_that!(json.get("cc"), none());
        expect_that!(json.get("bcc"), none());
        expect_that!(json.get("tag"), none());
        expect_that!(json.get("rely_to"), none());
        expect_that!(json.get("headers"), none());
        expect_that!(json.get("metadata"), none());
        expect_that!(json.get("attachments"), none());
        expect_that!(json.get("message_stream"), none());
    }

    #[gtest]
    fn test_email_request_includes_optional_fields_when_present() {
        let mut metadata = HashMap::new();
        metadata.insert("literary_genre".to_owned(), "african-fiction".to_owned());

        let request = EmailMessage {
            r#from: "chimamanda.adichie@example.africa".to_owned(),
            to: Recipients(vec!["yaa.asantewaa@example.africa".to_owned()]),
            subject: "New Novel Draft Ready for Review".to_owned(),
            body: Body::Text("The story of our ancestors deserves to be told.".to_owned()),
            cc: Some(Recipients(vec!["steve.biko@example.africa".to_owned()])),
            bcc: Some(Recipients(vec!["miriam.makeba@example.africa".to_owned()])),
            tag: Some("african-literature".to_owned()),
            rely_to: Some(Recipients(vec!["gbehanzin@example.africa".to_owned()])),
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

        let json: Value = serde_json::to_value(&request).expect("serialization to succeed");
        expect_that!(
            json.get("cc").and_then(|v| v.as_str()),
            some(eq("steve.biko@example.africa"))
        );
        expect_that!(
            json.get("bcc").and_then(|v| v.as_str()),
            some(eq("miriam.makeba@example.africa"))
        );
        expect_that!(
            json.get("tag").and_then(|v| v.as_str()),
            some(eq("african-literature"))
        );
        expect_that!(
            json.get("rely_to").and_then(|v| v.as_str()),
            some(eq("gbehanzin@example.africa"))
        );
        expect_that!(
            json.get("headers")
                .and_then(|v| v.as_array())
                .map(|a| a.len()),
            some(eq(1))
        );
        expect_that!(
            json.get("metadata")
                .and_then(|v| v.get("literary_genre"))
                .and_then(|v| v.as_str()),
            some(eq("african-fiction"))
        );
        expect_that!(
            json.get("attachments")
                .and_then(|v| v.as_array())
                .map(|a| a.len()),
            some(eq(1))
        );
        expect_that!(
            json.get("message_stream").and_then(|v| v.as_str()),
            some(eq("literary-submissions"))
        );
    }

    #[gtest]
    fn test_email_request_body_flattens_correctly() {
        let request = EmailMessage {
            r#from: "patrice.lumumba@example.africa".to_owned(),
            to: Recipients(vec!["wangari.maathai@example.africa".to_owned()]),
            subject: "Unity for Congo's Future".to_owned(),
            body: Body::Text("Together we shall build a sovereign nation.".to_owned()),
            cc: None,
            bcc: None,
            tag: None,
            rely_to: None,
            headers: None,
            metadata: None,
            attachments: None,
            message_stream: None,
        };

        let json: Value = serde_json::to_value(&request).expect("serialization to succeed");

        expect_that!(json.get("body"), none());
        expect_that!(
            json.get("Text").and_then(|v| v.as_str()),
            some(eq("Together we shall build a sovereign nation."))
        );
    }

    #[gtest]
    fn test_body_text_serializes_as_text_body() {
        let body = Body::Text("The Green Belt Movement has planted one million trees.".to_owned());
        let json: Value = serde_json::to_value(&body).expect("serialization to succeed");

        expect_that!(
            json.get("Text").and_then(|v| v.as_str()),
            some(eq("The Green Belt Movement has planted one million trees."))
        );
        expect_that!(json.get("Html"), none());
    }

    #[gtest]
    fn test_body_html_serializes_as_html_body() {
        let body = Body::Html("<h1>Pan-African Unity Conference</h1>".to_owned());
        let json: Value = serde_json::to_value(&body).expect("serialization to succeed");

        expect_that!(
            json.get("Html").and_then(|v| v.as_str()),
            some(eq("<h1>Pan-African Unity Conference</h1>"))
        );
        expect_that!(json.get("Text"), none());
    }

    #[gtest]
    fn test_header_serializes_name_and_value() {
        let header = Header {
            name: "X-Movement-Id".to_owned(),
            value: "green-belt-kenya-1977".to_owned(),
        };
        let json: Value = serde_json::to_value(&header).expect("serialization to succeed");

        expect_that!(
            json.get("name").and_then(|v| v.as_str()),
            some(eq("X-Movement-Id"))
        );
        expect_that!(
            json.get("value").and_then(|v| v.as_str()),
            some(eq("green-belt-kenya-1977"))
        );
    }

    #[gtest]
    fn test_recipients_single_email_serializes() {
        let recipients = Recipients(vec!["steve.biko@example.africa".to_owned()]);
        let json: Value = serde_json::to_value(&recipients).expect("serialization to succeed");

        expect_that!(json.as_str(), some(eq("steve.biko@example.africa")));
    }

    #[gtest]
    fn test_recipients_multiple_emails_comma_separated() {
        let recipients = Recipients(vec![
            "wangari.maathai@example.africa".to_owned(),
            "thomas.sankara@example.africa".to_owned(),
            "miriam.makeba@example.africa".to_owned(),
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
    fn test_attachment_serializes_all_fields() {
        let attachment = Attachment {
            name: "reforestation-report.xlsx".to_owned(),
            content: "UEsDBBQAAAAIAA==".to_owned(),
            content_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                .to_owned(),
        };
        let json: Value = serde_json::to_value(&attachment).expect("serialization to succeed");

        expect_that!(
            json.get("name").and_then(|v| v.as_str()),
            some(eq("reforestation-report.xlsx"))
        );
        expect_that!(
            json.get("content").and_then(|v| v.as_str()),
            some(eq("UEsDBBQAAAAIAA=="))
        );
        expect_that!(
            json.get("content_type").and_then(|v| v.as_str()),
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
        fn test_email_request_valid_from_email() {
            let request = EmailMessage {
                r#from: "wangari.maathai@example.africa".to_owned(),
                to: Recipients(vec!["patrice.lumumba@example.africa".to_owned()]),
                subject: "Environmental Restoration Initiative".to_owned(),
                body: Body::Text(
                    "Every tree we plant is a step toward healing our land.".to_owned(),
                ),
                cc: None,
                bcc: None,
                tag: None,
                rely_to: None,
                headers: None,
                metadata: None,
                attachments: None,
                message_stream: None,
            };

            expect_that!(request.validate(), ok(anything()));
        }

        #[gtest]
        fn test_email_request_invalid_from_email_fails() {
            let request = EmailMessage {
                r#from: "this-is-not-an-email-address".to_owned(),
                to: Recipients(vec!["thomas.sankara@example.africa".to_owned()]),
                subject: "Revolutionary Economic Reforms".to_owned(),
                body: Body::Text("The people of Burkina Faso demand self-reliance.".to_owned()),
                cc: None,
                bcc: None,
                tag: None,
                rely_to: None,
                headers: None,
                metadata: None,
                attachments: None,
                message_stream: None,
            };

            expect_that!(request.validate(), err(anything()));
        }

        #[gtest]
        fn test_email_request_validates_nested_to_recipients() {
            let request = EmailMessage {
                r#from: "chimamanda.adichie@example.africa".to_owned(),
                to: Recipients(vec!["broken-recipient-format".to_owned()]),
                subject: "The Danger of a Single Story".to_owned(),
                body: Body::Text("Our narratives shape how the world sees Africa.".to_owned()),
                cc: None,
                bcc: None,
                tag: None,
                rely_to: None,
                headers: None,
                metadata: None,
                attachments: None,
                message_stream: None,
            };

            expect_that!(request.validate(), err(anything()));
        }

        #[gtest]
        fn test_recipients_valid_single_email() {
            let recipients = Recipients(vec!["patrice.lumumba@example.africa".to_owned()]);
            expect_that!(recipients.validate(), ok(anything()));
        }

        #[gtest]
        fn test_recipients_invalid_single_email_fails() {
            let recipients = Recipients(vec!["completely-invalid".to_owned()]);
            expect_that!(recipients.validate(), err(anything()));
        }
    }

    #[cfg(feature = "bon")]
    mod builder_tests {
        use super::*;

        #[gtest]
        fn test_email_request_builder_with_required_fields() {
            let request = EmailMessage::builder()
                .r#from("patrice.lumumba@example.africa".to_owned())
                .to(Recipients(vec!["kwame.nkrumah@example.africa".to_owned()]))
                .subject("Congo's Path to Sovereignty".to_owned())
                .body(Body::Text(
                    "Independence is not a gift but a right of all peoples.".to_owned(),
                ))
                .build();

            expect_that!(
                request.r#from.as_str(),
                eq("patrice.lumumba@example.africa")
            );
            expect_that!(request.subject.as_str(), eq("Congo's Path to Sovereignty"));
            expect_that!(request.cc, none());
            expect_that!(request.bcc, none());
            expect_that!(request.tag, none());
        }

        #[gtest]
        fn test_email_request_builder_with_all_fields() {
            let mut metadata = HashMap::new();
            metadata.insert("heritage".to_owned(), "ashanti-kingdom".to_owned());

            let request = EmailMessage::builder()
                .r#from("chimamanda.adichie@example.africa".to_owned())
                .to(Recipients(vec!["yaa.asantewaa@example.africa".to_owned()]))
                .subject("Celebrating African Women in Literature".to_owned())
                .body(Body::Html(
                    "<p>Your courage inspires generations of writers.</p>".to_owned(),
                ))
                .cc(Recipients(vec!["steve.biko@example.africa".to_owned()]))
                .bcc(Recipients(vec!["miriam.makeba@example.africa".to_owned()]))
                .tag("african-women-history".to_owned())
                .rely_to(Recipients(vec!["gbehanzin@example.africa".to_owned()]))
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
                request.r#from.as_str(),
                eq("chimamanda.adichie@example.africa")
            );
            expect_that!(
                request.to.0.first().map(|s| s.as_str()),
                some(eq("yaa.asantewaa@example.africa"))
            );
            expect_that!(
                request.subject.as_str(),
                eq("Celebrating African Women in Literature")
            );
            expect_that!(
                request
                    .cc
                    .as_ref()
                    .and_then(|r| r.0.first())
                    .map(|s| s.as_str()),
                some(eq("steve.biko@example.africa"))
            );
            expect_that!(
                request
                    .bcc
                    .as_ref()
                    .and_then(|r| r.0.first())
                    .map(|s| s.as_str()),
                some(eq("miriam.makeba@example.africa"))
            );
            expect_that!(request.tag.as_deref(), some(eq("african-women-history")));
            expect_that!(
                request
                    .rely_to
                    .as_ref()
                    .and_then(|r| r.0.first())
                    .map(|s| s.as_str()),
                some(eq("gbehanzin@example.africa"))
            );
            expect_that!(
                request
                    .headers
                    .as_ref()
                    .and_then(|h| h.first())
                    .map(|h| h.name.as_str()),
                some(eq("X-Literary-Tribute"))
            );
            expect_that!(
                request
                    .metadata
                    .as_ref()
                    .and_then(|m| m.get("heritage"))
                    .map(|s| s.as_str()),
                some(eq("ashanti-kingdom"))
            );
            expect_that!(
                request
                    .attachments
                    .as_ref()
                    .and_then(|a| a.first())
                    .map(|a| a.name.as_str()),
                some(eq("war-of-the-golden-stool.json"))
            );
            expect_that!(
                request.message_stream.as_deref(),
                some(eq("african-heritage"))
            );
        }
    }
}
