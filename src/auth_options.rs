use std::str::Chars;
use simple_string_patterns::CharGroupMatch;

/// Authentication options
/// Only the API key is mandatory.
/// By default a uuid is not validated, the random split characters are `%.,` and max age or tolerance is 5 minutes (or 300000 milliseconds)
/// NB: The random split characters (rand_char_str) may not include alphanumeric characters or underscores, as these would break other validation
/// Usage:
/// ```rust
/// AuthOptions::new("my_cryptic_shared_api_key").check_uuid(true).set_tolerance_secs(1);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AuthOptions<'a> {
  process_uuid: bool,
  api_key: &'a str,
  rand_char_str: &'a str,
  tolerance: u32,
}

impl<'a> AuthOptions<'a> {
  pub fn new(api_key: &'a str) -> Self {
    AuthOptions {
      process_uuid: false,
      api_key,
      rand_char_str: "%.,",
      tolerance: 300_000 // five minutes
    }
  }

  pub fn set_rand_char_str(&mut self, char_str: &'a str) -> Self {
    let is_invalid = char_str.has_alphanumeric() || char_str.contains("_");
    if !is_invalid {
      self.rand_char_str = char_str;
    }
    self.to_owned()
  }

  pub fn key(&self) -> &'a str {
    self.api_key
  }

  pub fn rand_chars(&self) -> Chars {
    self.rand_char_str.chars()
  }

  pub fn tolerance(&self) -> i64 {
    self.tolerance as i64
  }

  pub fn should_check_uuid(&self) -> bool {
    self.process_uuid
  }

  pub fn check_uuid(&mut self, val: bool) -> Self {
    self.process_uuid = val;
    self.to_owned()
  }

  pub fn set_tolerance(&mut self, millis: u32) -> Self {
    self.tolerance = millis;
    self.to_owned()
  }

  pub fn set_tolerance_secs(&mut self, secs: u32) -> Self {
    self.tolerance = secs * 1000;
    self.to_owned()
  }

  pub fn set_tolerance_mins(&mut self, mins: u32) -> Self {
    self.tolerance = mins * 60 * 1000;
    self.to_owned()
  }

}


