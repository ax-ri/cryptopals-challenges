use std::fs;
use std::iter::zip;
use std::num::ParseIntError;

// from https://stackoverflow.com/a/52992629
pub fn hex_string_to_bytes(hex_string: &str) -> Vec<u8> {
    assert_eq!(hex_string.len() & 1, 0);
    let result: Result<Vec<u8>, ParseIntError> = (0..hex_string.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_string[i..i + 2], 16))
        .collect();
    result.expect("Integer parse error: unable to parse byte")
}

pub mod base64 {
    static ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    static PADDING: char = '=';

    pub fn encode(bytes: &[u8]) -> String {
        let mut out = String::new();

        let encoder = |chunk: &[u8; 3]| {
            let n0 = (chunk[0] & 0b11111100) >> 2;
            let n1 = ((chunk[0] & 0b00000011) << 4) | ((chunk[1] & 0b11110000) >> 4);
            let n2 = ((chunk[1] & 0b00001111) << 2) | ((chunk[2] & 0b11000000) >> 6);
            let n3 = chunk[2] & 0b00111111;

            let n0 = n0 as usize;
            let n1 = n1 as usize;
            let n2 = n2 as usize;
            let n3 = n3 as usize;

            debug_assert!(n0 < ALPHABET.len());
            debug_assert!(n1 < ALPHABET.len());
            debug_assert!(n2 < ALPHABET.len());
            debug_assert!(n3 < ALPHABET.len());

            (n0, n1, n2, n3)
        };

        let (chunks, remainder) = bytes.as_chunks::<3>();
        for chunk in chunks {
            let (n0, n1, n2, n3) = encoder(chunk);
            out.push(char::from(ALPHABET[n0]));
            out.push(char::from(ALPHABET[n1]));
            out.push(char::from(ALPHABET[n2]));
            out.push(char::from(ALPHABET[n3]));
        }

        match *remainder {
            [] => (),
            [b0] => {
                let (n0, n1, _, _) = encoder(&[b0, 0, 0]);
                out.push(char::from(ALPHABET[n0]));
                out.push(char::from(ALPHABET[n1]));
                out.push(PADDING);
                out.push(PADDING);
            }
            [b0, b1] => {
                let (n0, n1, n2, _) = encoder(&[b0, b1, 0]);
                out.push(char::from(ALPHABET[n0]));
                out.push(char::from(ALPHABET[n1]));
                out.push(char::from(ALPHABET[n2]));
                out.push(PADDING);
            }
            _ => unreachable!(),
        }

        out
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_encode_padding_1() {
            let bytes = b"\x4D\x61";
            assert_eq!(encode(bytes), "TWE=");
        }

        #[test]
        fn test_encode_padding_2() {
            let bytes = b"\x4D";
            assert_eq!(encode(bytes), "TQ==");
        }
    }
}

pub fn vec_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());
    zip(a.iter(), b.iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<_>>()
}

pub(crate) fn vec_distance(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    assert!(!a.is_empty());
    zip(a, b)
        .map(|(a, b)| (a - b) * (a - b))
        .reduce(|acc, x| acc + x)
        .unwrap()
}

pub(crate) fn compute_characters_frequencies(m: &str) -> [f32; 255] {
    let mut frequencies = [0.0_f32; 255];
    for c in m.bytes() {
        frequencies[c as usize] += 1.0;
    }
    let count = m.len() as f32;
    for f in frequencies.iter_mut() {
        *f /= count;
    }
    frequencies
}

pub(crate) fn generate_reference_frequencies() -> [f32; 255] {
    let dictionary = fs::read_to_string("assets/english_words.txt")
        .unwrap()
        .replace("\n", " ");
    compute_characters_frequencies(&dictionary)
}
