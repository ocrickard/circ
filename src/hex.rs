
fn decode_hex_byte(byte: u8) -> Result<u8, String> {
  // From Brian Smith's ring project example
  if byte >= b'0' && byte <= b'9' {
    Ok(byte - b'0')
  } else if byte >= b'a' && byte <= b'f' {
    Ok(byte - b'a' + 10u8)
  } else if byte >= b'A' && byte <= b'F' {
    Ok(byte - b'A' + 10u8)
  } else {
    Err(format!("Invalid hex digit '{}'", byte as char))
  }
}

// Take a binary byte and convert to a hex-encoded utf8 byte.
fn encode_hex_byte(byte: &u8) -> Result<u8, String> {
  // 0123456789abcdef
  static HEX_CONVERSION_TABLE: &'static [u8; 16] = b"0123456789abcdef";

  if *byte as usize >= HEX_CONVERSION_TABLE.len() {
    Err(format!("Invalid byte: {}", byte))
  } else {
    Ok(HEX_CONVERSION_TABLE[*byte as usize])
  }
}

// Take a hex-encoded utf8 buffer and convert to a binary buffer.
pub fn decode_hex_buffer(buffer: &[u8]) -> Result<Vec<u8>, String> {
  if buffer.len() % 2 != 0 {
    return Err("Hex string does not have an even number of digits".to_string());
  }

  // Each hex byte holds 4 bits of information for the binary buffer.
  // Thus we just have to combine each pair of 2 values in the original
  // buffer, decode their values, and we've got a binary buffer.
  let mut result = Vec::with_capacity(buffer.len() / 2);
  for digits in buffer.chunks(2) {
    let hi = decode_hex_byte(digits[0])?;
    let lo = decode_hex_byte(digits[1])?;
    result.push((hi * 0x10) | lo);
  }
  Ok(result)
}

// Take a binary buffer and encode to utf8 hex bytes.
pub fn encode_hex_buffer(buffer: &[u8]) -> Result<Vec<u8>, String> {
  // Each binary byte contains 8 bits of information. We need to map
  // this to twice that number of hex bytes, which each contain 4 bits.
  let mut output: Vec<u8> = Vec::new();

  for c in buffer {
    output.push(encode_hex_byte(&(c >> 4))?);
    output.push(encode_hex_byte(&(c & 0b1111))?);
  }

  Ok(output)
}