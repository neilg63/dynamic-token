mod constants;
mod utils;
mod from_key;
mod to_key;
mod auth_status;
mod valid_auth_token;
mod auth_options;

/// This crate encodes s time-sensitive authentication token from a shared API Key injected in an base-36 timestamp, a set of random split characters and an optional UUID.
/// It also decodes the generated token and applies shared rules to validate the decoded API key, timestamp tolerance, split characters and, 
/// if required, check for a well-formed UUID string for a second authorisation step.

pub use constants::*;
pub use from_key::*;
pub use to_key::*;
pub use auth_status::*;
pub use valid_auth_token::*;
pub use auth_options::*;
pub use utils::{encode_base64,decode_base64};
