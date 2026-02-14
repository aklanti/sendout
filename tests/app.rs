//! Test application

use reqwest::Client;
use reqwest::redirect::Policy;
use sendout::ServiceConfig;
use sendout::email::{Body, EmailMessage};
use uuid::Uuid;
use wiremock::MockServer;

/// Test application
pub struct TestApp {
    /// Email service provider server
    pub email_server: MockServer,
    /// Email server configuration
    pub config: ServiceConfig,
}

impl TestApp {
    /// Spawns new test application
    pub async fn spawn() -> Self {
        let email_server = MockServer::start().await;
        let config = ServiceConfig {
            base_url: email_server.uri(),
            server_token: String::from(Uuid::new_v4()).into(),
            account_token: Some(String::from(Uuid::new_v4()).into()),
            from_email: "test-user".into(),
        };

        Self {
            email_server,
            config,
        }
    }

    /// Creates new request client
    pub fn reqwest_client() -> Result<Client, Box<dyn std::error::Error>> {
        let client = Client::builder().redirect(Policy::none()).build()?;
        Ok(client)
    }

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
