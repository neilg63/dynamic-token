
/// characters used for base-36 conversion
pub const BASE_36_CHARS: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

/// characters used for hexadecimal conversion
pub const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

/// Minimum length of the hexadecimal UUID
pub const MIN_VALID_UUID_LENGTH: usize = 24;

/// Max random start offset for the encoded API key after the initial control character
pub const MAX_API_KEY_OFFSET: usize = 6;