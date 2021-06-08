use std::str;
use std::collections::HashMap;
use std::str::from_utf8;

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

fn score_plaintext(s: &str) -> isize {
  let s = s.to_lowercase();
  let mut score: isize = 0;
  let acc: HashMap<char, isize> = HashMap::new();

  let freq = s.chars().fold(acc, |mut acc, c| {
    let val = acc.entry(c).or_insert(0);
    *val += 1;
    acc
  });

  let pairs = vec![
    ('e', 't'), ('t', 'a'), ('a', 'o'), ('o', 'i'), ('i', 'n'),
    ('n', 's'), ('s', 'r'), ('r', 'h'), ('h', 'd'), ('d', 'l'), 
    ('l', 'u'), ('u', 'c')
  ];

  for (l, r) in pairs {
    if freq.contains_key(&l) && freq.contains_key(&r) && freq[&l] <= freq[&r] { 
      score += 50; 
    }
  }

  score
}

pub fn c3(s: &str) -> String {
  let scores = (0..=255).map(|xor| {
    let s = hex_to_byte_array(s);
    let s = s.iter().map(|c| c ^ xor);
    String::from_utf8(s.collect::<Vec<_>>())
  })
  .filter_map(|s| s.ok())
  .max_by_key(|s| {
    score_plaintext(s)
  });

  let x = scores.unwrap();
  println!("{}", x);
  x
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

    #[test]
    fn test_c3() {
        assert_eq!(
          c3("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"), 
          "cOOKING\u{0}mc\u{7}S\u{0}LIKE\u{0}A\u{0}POUND\u{0}OF\u{0}BACON"
        );
    }
}
