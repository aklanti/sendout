//! Email sending module with Postmark

pub mod request;
pub mod response;

#[doc(inline)]
pub use request::PostmarkEmailRequest;
#[doc(inline)]
pub use response::PostmarkEmailResponse;
