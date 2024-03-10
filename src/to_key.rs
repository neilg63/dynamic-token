use crate::auth_options::AuthOptions;
use crate::utils::*;


/// Generate a time-sensitive dynamic key from a shared API key as defined in the options
/// the current millisecond timestamp (encoded, split and injected)
/// some random characters and an optional uuid that can be decoded and used for additional authentication
pub fn to_dynamic_key(options: &AuthOptions, uuid_opt: Option<&str>) -> String {  
  let ts = milliseconds_now(); 
  let ts_list = i64_to_base_36(ts).unwrap_or("".to_string()).chars().rev().collect::<String>();
  let ts_list_36 = base_36_str_to_u64(&ts_list[..1]).unwrap_or(0);
  let is_under = ts_list_36 < usize::MAX as u64;
  let offset = if is_under { (ts_list_36 as usize % 6) + 2 } else { 0 };
  let mut parts: Vec<String> = vec![];

  let merged_list = [&ts_list.as_str()[..offset], options.key(), &ts_list.as_str()[offset..]].concat();
  let base_str = [merged_list, rand_int_36(3)].join(
    &rand_char_as_string(options.rand_chars())
  );
  parts.push(base_str);
  if let Some(uuid) = uuid_opt { 
    if let Some(uid_str) = hex_string_to_base36_parts(uuid) {
      let rand_int_str = rand_int_36(3);
      let uid_part = [uid_str, rand_int_str].join(
        &rand_char_as_string(options.rand_chars())
      );
      parts.push(uid_part);
    }
  }
  let key_str = parts.join("__");
  encode_base64(&key_str)
}