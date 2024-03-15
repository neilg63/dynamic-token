use simple_string_patterns::CharGroupMatch;

/// Authentication options
/// Only the API key is mandatory.
/// By default a uuid is not validated, the random split characters are `%.,` and max age or tolerance is 5 minutes (or 300000 milliseconds)
/// NB: The random split characters (rand_char_str) may not include alphanumeric characters or underscores, as these would break other validation
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

  /// Return the shared API key, for internal use in your app only
  pub fn key(&self) -> &'a str {
    self.api_key
  }

  /// Return the random split characters as an array
  pub fn rand_chars(&self) -> Vec<char> {
    self.rand_char_str.chars().into_iter().collect::<Vec<char>>()
  }

  /// Timestamp tolerance in milliseconds
  pub fn tolerance(&self) -> i64 {
    self.tolerance as i64
  }

  /// Only validate embedded UUIDs if this flag is true
  pub fn should_check_uuid(&self) -> bool {
    self.process_uuid
  }

  /// Set check UUID status. True means it must be present in a valid hexadecimal format once decoded
  pub fn check_uuid(&mut self, val: bool) -> Self {
    self.process_uuid = val;
    self.to_owned()
  }

  /// Set timestamp tolerance in milliseconds
  pub fn set_tolerance(&mut self, millis: u32) -> Self {
    self.tolerance = millis;
    self.to_owned()
  }

  /// Set timestamp tolerance in seconds
  pub fn set_tolerance_secs(&mut self, secs: u32) -> Self {
    self.tolerance = secs * 1000;
    self.to_owned()
  }

  /// Set timestamp tolerance in minutes
  pub fn set_tolerance_mins(&mut self, mins: u32) -> Self {
    self.tolerance = mins * 60 * 1000;
    self.to_owned()
  }

}


