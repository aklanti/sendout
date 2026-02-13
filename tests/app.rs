//! Test application

use sendout::ServiceConfig;
use sendout::email::{Body, EmailMessage};
use wiremock::MockServer;

/// Test application
pub struct TestApp<EmailClient> {
    /// Email service provider client
    pub email_client: EmailClient,
    /// Email service provider server
    pub email_server: MockServer,
    /// Email server configuration
    pub config: ServiceConfig,
}

impl<EmailClient> TestApp<EmailClient> {
    /// Create email message
    pub fn email_message() -> EmailMessage {
        EmailMessage {
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
        }
    }
}
