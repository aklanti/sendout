//! HTTP request execution abstraction

use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response};

use crate::error::SendoutError;

/// Trait for executing HTTP requests
#[async_trait]
pub trait Execute: Send + Sync {
    /// Execute an HTTP request and return the response
    async fn execute<Req, Res>(&self, request: Req) -> Result<Res, SendoutError>
    where
        Req: Into<Request<Bytes>> + Send,
        Res: TryFrom<Response<Bytes>, Error = SendoutError>;
}
