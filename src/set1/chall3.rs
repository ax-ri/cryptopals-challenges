use crate::set1::utils;
use std::fs;

fn single_byte_xor_decrypt(ciphertext: &[u8]) -> (char, String) {
    let reference_frequencies = {
        let dictionary = fs::read_to_string("assets/english_words.txt")
            .unwrap()
            .replace("\n", " ");
        utils::compute_characters_frequencies(&dictionary)
    };

    let (mut minimal_score, mut best_key) = (f32::MAX, 0_u8);
    for key in 0..255_u8 {
        let key_msg = vec![key; ciphertext.len()];
        let decrypted = utils::vec_xor(ciphertext, &key_msg);
        if let Ok(decoded) = String::from_utf8(decrypted) {
            let frequencies = utils::compute_characters_frequencies(&decoded);
            let score = utils::vec_distance(&frequencies, &reference_frequencies);
            if score < minimal_score {
                minimal_score = score;
                best_key = key;
            }
        }
    }

    let key_msg = vec![best_key; ciphertext.len()];
    let decrypted = String::from_utf8(utils::vec_xor(ciphertext, &key_msg)).unwrap();

    (char::from(best_key), decrypted)
}

pub fn run() {
    let ciphertext =
    b"\x1b\x37\x37\x33\x31\x36\x3f\x78\x15\x1b\x7f\x2b\x78\x34\x31\x33\x3d\x78\x39\x78\x28\x37\x2d\x36\x3c\x78\x37\x3e\x78\x3a\x39\x3b\x37\x36";
    let (key, plaintext) = single_byte_xor_decrypt(ciphertext);
    assert_eq!(key, 'X');
    assert_eq!(plaintext, "Cooking MC's like a pound of bacon");
}
