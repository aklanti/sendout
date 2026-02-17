//! Defines the trait that send the HTTP request over the wire

use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response};

use crate::error::Error;

/// Trait for sending HTTP requests
#[async_trait]
pub trait Execute: Send + Sync {
    /// Sends the request and returns a parsed response
    ///
    /// It returns an error if something goes wrong
    async fn execute<Req, Res>(&self, request: Req) -> Result<Res, Error>
    where
        Req: Into<Request<Bytes>> + Send,
        Res: TryFrom<Response<Bytes>, Error = Error>;
}
