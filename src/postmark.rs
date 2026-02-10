//! Email sending module with Postmark

pub mod client;
pub mod request;
pub mod response;

#[doc(inline)]
pub use client::PostmarkClient;
#[doc(inline)]
pub use request::PostmarkEmailRequest;
#[doc(inline)]
pub use response::PostmarkEmailResponse;
