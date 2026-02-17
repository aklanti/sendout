[![Build Status][actions-badge]][actions-url]
[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MPL-2.0 license][mpl-2.0-badge]][mpl-2.0-license]

[crates-badge]: https://img.shields.io/crates/v/sendout
[crates-url]: https://crates.io/crates/sendout
[docs-badge]: https://img.shields.io/docsrs/sendout/latest
[docs-url]: https://docs.rs/sendout/latest/sendout/
[mpl-2.0-badge]: https://img.shields.io/badge/License-MPL_2.0-blue.svg
[mpl-2.0-license]: LICENSE
[actions-badge]: https://github.com/aklanti/sendout/workflows/CI/badge.svg
[actions-url]: https://github.com/aklanti/sendout/actions/workflows/main.yaml

# Sendout

Send emails from Rust through API-based providers like Postmark, with strong types and compile-time validation.

This crate will never include open tracking or other privacy-invasive features.

## Usage

Add `sendout` to your project:

```toml
[dependencies]
sendout = { version = "0.2", features = ["postmark", "reqwest"] }
```

Then send an email:

```rust
use reqwest::Client;
use sendout::email::EmailMessage;
use sendout::EmailService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let email_message = EmailMessage {
        from: "wangari.maathai@example.africa".to_owned(),
        to: vec!["kwame.nkrumah@example.africa"].into(),
        subject: "Green Belt Movement Monthly Update".to_owned(),
        body: Body::Text("We planted 10,000 trees across Kenya this month.".to_owned()),
    };

    let config = ServiceConfig {
        base_url: "https://example.africa".into(),
        server_token: String::from("<SERVER_TOKEN>").into(),
        account_token: Some("<ACCOUNT_TOKEN>").into(),
        from_email: "test-user".into(),
    };

    let reqwest_client = Client::new();
    let postmark_client = PostmarkClient::new(client, config);
    postmark_client.send_email(email_message).await?;

    Ok(())
}
```

## Optional Features

- `postmark` - Postmark provider support
- `reqwest` - reqwest as the HTTP backend
- `bon` - builder pattern for messages
- `garde` - validate fields like email format, lengths, and more
- `tracing` - instrument calls with the `tracing` ecosystem
- `test-util` - mock sender and helpers for testing

 ### Supported Rust Versions

The minimum supported Rust version is **1.93.0**.

### License

Unless otherwise noted, this project is licensed under the [Mozilla Public License Version 2.0.](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `sendout` by you, shall be licensed as MPL-2.0, without any additional
terms or conditions.
