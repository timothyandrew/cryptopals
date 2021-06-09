use std::fs::read_to_string;
use std::str::from_utf8;

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

    let mut pairs = vec!['e', 't', 'a', 'o', 'i', 'n'];
    pairs.reverse();

    for (i, c) in pairs.iter().enumerate() {
        if let Some(n) = freq.get(c) {
            score += n + i;
        }
    }

    score
}

pub fn decode_single_char_xor(s: &Vec<u8>) -> Option<String> {
    (0..=255)
        .map(|xor| {
            let s = s.iter().map(|c| c ^ xor);
            String::from_utf8(s.collect::<Vec<_>>())
        })
        .filter_map(|s| s.ok())
        .max_by_key(|s| score_plaintext(s))
}

pub fn decode_single_char_xor_findkey(s: &Vec<u8>) -> Option<(u8, String)> {
    (0..=255)
        .map(|xor| {
            let s = s.iter().map(|c| c ^ xor);
            (xor, String::from_utf8(s.collect::<Vec<_>>()))
        })
        .filter_map(|(xor, s)| if let Ok(s) = s { Some((xor, s)) } else { None })
        .max_by_key(|(_xor, s)| score_plaintext(s))
}

pub fn file_to_lines(filename: &str) -> Vec<String> {
    let s = read_to_string(filename);
    let s = s.unwrap();
    let s = s.split("\n").map(|l| String::from(l.trim()));
    s.collect::<Vec<_>>()
}

pub fn edit_distance(l: &[u8], r: &[u8]) -> u32 {
    assert!(l.len() == r.len());

    let mut result = 0;

    for i in 0..l.len() {
        result += u8::count_ones(l[i] ^ r[i]);
    }

    result
}

pub fn transpose_bytes(bytes: &Vec<u8>, chunk_size: usize) -> Vec<Vec<u8>> {
    let chunks = bytes.chunks(chunk_size).collect::<Vec<_>>();

    let transposed = (0..chunk_size).map(|n| {
        let mut result = Vec::new();

        for &chunk in &chunks {
            if n < chunk.len() {
                result.push(chunk[n]);
            }
        }

        result
    });

    transposed.collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        let l = "this is a test".as_bytes();
        let r = "wokka wokka!!!".as_bytes();

        assert_eq!(edit_distance(l, r), 37);
    }
}
