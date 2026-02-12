//! Implements services using reqwest::Client as client
use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response, StatusCode};
use reqwest::Client;

use super::PostmarkClient;
use crate::EmailService;
use crate::email::{EmailDelivery, EmailMessage};
use crate::error::Error;
use crate::execute::Execute;
use crate::postmark::{PostmarkEmailRequest, PostmarkEmailResponse};

#[async_trait]
impl EmailService<EmailMessage, EmailDelivery> for PostmarkClient<Client> {
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(name = "PostmarkClient::send_email")
    )]
    async fn send_email(&self, email: EmailMessage) -> Result<EmailDelivery, Error> {
        let postmark_request: PostmarkEmailRequest = email.into();
        let request = self.new_http_request(&postmark_request)?;

        let response: PostmarkEmailResponse = self.execute(request).await.inspect_err(|_err| {
            #[cfg(feature = "tracing")]
            tracing::error!(?_err);
        })?;

        Ok(response.into())
    }
}

#[async_trait]
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
