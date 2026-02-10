//! Email data structure

pub mod delivery;
pub mod single_email;

#[doc(inline)]
pub use delivery::EmailDelivery;
#[doc(inline)]
pub use single_email::{Attachment, Body, Header, Recipients, SingleEmail};
