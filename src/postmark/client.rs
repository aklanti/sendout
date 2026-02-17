//! The HTTP client that talks to the Postmark API
use bytes::Bytes;
use http::Request;
use secrecy::ExposeSecret;

#[cfg(feature = "reqwest")]
pub mod reqwest;

use crate::api::ApiRequest;
use crate::config::ServiceConfig;
use crate::error::Error;

/// Client for interacting with Postmark APIs
#[derive(Debug)]
pub struct PostmarkClient<C> {
    /// Service provider configuration
    pub config: ServiceConfig,
    /// HTTP Client
    pub client: C,
}

impl<C> PostmarkClient<C> {
    /// Server header name
    const X_POSTMARK_SERVER_TOKEN: &str = "X-Postmark-Server-Token";
    /// Account header name
    const X_POSTMARK_ACCOUNT_TOKEN: &str = "X-Postmark-Account-Token";

    /// Creates new [`PostmarkClient`] instance
    pub const fn new(client: C, config: ServiceConfig) -> Self {
        Self { client, config }
    }

    /// Creates new HTTP request for Postmark API
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(
            name = "PostmarkClient::new_http_request",
            skip(self, request),
            err(Debug)
        )
    )]
    pub fn new_http_request<R: ApiRequest>(&self, request: &R) -> Result<Request<Bytes>, Error> {
        let body = serde_json::to_vec(request)
            .map(Bytes::from)
            .map_err(|err| {
                #[cfg(feature = "tracing")]
                tracing::error!(?err);
                Error::SendFailed(format!("failed to serialize email: {err}"))
            })?;
        let uri = format!("{}{}", self.config.base_url, R::ENDPOINT);

        let mut request = Request::builder()
            .method(R::METHOD)
            .uri(uri)
            .header("content-type", "application/json")
            .header("accept", "application/json")
            .header(
                Self::X_POSTMARK_SERVER_TOKEN,
                self.config.server_token.expose_secret(),
            );

        if let Some(account_token) = &self.config.account_token {
            request = request.header(
                Self::X_POSTMARK_ACCOUNT_TOKEN,
                account_token.expose_secret(),
            );
        }

        request.body(body).map_err(|err| {
            #[cfg(feature = "tracing")]
            tracing::error!(?err);
            Error::SendFailed(format!("failed to build HTTP request: {err}"))
        })
    }
}
