//! Tool functions that are useful in several places

pub mod aes;
pub mod base64;
pub mod hex;

use rand::{Rng, RngExt};
use std::fs;
use std::iter::zip;

pub fn vec_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());
    zip(a.iter(), b.iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<_>>()
}

pub fn static_array_xor<const N: usize>(a: &[u8; N], b: &[u8; N]) -> [u8; N] {
    zip(a.iter(), b.iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub(crate) fn vec_distance(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    assert!(!a.is_empty());
    zip(a, b).map(|(a, b)| (a - b) * (a - b)).sum::<f32>()
}

pub(crate) fn compute_characters_frequencies(m: &[u8]) -> [f32; 256] {
    let mut frequencies = [0.0_f32; 256];
    for c in m {
        frequencies[*c as usize] += 1.0;
    }
    let count = m.len() as f32;
    for f in frequencies.iter_mut() {
        *f /= count;
    }
    frequencies
}

pub(crate) fn generate_reference_frequencies() -> [f32; 256] {
    let dictionary = fs::read_to_string("assets/english_words.txt")
        .unwrap()
        .replace("\n", " ");
    compute_characters_frequencies(dictionary.as_bytes())
}

pub(crate) fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    assert_eq!(a.len(), b.len());
    if a.is_empty() {
        return 0;
    }
    zip(a, b).map(|(a, b)| (*a ^ *b).count_ones()).sum()
}

pub(crate) fn generate_random_key(rng: &mut impl Rng) -> aes::Key {
    rng.random()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance(b"", b""), 0);
        assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37);
    }
}
