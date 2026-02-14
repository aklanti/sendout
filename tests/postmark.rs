//! Integration tests with Postmark client

#![cfg(all(feature = "postmark", feature = "reqwest"))]

mod app;
#[path = "postmark/mod.rs"]
mod postmark_tests;

use reqwest::Client;
use sendout::postmark::PostmarkClient;

use app::TestApp;

impl TestApp {
    /// Spawn new test application
    fn postmark_client(&self) -> PostmarkClient<Client> {
        let reqwest_client = Self::reqwest_client().expect("to create reqwest client");
        PostmarkClient::new(reqwest_client, self.config.clone())
    }
}
