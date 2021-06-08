use std::str::from_utf8;

pub fn hex_to_byte_array(hex: &str) -> Vec<u8> {
  if hex.len() % 2 != 0 {
    panic!("Hex strings should have length divisible by 2");
  }

  // This is ok because all chars in range only take up a single byte
  let buf = hex.as_bytes().chunks_exact(2);
  let buf = buf.map(|c| from_utf8(c).unwrap());
  let buf = buf.map(|c| u8::from_str_radix(c, 16));
  let buf = buf.map(|c| c.unwrap());
  buf.collect::<Vec<_>>()
}

pub fn byte_array_to_hex(b: Vec<u8>) -> String {
  let hex = b.iter().map(|d| format!("{:x}", d));
  hex.collect::<Vec<_>>().join("")
}

pub fn hex_to_base64(hex: &str) -> String {
  base64::encode(hex_to_byte_array(hex))
}