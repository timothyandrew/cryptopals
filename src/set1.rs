use std::str;

pub fn c1(s: &str) -> String {
  if s.len() % 2 != 0 {
    panic!("Hex strings should have length divisible by 2");
  }

  // This is ok because all chars in range only take up a single byte
  let buf = s.as_bytes().chunks_exact(2);
  let buf = buf.map(|c| str::from_utf8(c).unwrap());
  let buf = buf.map(|c| u8::from_str_radix(c, 16));
  let buf = buf.map(|c| c.unwrap());
  let buf = buf.collect::<Vec<_>>();

  base64::encode(buf)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_c1() {
        assert_eq!(
          c1("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"), 
          "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
