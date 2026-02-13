//! Integration tests with Postmark client

#![cfg(feature = "postmark")]

mod app;
#[path = "postmark/mod.rs"]
mod postmark_tests;

use reqwest::Client;
use reqwest::redirect::Policy;
use sendout::config::ServiceConfig;
use sendout::postmark::PostmarkClient;
use wiremock::MockServer;

use app::TestApp;

impl TestApp<PostmarkClient<Client>> {
    /// Spawn new test application
    async fn with_postmark() -> Result<Self, Box<dyn std::error::Error>> {
        let email_server = MockServer::start().await;
        let client = Client::builder().redirect(Policy::none()).build()?;
        let config = ServiceConfig {
            base_url: email_server.uri(),
            server_token: String::from("test-server-token").into(),
            account_token: Some(String::from("test-account-token").into()),
            from_email: "test-user".into(),
        };
        let email_client = PostmarkClient::new(client, config.clone());

        Ok(Self {
            email_client,
            email_server,
            config,
        })
    }
}
