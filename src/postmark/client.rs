//! Postmark client

use bytes::Bytes;
use http::Request;
use secrecy::ExposeSecret;

use crate::api::ApiRequest;
use crate::config::ServiceConfig;
use crate::error::Error;

/// Client for interacting with Postmark APIs
#[derive(Debug)]
pub struct PostmarkClient<C> {
    /// Service configuration data
    pub config: ServiceConfig,
    /// HTTP Client
    pub client: C,
}

impl<C> PostmarkClient<C> {
    /// Server header name
    const X_POSTMARK_SERVER: &str = "X-POSTMARK-SERVER";
    ///  Account header name
    const X_POSTMARK_ACCOUNT: &str = "X-POSTMARK-ACCOUNT";

    /// Creates new HTTP request for sending an email via the Postmark API
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
            .header(
                Self::X_POSTMARK_SERVER,
                self.config.server_token.expose_secret(),
            );

        if let Some(account_token) = &self.config.account_token {
            request = request.header(Self::X_POSTMARK_ACCOUNT, account_token.expose_secret());
        }

        request.body(body).map_err(|err| {
            #[cfg(feature = "tracing")]
            tracing::error!(?err);
            Error::SendFailed(format!("failed to build HTTP request: {err}"))
        })
    }
}

#[cfg(feature = "reqwest")]
#[async_trait::async_trait]
impl crate::Execute for PostmarkClient<reqwest::Client> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(name = "PosmarkClient::execute", skip(self, request), err(Debug))
    )]
    async fn execute<Req, Res>(&self, request: Req) -> Result<Res, Error>
    where
        Req: Into<Request<Bytes>> + Send,
        Res: TryFrom<http::Response<Bytes>, Error = Error>,
    {
        use http::{Response, StatusCode};

        let request = request.into();
        let reqwest_request = request.try_into().inspect_err(|_err| {
            #[cfg(feature = "tracing")]
            tracing::error!(?_err);
        })?;

        let response = self.client.execute(reqwest_request).await?;

        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            return Err(Error::RateLimitExceeded);
        }
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.bytes().await.inspect_err(|_err| {
            #[cfg(feature = "tracing")]
            tracing::error!(?_err)
        })?;

        let mut http_response = Response::builder()
            .status(status)
            .body(body)
            .map_err(|err| {
                #[cfg(feature = "tracing")]
                tracing::error!(?err);

                Error::SendFailed(format!("failed to create response {err}"))
            })?;

        *http_response.headers_mut() = headers;
        Res::try_from(http_response)
    }
}
