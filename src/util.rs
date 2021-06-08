use std::fs::read_to_string;
use std::str::from_utf8;
use std::fs::File;
use std::collections::HashMap;

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

pub fn byte_array_to_utf8(hex: &str) -> String {
  base64::encode(hex_to_byte_array(hex))
}

pub fn byte_array_to_hex(b: Vec<u8>) -> String {
  let hex = b.iter().map(|d| format!("{:01$x}", d, 2));
  hex.collect::<Vec<_>>().join("")
}

pub fn hex_to_base64(hex: &str) -> String {
  base64::encode(hex_to_byte_array(hex))
}

pub fn score_plaintext(s: &str) -> usize {
  let s = s.to_lowercase();
  let mut score: usize = 1;
  let acc: HashMap<char, usize> = HashMap::new();

  let freq = s.chars().fold(acc, |mut acc, c| {
    let val = acc.entry(c).or_insert(0);
    *val += 1;
    acc
  });

  let mut pairs = vec![ 'e', 't', 'a', 'o', 'i', 'n'];
  pairs.reverse();

  for (i, c) in pairs.iter().enumerate() {
    if let Some(n) = freq.get(c) {
      score += n + i;
    }
  }

  score
}

pub fn decode_single_char_xor(s: &str) -> Option<String> {
  (0..=255).map(|xor| {
    let s = hex_to_byte_array(s);
    let s = s.iter().map(|c| c ^ xor);
    String::from_utf8(s.collect::<Vec<_>>())
  })
  .filter_map(|s| s.ok())
  .max_by_key(|s| {
    println!("{} {}", s, score_plaintext(s));
    score_plaintext(s)
  })
}

pub fn file_to_lines(filename: &str) -> Vec<String> {
  let s = read_to_string(filename);
  let s = s.unwrap();
  let s = s.split("\n").map(|l| String::from(l.trim()));
  s.collect::<Vec<_>>()
}