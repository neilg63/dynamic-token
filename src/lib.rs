mod constants;
mod utils;
mod from_key;
mod to_key;
mod auth_status;
mod valid_auth_token;
mod auth_options;

pub use constants::*;
pub use from_key::*;
pub use to_key::*;
pub use auth_status::*;
pub use valid_auth_token::*;
pub use auth_options::*;



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_dynamic_key() {
    use std::{thread, time};
    let tolerance_in_millis = 200;
    let hundred_millis = time::Duration::from_millis(tolerance_in_millis / 2);

    let uuid = "6061f78686f34f52da3ef464";
    let api_key = "Opus;Magna-897";
    let options = AuthOptions::new(api_key).check_uuid(true).set_tolerance(tolerance_in_millis as u32);
    let to_key = to_dynamic_key(&options, Some(uuid));
    let valid_auth_token = from_dynamic_key(&to_key, &options);

    println!("API KEY: {}, uuid: {}, encoded {}", api_key, uuid, to_key);

    assert!(valid_auth_token.valid());

    assert!(valid_auth_token.age() < 1000); // less than a second
    
    assert!(valid_auth_token.has_user());

    // check the encoded uuid matches the decoded one
    assert_eq!(valid_auth_token.uuid(), uuid.to_string());

    // check if it's still valid after 100 milliseconds
    thread::sleep(hundred_millis);
    let auth_token_2 = from_dynamic_key(&to_key, &options);
    assert!(auth_token_2.valid());

    // Make sure the token is not valid after 200 milliseconds millisecond timeout. NB: in practice you allow at least 5 or seconds
    thread::sleep(hundred_millis);
    let auth_token_3 = from_dynamic_key(&to_key, &options);
    assert_eq!(auth_token_3.valid(), false);
    assert_eq!(auth_token_3.status(), AuthStatus::TimedOut);
    
  }
}

