use std::collections::HashSet;
use std::str;

use ordered_float::OrderedFloat;

use crate::util::decode_single_char_xor;
use crate::util::decode_single_char_xor_findkey;
use crate::util::edit_distance;
use crate::util::file_to_lines;
use crate::util::score_plaintext;
use crate::util::transpose_bytes;
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

pub fn c3(s: &str) -> String {
    let s = hex_to_byte_array(s);
    decode_single_char_xor(&s).unwrap()
}

pub fn c4(filename: &str) -> String {
    let lines = file_to_lines(filename);
    let lines = lines.iter().map(|l| {
        let l = hex_to_byte_array(l);
        decode_single_char_xor(&l)
    });
    let lines = lines.filter_map(|l| l);
    let line = lines.max_by_key(|l| score_plaintext(l));
    line.unwrap()
}

pub fn c5(s: &str) -> String {
    let key = vec!['I', 'C', 'E'];
    let mut key = key.iter().map(|c| *c as u8).cycle();
    let s = s.as_bytes();

    let s = s.iter().map(|c| c ^ key.next().unwrap());

    let s = s.collect::<Vec<_>>();

    byte_array_to_hex(s)
}

pub fn c6(filename: &str) -> HashSet<String> {
    let b = file_to_lines(filename);
    let b = b.join("");
    let b = base64::decode(b).unwrap();

    let mut keysizes = (2..=40).collect::<Vec<_>>();

    keysizes.sort_by_key(|&keysize| {
        let first = &b[0..keysize];
        let second = &b[keysize..(keysize * 2)];
        let third = &b[(keysize * 2)..(keysize * 3)];

        // I'm still not entirely sure _why_ this works. Why exactly is it the case that
        // the right keysize minimizes the edit distance between chunks? 🤔
        let dist = edit_distance(first, second)
            + edit_distance(second, third)
            + edit_distance(first, third);
        let dist = dist as f64 / 3.0;

        OrderedFloat(dist / keysize as f64)
    });

    let keys = keysizes[0..4].iter().map(|keysize| {
        let transposed = transpose_bytes(&b, *keysize);
        let transposed = transposed
            .iter()
            .map(|t| decode_single_char_xor_findkey(t).unwrap());
        let transposed = transposed.map(|(xor, _s)| xor);
        let transposed = transposed.collect::<Vec<_>>();
        String::from_utf8(transposed).unwrap()
    });

    keys.collect::<HashSet<_>>()
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
            c2(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
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

    #[test]
    fn test_c4() {
        assert_eq!(c4("resources/s1c4.txt"), "Now that the party is jumping\n");
    }

    #[test]
    fn test_c5() {
        assert_eq!(
          c5("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"), 
          "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }

    #[test]
    fn test_c6() {
        let result = c6("resources/s1c6.txt");
        assert!(result.contains("terminator x: bring the noise"));
    }
}
