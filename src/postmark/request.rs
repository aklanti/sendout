//! Postmark-specific request types
use std::collections::HashMap;

#[cfg(feature = "garde")]
use garde::Validate;
use http::Method;
use serde::Serialize;
use serde_with::formats::CommaSeparator;
use serde_with::{StringWithSeparator, serde_as};

use crate::api::ApiRequest;
use crate::email::{Attachment, Body, EmailMessage, Header, Recipients};

/// Postmark email request
#[serde_as]
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "garde", derive(Validate))]
pub struct PostmarkEmailRequest {
    /// The sender email address
    #[cfg_attr(feature = "garde", garde(email))]
    pub from: String,
    /// Recipient email addresses
    #[cfg_attr(feature = "garde", garde(length(min = 1, max = 50), inner(email)))]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    pub to: Vec<String>,
    /// Email subject
    #[cfg_attr(feature = "garde", garde(skip))]
    pub subject: String,
    /// Email body
    #[cfg_attr(feature = "garde", garde(skip))]
    #[serde(flatten)]
    pub body: PostmarkBody,
    /// Cc recipient email addresses
    #[cfg_attr(feature = "garde", garde(length(max = 50), inner(inner(email))))]
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub cc: Option<Vec<String>>,
    /// Bcc recipient email addresses
    #[cfg_attr(feature = "garde", garde(length(max = 50), inner(inner(email))))]
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub bcc: Option<Vec<String>>,
    /// Email tag for categorization (max 1000 chars for Postmark)
    #[cfg_attr(feature = "garde", garde(length(max = 1000)))]
    pub tag: Option<String>,
    /// Reply-To override
    #[cfg_attr(feature = "garde", garde(inner(inner(email))))]
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, String>>")]
    pub reply_to: Option<Vec<String>>,
    /// Custom headers
    #[cfg_attr(feature = "garde", garde(skip))]
    pub headers: Option<Vec<PostmarkHeader>>,
    /// Custom metadata key/value pairs
    #[cfg_attr(feature = "garde", garde(skip))]
    pub metadata: Option<HashMap<String, String>>,
    /// File attachments
    #[cfg_attr(feature = "garde", garde(skip))]
    pub attachments: Option<Vec<PostmarkAttachment>>,
    /// Message stream ID
    #[cfg_attr(feature = "garde", garde(skip))]
    pub message_stream: Option<String>,
}

/// Postmark email body
#[derive(Debug, Clone, Serialize)]
pub enum PostmarkBody {
    /// Plain text email body
    TextBody(String),
    /// HTML email body
    HtmlBody(String),
}

/// Postmark custom header
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PostmarkHeader {
    /// Header name
    pub name: String,
    /// Header value
    pub value: String,
}

/// Postmark attachment
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PostmarkAttachment {
    /// File name
    pub name: String,
    /// Base64-encoded content
    pub content: String,
    /// MIME content type
    pub content_type: String,
}

impl ApiRequest for PostmarkEmailRequest {
    const METHOD: Method = Method::POST;
    const ENDPOINT: &'static str = "/email";
}

impl From<Body> for PostmarkBody {
    fn from(body: Body) -> Self {
        match body {
            Body::Text(text) => PostmarkBody::TextBody(text),
            Body::Html(html) => PostmarkBody::HtmlBody(html),
        }
    }
}

impl From<Header> for PostmarkHeader {
    fn from(header: Header) -> Self {
        Self {
            name: header.name,
            value: header.value,
        }
    }
}

impl From<Attachment> for PostmarkAttachment {
    fn from(attachment: Attachment) -> Self {
        Self {
            name: attachment.name,
            content: attachment.content,
            content_type: attachment.content_type,
        }
    }
}

impl From<EmailMessage> for PostmarkEmailRequest {
    fn from(email: EmailMessage) -> Self {
        Self {
            from: email.from,
            to: email.to.into_inner(),
            subject: email.subject,
            body: email.body.into(),
            cc: email.cc.map(Recipients::into_inner),
            bcc: email.bcc.map(Recipients::into_inner),
            tag: email.tag,
            reply_to: email.reply_to.map(Recipients::into_inner),
            headers: email
                .headers
                .map(|header| header.into_iter().map(Into::into).collect()),
            metadata: email.metadata,
            attachments: email
                .attachments
                .map(|atts| atts.into_iter().map(Into::into).collect()),
            message_stream: email.message_stream,
        }
    }
}

#[cfg(test)]
mod tests {
    use googletest::matchers::{eq, none, some};
    use googletest::{expect_that, gtest};
    use http::Method;
    use serde_json::Value;

    use crate::api::ApiRequest;
    use crate::email::{Attachment, Body, EmailMessage, Header, Recipients};

    use super::*;

    /// Create a minimal email data with given body variant.
    fn minimal_email(body: Body) -> EmailMessage {
        EmailMessage {
            from: "wangari.maathai@example.africa".to_owned(),
            to: vec!["kwame.nkrumah@example.africa"].into(),
            subject: "Green Belt Movement Monthly Update".to_owned(),
            body,
            cc: None,
            bcc: None,
            tag: None,
            reply_to: None,
            headers: None,
            metadata: None,
            attachments: None,
            message_stream: None,
        }
    }

    #[gtest]
    fn api_request_method_is_post() {
        expect_that!(PostmarkEmailRequest::METHOD, eq(Method::POST));
    }

    #[gtest]
    fn api_request_endpoint_is_email() {
        expect_that!(PostmarkEmailRequest::ENDPOINT, eq("/email"));
    }

    #[gtest]
    fn from_email_request_maps_required_fields() {
        let email = minimal_email(Body::Text("Hello".to_owned()));
        let postmark: PostmarkEmailRequest = email.into();

        expect_that!(postmark.from.as_str(), eq("wangari.maathai@example.africa"));
        expect_that!(
            postmark.to.first().map(|r| r.as_str()),
            some(eq("kwame.nkrumah@example.africa"))
        );
        expect_that!(
            postmark.subject.as_str(),
            eq("Green Belt Movement Monthly Update")
        );
    }

    #[gtest]
    fn from_email_request_converts_text_body() {
        let email = minimal_email(Body::Text("plain text".to_owned()));
        let postmark: PostmarkEmailRequest = email.into();
        assert!(matches!(postmark.body, PostmarkBody::TextBody(ref body) if body == "plain text"));
    }

    #[gtest]
    fn from_email_request_converts_html_body() {
        let email = minimal_email(Body::Html("<p>html</p>".to_owned()));
        let postmark: PostmarkEmailRequest = email.into();
        assert!(matches!(postmark.body, PostmarkBody::HtmlBody(ref body) if body == "<p>html</p>"));
    }

    #[gtest]
    fn from_email_request_maps_optional_fields() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_owned(), "value".to_owned());

        let email = EmailMessage {
            from: "chimamanda.adichie@example.africa".to_owned(),
            to: vec!["yaa.asantewaa@example.africa"].into(),
            subject: "Subject".to_owned(),
            body: Body::Text("Body".to_owned()),
            cc: Some(Recipients::from(vec![
                "steve.biko@example.africa".to_owned(),
            ])),
            bcc: Some(vec!["miriam.makeba@example.africa"].into()),
            tag: Some("tag-value".to_owned()),
            reply_to: Some(Recipients::from(vec![
                "gbehanzin@example.africa".to_owned(),
            ])),
            headers: Some(vec![Header {
                name: "X-Custom".to_owned(),
                value: "custom-value".to_owned(),
            }]),
            metadata: Some(metadata),
            attachments: Some(vec![Attachment {
                name: "file.pdf".to_owned(),
                content: "base64data".to_owned(),
                content_type: "application/pdf".to_owned(),
            }]),
            message_stream: Some("outbound".to_owned()),
        };

        let postmark: PostmarkEmailRequest = email.into();

        expect_that!(
            postmark
                .cc
                .as_ref()
                .and_then(|v| v.first())
                .map(|s| s.as_str()),
            some(eq("steve.biko@example.africa"))
        );
        expect_that!(
            postmark
                .bcc
                .as_ref()
                .and_then(|v| v.first())
                .map(|s| s.as_str()),
            some(eq("miriam.makeba@example.africa"))
        );
        expect_that!(postmark.tag.as_deref(), some(eq("tag-value")));
        expect_that!(
            postmark
                .reply_to
                .as_ref()
                .and_then(|v| v.first())
                .map(|s| s.as_str()),
            some(eq("gbehanzin@example.africa"))
        );
        expect_that!(
            postmark
                .headers
                .as_ref()
                .and_then(|h| h.first())
                .map(|h| h.name.as_str()),
            some(eq("X-Custom"))
        );
        expect_that!(
            postmark
                .metadata
                .as_ref()
                .and_then(|m| m.get("key"))
                .map(|s| s.as_str()),
            some(eq("value"))
        );
        expect_that!(
            postmark
                .attachments
                .as_ref()
                .and_then(|a| a.first())
                .map(|a| a.name.as_str()),
            some(eq("file.pdf"))
        );
        expect_that!(postmark.message_stream.as_deref(), some(eq("outbound")));
    }

    #[gtest]
    fn pascal_case_serialization_required_fields() {
        let email = minimal_email(Body::Text(
            "We planted 10,000 trees across Kenya this month.".to_owned(),
        ));
        let postmark: PostmarkEmailRequest = email.into();

        let json: Value = serde_json::to_value(&postmark).expect("serialization to succeed");

        expect_that!(
            json.get("From").and_then(|v| v.as_str()),
            some(eq("wangari.maathai@example.africa"))
        );
        expect_that!(
            json.get("To").and_then(|v| v.as_str()),
            some(eq("kwame.nkrumah@example.africa"))
        );
        expect_that!(
            json.get("Subject").and_then(|v| v.as_str()),
            some(eq("Green Belt Movement Monthly Update"))
        );
        expect_that!(
            json.get("TextBody").and_then(|v| v.as_str()),
            some(eq("We planted 10,000 trees across Kenya this month."))
        );
    }

    #[gtest]
    fn pascal_case_serialization_optional_fields() {
        let email = EmailMessage {
            from: "kwame.nkrumah@example.africa".to_owned(),
            to: vec!["yaa.asantewaa@example.africa"].into(),
            subject: "Pan-African Congress Invitation".to_owned(),
            body: Body::Text("Africa must unite for true independence.".to_owned()),
            cc: Some(vec!["steve.biko@example.africa"].into()),
            bcc: None,
            tag: Some("pan-african-congress".to_owned()),
            reply_to: None,
            headers: None,
            metadata: None,
            attachments: None,
            message_stream: Some("independence-movement".to_owned()),
        };

        let postmark: PostmarkEmailRequest = email.into();
        let json: Value = serde_json::to_value(&postmark).expect("serialization to succeed");

        expect_that!(
            json.get("From").and_then(|v| v.as_str()),
            some(eq("kwame.nkrumah@example.africa"))
        );
        expect_that!(
            json.get("To").and_then(|v| v.as_str()),
            some(eq("yaa.asantewaa@example.africa"))
        );
        expect_that!(
            json.get("Subject").and_then(|v| v.as_str()),
            some(eq("Pan-African Congress Invitation"))
        );
        expect_that!(
            json.get("Cc").and_then(|v| v.as_str()),
            some(eq("steve.biko@example.africa"))
        );
        expect_that!(
            json.get("Tag").and_then(|v| v.as_str()),
            some(eq("pan-african-congress"))
        );
        expect_that!(
            json.get("MessageStream").and_then(|v| v.as_str()),
            some(eq("independence-movement"))
        );
    }

    #[gtest]
    fn pascal_case_omits_none_optional_fields() {
        let email = minimal_email(Body::Text("Body".to_owned()));
        let postmark: PostmarkEmailRequest = email.into();

        let json: Value = serde_json::to_value(&postmark).expect("serialization to succeed");

        expect_that!(json.get("Cc"), none());
        expect_that!(json.get("Bcc"), none());
        expect_that!(json.get("Tag"), none());
        expect_that!(json.get("ReplyTo"), none());
        expect_that!(json.get("Headers"), none());
        expect_that!(json.get("Metadata"), none());
        expect_that!(json.get("Attachments"), none());
        expect_that!(json.get("MessageStream"), none());
    }

    #[gtest]
    fn body_flattens_body_key_for_both_variants() {
        let email = minimal_email(Body::Text(
            "Together we shall build a sovereign nation.".to_owned(),
        ));
        let postmark: PostmarkEmailRequest = email.into();
        let json: Value = serde_json::to_value(&postmark).expect("serialization to succeed");
        expect_that!(json.get("body"), none());
        expect_that!(json.get("Body"), none());
        expect_that!(
            json.get("TextBody").and_then(|v| v.as_str()),
            some(eq("Together we shall build a sovereign nation."))
        );
        expect_that!(json.get("HtmlBody"), none());

        let email = minimal_email(Body::Html(
            "<h1>Pan-African Unity Conference</h1>".to_owned(),
        ));
        let postmark: PostmarkEmailRequest = email.into();
        let json: Value = serde_json::to_value(&postmark).expect("serialization to succeed");
        expect_that!(
            json.get("HtmlBody").and_then(|v| v.as_str()),
            some(eq("<h1>Pan-African Unity Conference</h1>"))
        );
        expect_that!(json.get("TextBody"), none());
    }

    #[gtest]
    fn header_serializes_pascal_case() {
        let header = PostmarkHeader {
            name: "X-Movement-Id".to_owned(),
            value: "green-belt-kenya-1977".to_owned(),
        };
        let json: Value = serde_json::to_value(&header).expect("serialization to succeed");

        expect_that!(
            json.get("Name").and_then(|v| v.as_str()),
            some(eq("X-Movement-Id"))
        );
        expect_that!(
            json.get("Value").and_then(|v| v.as_str()),
            some(eq("green-belt-kenya-1977"))
        );
    }

    #[gtest]
    fn attachment_serializes_pascal_case() {
        let attachment = PostmarkAttachment {
            name: "reforestation-report.xlsx".to_owned(),
            content: "UEsDBBQAAAAIAA==".to_owned(),
            content_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                .to_owned(),
        };
        let json: Value = serde_json::to_value(&attachment).expect("serialization to succeed");

        expect_that!(
            json.get("Name").and_then(|v| v.as_str()),
            some(eq("reforestation-report.xlsx"))
        );
        expect_that!(
            json.get("Content").and_then(|v| v.as_str()),
            some(eq("UEsDBBQAAAAIAA=="))
        );
        expect_that!(
            json.get("ContentType").and_then(|v| v.as_str()),
            some(eq(
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            ))
        );
    }

    #[gtest]
    fn multiple_recipients_comma_separated() {
        let postmark = PostmarkEmailRequest {
            from: "sender@example.africa".to_owned(),
            to: vec![
                "wangari.maathai@example.africa".to_owned(),
                "thomas.sankara@example.africa".to_owned(),
                "miriam.makeba@example.africa".to_owned(),
            ],
            subject: "Multi-recipient".to_owned(),
            body: PostmarkBody::TextBody("Hello all".to_owned()),
            cc: None,
            bcc: None,
            tag: None,
            reply_to: None,
            headers: None,
            metadata: None,
            attachments: None,
            message_stream: None,
        };

        let json: Value = serde_json::to_value(&postmark).expect("serialization to succeed");

        expect_that!(
            json.get("To").and_then(|v| v.as_str()),
            some(eq(
                "wangari.maathai@example.africa,thomas.sankara@example.africa,miriam.makeba@example.africa"
            ))
        );
    }

    #[gtest]
    fn full_request_with_all_fields_serialization() {
        let mut metadata = HashMap::new();
        metadata.insert("literary_genre".to_owned(), "african-fiction".to_owned());

        let postmark = PostmarkEmailRequest {
            from: "chimamanda.adichie@example.africa".to_owned(),
            to: vec!["yaa.asantewaa@example.africa".to_owned()],
            subject: "New Novel Draft Ready for Review".to_owned(),
            body: PostmarkBody::TextBody(
                "The story of our ancestors deserves to be told.".to_owned(),
            ),
            cc: Some(vec!["steve.biko@example.africa".to_owned()]),
            bcc: Some(vec!["miriam.makeba@example.africa".to_owned()]),
            tag: Some("african-literature".to_owned()),
            reply_to: Some(vec!["gbehanzin@example.africa".to_owned()]),
            headers: Some(vec![PostmarkHeader {
                name: "X-Manuscript-Id".to_owned(),
                value: "half-of-a-yellow-sun-draft".to_owned(),
            }]),
            metadata: Some(metadata),
            attachments: Some(vec![PostmarkAttachment {
                name: "manuscript-chapter-one.pdf".to_owned(),
                content: "JVBERi0xLjQKJcfs".to_owned(),
                content_type: "application/pdf".to_owned(),
            }]),
            message_stream: Some("literary-submissions".to_owned()),
        };

        let json: Value = serde_json::to_value(&postmark).expect("serialization to succeed");

        expect_that!(
            json.get("From").and_then(|v| v.as_str()),
            some(eq("chimamanda.adichie@example.africa"))
        );
        expect_that!(
            json.get("Cc").and_then(|v| v.as_str()),
            some(eq("steve.biko@example.africa"))
        );
        expect_that!(
            json.get("Bcc").and_then(|v| v.as_str()),
            some(eq("miriam.makeba@example.africa"))
        );
        expect_that!(
            json.get("Tag").and_then(|v| v.as_str()),
            some(eq("african-literature"))
        );
        expect_that!(
            json.get("ReplyTo").and_then(|v| v.as_str()),
            some(eq("gbehanzin@example.africa"))
        );
        expect_that!(
            json.get("Headers")
                .and_then(|v| v.as_array())
                .map(|a| a.len()),
            some(eq(1))
        );
        expect_that!(
            json.get("Metadata")
                .and_then(|v| v.get("literary_genre"))
                .and_then(|v| v.as_str()),
            some(eq("african-fiction"))
        );
        expect_that!(
            json.get("Attachments")
                .and_then(|v| v.as_array())
                .map(|a| a.len()),
            some(eq(1))
        );
        expect_that!(
            json.get("MessageStream").and_then(|v| v.as_str()),
            some(eq("literary-submissions"))
        );
    }

    #[cfg(feature = "garde")]
    mod validation_tests {
        use garde::Validate;
        use googletest::matchers::{anything, err, ok};

        use super::*;

        #[gtest]
        fn tag_max_length_1000_fails() {
            let long_tag = "x".repeat(1001);
            let email = EmailMessage {
                from: "miriam.makeba@example.africa".to_owned(),
                to: Recipients::from(vec!["gbehanzin@example.africa".to_owned()]),
                subject: "Mama Africa World Tour Dates".to_owned(),
                body: Body::Text("Music carries the voice of our people across oceans.".to_owned()),
                cc: None,
                bcc: None,
                tag: Some(long_tag),
                reply_to: None,
                headers: None,
                metadata: None,
                attachments: None,
                message_stream: None,
            };

            let postmark: PostmarkEmailRequest = email.into();
            expect_that!(postmark.validate(), err(anything()));
        }

        #[gtest]
        fn tag_at_max_length_1000_passes() {
            let max_tag = "y".repeat(1000);
            let email = EmailMessage {
                from: "wangari.maathai@example.africa".to_owned(),
                to: Recipients::from(vec!["thomas.sankara@example.africa".to_owned()]),
                subject: "Reforestation Partnership Proposal".to_owned(),
                body: Body::Text(
                    "Let us combine our efforts to restore Africa's forests.".to_owned(),
                ),
                cc: None,
                bcc: None,
                tag: Some(max_tag),
                reply_to: None,
                headers: None,
                metadata: None,
                attachments: None,
                message_stream: None,
            };

            let postmark: PostmarkEmailRequest = email.into();
            expect_that!(postmark.validate(), ok(anything()));
        }

        #[gtest]
        fn recipients_max_50_passes() {
            let emails: Vec<String> = (1..=50)
                .map(|count| format!("member{count}@example.africa"))
                .collect();
            let postmark = PostmarkEmailRequest {
                from: "sender@example.africa".to_owned(),
                to: emails,
                subject: "Subject".to_owned(),
                body: PostmarkBody::TextBody("Body".to_owned()),
                cc: None,
                bcc: None,
                tag: None,
                reply_to: None,
                headers: None,
                metadata: None,
                attachments: None,
                message_stream: None,
            };
            expect_that!(postmark.validate(), ok(anything()));
        }

        #[gtest]
        fn recipients_exceeds_50_fails() {
            let emails: Vec<String> = (1..=51)
                .map(|count| format!("overflow{count}@example.africa"))
                .collect();
            let postmark = PostmarkEmailRequest {
                from: "sender@example.africa".to_owned(),
                to: emails,
                subject: "Subject".to_owned(),
                body: PostmarkBody::TextBody("Body".to_owned()),
                cc: None,
                bcc: None,
                tag: None,
                reply_to: None,
                headers: None,
                metadata: None,
                attachments: None,
                message_stream: None,
            };
            expect_that!(postmark.validate(), err(anything()));
        }
    }
}
