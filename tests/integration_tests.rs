use dynamic_token::*;
#[cfg(test)]



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
  
  // now generate a new token 200 milliseconds later with the same options and ensure they are not the same
  let to_key_2 = to_dynamic_key(&options, Some(uuid));
  assert_ne!(to_key_2, to_key);

  
}

#[test]
pub fn test_control_chars() {

  let api_key = "+uNiCode_δࢡබi😉▲⎈";
  let control_chars = "㌀兣◐";
  let options = AuthOptions::new(api_key).set_tolerance_mins(2).set_rand_char_str(control_chars);
  let to_key = to_dynamic_key(&options,None);
  use std::{thread, time};
  let two_hundred_millis = time::Duration::from_millis(200);
  thread::sleep(two_hundred_millis);

  let auth_token = from_dynamic_key(&to_key, &options);

  assert!(auth_token.valid());

}

#[test]
pub fn test_long_uuid() {

  let api_key = "KrYpt0_δࢡබi😉▲⎈"; // you may use non-Latin letters and emojis, indeed any valid unicode character
  let control_chars = "∑㌀兣◐"; // you can use ideograms and emojis but not alphanumeric characters or underscores
  let options = AuthOptions::new(api_key).check_uuid(true).set_tolerance_mins(2).set_rand_char_str(control_chars);
  let long_uuid= "acde070d-8c4c-4f0d-9d8a-162843c10333";
  let to_key = to_dynamic_key(&options,Some(long_uuid));
  use std::{thread, time};
  let two_hundred_millis = time::Duration::from_millis(200);
  thread::sleep(two_hundred_millis);

  let auth_token = from_dynamic_key(&to_key, &options);
  // check if it's valid
  assert!(auth_token.valid());

  // The extracted hexdecimal UUID should be 32 characters long without hyphens
  assert_eq!(auth_token.uuid().len(), 32);

  // check the long UUID is intact without hyphens
  assert_eq!(auth_token.uuid(), long_uuid.replace("-", ""));

}