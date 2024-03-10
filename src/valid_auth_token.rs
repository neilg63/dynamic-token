use crate::auth_status::AuthStatus;

#[derive(Debug, Clone)]
pub struct ValidAuthToken(pub bool, pub Option<String>, pub Option<i64>, pub AuthStatus);

impl ValidAuthToken {
  // Always return a 64bit integer for milliseconds irrespective of decoded initial values
  // The age is the milliseconds between the timestamp encoded in the token and the current universal timestamp
  // Negative values indicate the encoded timestamp is the future
  pub fn age(&self) -> i64 {
    if let Some(age_int) = self.2 {
      age_int
    } else {
      0i64
    }
  }

  pub fn valid(&self) -> bool {
    self.0
  }

  pub fn uuid(&self) -> String {
    self.1.clone().unwrap_or("".to_string())
  }

  pub fn has_user(&self) -> bool {
    self.valid() && self.1.is_some()
  }

  pub fn status(&self) -> AuthStatus {
    self.3.to_owned()
  }

  pub fn status_key(&self) -> String {
    self.3.to_key()
  }

}