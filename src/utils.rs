use std::str::{self};
use rand::Rng;
use utcnow::utcnow;
use base64::{Engine as _, engine::general_purpose};
use crate::constants::*;

/// Fetch the current timestamp in milliseconds with the standard library
pub(crate) fn milliseconds_now() -> i64 {
  let ts = utcnow().unwrap().as_millis();
  let max = i64::MAX;
  if ts <= max as i128 {
    ts as i64
  } else {
    max
  }
}

pub(crate) fn from_base_36(sample: &str) -> Option<Vec<u8>> {
  if sample.len() > 0 {
    base_encode::from_str(sample, 36, BASE_36_CHARS)
  } else {
    None
  }
}

pub(crate) fn from_base_16(sample: &str) -> Option<Vec<u8>> {
  base_encode::from_str(sample, 16, HEX_CHARS)
}

pub(crate) fn base_36_parts_to_hex_dec(sample: &str) -> Option<String> {
    if let Some(values) = from_base_36(sample) {
      base_encode::to_string(&values, 16, HEX_CHARS)
    } else {
      None
    }
}

pub(crate) fn hex_dec_to_base_36(sample: &str) -> Option<String> {
    if let Some(values) = from_base_16(sample) {
      base_encode::to_string(&values, 36, BASE_36_CHARS)
    } else {
      None
    }
}

/// Convert a base-10 integer to a base-36 string
pub(crate) fn dec_to_base_36(value: u32) -> Option<String> {
  let buf = value.to_be_bytes();
  base_encode::to_string(&buf, 36, BASE_36_CHARS)
}

/// Convert a large integer to a shorter sequence of letters and numerals
pub(crate) fn i64_to_base_36(value: i64) -> Option<String> {
  let buf = value.to_be_bytes();
  base_encode::to_string(&buf, 36, BASE_36_CHARS)
}

/// Convert a base-36 digit to u8 integer.
pub(crate) fn base_36_to_u8(letter: &char) -> Option<u8> {
  if let Some(values) = from_base_36(&letter.to_string()) {
    if values.len() > 0 {
      if let Some(value) = values.last() {
        Some(*value)
      } else {
          None
      }
    } else {
      None
    }
  } else {
    None
  }
}

pub(crate) fn base_36_str_to_u64(sample: &str) -> Option<u64> {
  if let Some(values) = from_base_36(sample) {
    let radix: u64 = 256;
    let num_values = values.len();
    let max_36_pow = 12;
    if num_values > 0 {
      let max_pow = if num_values < max_36_pow { values.len() - 1 } else { max_36_pow };
      let mut curr_pow = max_pow as u32;
      let mut sum: u64 = 0;
      for v in values {
        let multiplier = if curr_pow > 0 { radix.pow(curr_pow) } else { 1u64 };
        sum += v as u64 * multiplier;
        if curr_pow > 0 { 
          curr_pow -= 1;
        }
      }
      Some(sum)
    } else {
        None
    }
  } else {
    None
  }
}

pub(crate) fn random_int(max: u32) -> u32 {
  let mut rng = rand::thread_rng();
  rng.gen::<u32>() % max
}

pub(crate) fn rand_char(characters: &[char]) -> char {
  let len = characters.len();
  let rand_index = random_int(len as u32) as usize;
  characters.get(rand_index).unwrap_or(&' ').to_owned()
}

pub(crate) fn rand_char_as_string(rand_chars: &[char]) -> String{
  rand_char(rand_chars).to_string()
}

pub(crate) fn rand_int_36(power: u8) -> String {
  let max = 10u32.pow(power as u32);
  let rand_int = random_int(max);
  dec_to_base_36(rand_int).unwrap_or("".to_string())
}

pub(crate) fn hex_string_to_base36_parts(hex_str: &str) -> Option<String> {
  if hex_str.len() >= MIN_VALID_UUID_LENGTH {
    let base36_str = vec![&hex_str[..12],&hex_str[12..]].into_iter()
      .map(|hd| hex_dec_to_base_36(hd).unwrap_or("".to_string()))
      .collect::<Vec<String>>()
      .join("_");
    Some(base36_str)
  } else {
    None
  }
}

/// base-64-encode with standard options
pub fn encode_base64(key_str: &str) -> String {
  general_purpose::STANDARD.encode(key_str)
}

/// base-64-decode with standard options
pub fn decode_base64(sample: &str) -> String {
  let decoded = general_purpose::STANDARD.decode(sample).unwrap_or(vec![]);
  str::from_utf8(&decoded).unwrap_or("").to_string()
}