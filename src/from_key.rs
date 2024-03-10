use simple_string_patterns::ToSegments;
use crate::utils::*;
use crate::valid_auth_token::ValidAuthToken;
use crate::auth_status::*;
use crate::auth_options::*;

/// Validate the dynamic key based on a shared API key, random characters, a timestamp and optional userID
pub fn from_dynamic_key(
  sample: &str,
  options: &AuthOptions,
) -> ValidAuthToken {
  let mut uuid: Option<String> = None;
  let mut valid = false;
  let api_key = options.key();
  let mut status = AuthStatus::None;
  let decoded_string = decode_base64(sample);
  let first_char = decoded_string.chars().nth(0).unwrap_or('-');
  let mut age: Option<i64> = None;
  if first_char.is_alphanumeric() {
    let decoded_int = base_36_to_u8(&first_char).unwrap_or(0);
    let offset = (decoded_int % 6) + 2;
    let api_key_index = decoded_string.find(api_key).unwrap_or(255) as u8;
    if api_key_index == offset {
      let parts = decoded_string.to_parts("__");
      // check userId if required
      if options.should_check_uuid() && parts.len() > 1 {
        if let Some(long_str) = parts.last() {
          let (first, second) = long_str.to_head_tail("_");
          if second.len() > 6 {
            let (tail_end, int_str) = random_chars_split(&second, options.rand_chars());
            let uid_str = [
              base_36_parts_to_hex_dec(&first),
              base_36_parts_to_hex_dec(&tail_end)
            ].into_iter().map(|opt| opt.unwrap_or("".to_string())).collect::<Vec<String>>().concat();
            let rand_int = base_36_str_to_u64(&int_str);
            
            if uid_str.len() > 23 {
              uuid = Some(uid_str.clone());
              valid = rand_int.is_some();
              status = if valid {
                AuthStatus::Ok
              } else {
                AuthStatus::UuidRequired
              };
            }
          } 
        }
      } else {
        valid = true;
        status = AuthStatus::Ok;
      }
      // use only the base string before the user ID
      let base_str = parts.get(0).unwrap();
      let ts_parts = base_str.to_parts(api_key);
      // the timestamp parts are either side if the decoded API key. Concatenate them to reassemble
      // ts_str and base_suffix is base-36 encoded timestamps. The latter is randomised and must only be a valid integer
      let (ts_str, base_suffix) = random_chars_split(
        &ts_parts.concat(),
        options.rand_chars()
      );
      if valid {
        // must have a valid base-36 format
        valid = ts_str.chars().all(|c| c.is_alphanumeric());
        if !valid {
          status = AuthStatus::InvalidTimestamp;
        }
      }
      // If a UUID is required the timestamp wioll not be authenticated
      if valid {
        if let Some(_suffix_int) = base_36_str_to_u64(&base_suffix) {
          // reverse the the base-36 string
          let characters: String = ts_str.chars().rev().collect();
          // Cast it to u64 and then to i64 for comparison
          let ts = base_36_str_to_u64(&characters).unwrap_or(0) as i64;
          let curr_ts = milliseconds_now();
          let age_ms = curr_ts - ts;
          let max_age_ms = options.tolerance();
          let min_age_ms = 0 - max_age_ms;
          // the dynamic key is valid if it's within the time range allowing for HTTP query time and / or minor discrepancies current unix time
          valid = age_ms >= min_age_ms && age_ms <= max_age_ms;
          // assign the age in milliseconds
          age = Some(age_ms);
          // if a uuid is required, the token is only valid if it has been set
          status = if valid {
            AuthStatus::Ok
          } else {
            AuthStatus::TimedOut
          };
        }
      }
    }
  }
  ValidAuthToken(valid, uuid, age, status)
}
