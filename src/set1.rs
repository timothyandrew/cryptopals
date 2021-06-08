use std::str;

use crate::util::{byte_array_to_hex, hex_to_base64, hex_to_byte_array};

pub fn c1(s: &str) -> String {
  hex_to_base64(s)
}

pub fn c2(l: &str, r: &str) -> String {
  let l = hex_to_byte_array(l);
  let r = hex_to_byte_array(r);

  assert!(l.len() == r.len());

  let buf = l.iter().zip(r.iter());
  let buf = buf.map(|(l, r)| l ^ r);
  let buf = buf.collect::<Vec<_>>();

  byte_array_to_hex(buf)
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

    #[test]
    fn test_c2() {
        assert_eq!(
          c2("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"), 
          "746865206b696420646f6e277420706c6179"
        );
    }
}
