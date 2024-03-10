
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AuthStatus {
  None, // could not be authenticated
  Ok, // is OK
  ApiKeyUnmatched, // the API key is missing from the decoded token
  ApiKeyMisplaced, // the API key is present but in the wrong place
  UuidRequired, // a correctly formatted UUID is required
  SecurityNumberRequired, // the random security number is not a valid integer, once decoded
  TimedOut, // the injected timestamp is older the max age
  InvalidTimestamp, // the timestamp components are malformed and cannot be converted to a millisecond timestamp
}

impl AuthStatus {
  pub fn to_key(&self) -> String {
    match self {
      AuthStatus::None => "invalid",
      AuthStatus::Ok => "ok",
      AuthStatus::ApiKeyUnmatched => "api_key_unmatched",
      AuthStatus::ApiKeyMisplaced => "api_key_misplaced",
      AuthStatus::UuidRequired => "uuid_required",
      AuthStatus::SecurityNumberRequired => "security_number_required",
      AuthStatus::TimedOut => "timed_out",
      AuthStatus::InvalidTimestamp => "invalid_imestamp",
    }.to_string()
  }
}
