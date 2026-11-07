use crate::set1::utils;
use crate::set1::utils::hex_string_to_bytes;

pub(crate) fn single_byte_xor_decrypt(
    ciphertext: &[u8],
    reference_frequencies: &[f32; 255],
) -> Result<(char, String, f32), ()> {
    let (mut minimal_error, mut best_key) = (f32::MAX, 0_u8);
    for key in 0..255_u8 {
        let key_msg = vec![key; ciphertext.len()];
        let decrypted = utils::vec_xor(ciphertext, &key_msg);
        if let Ok(decoded) = String::from_utf8(decrypted) {
            let frequencies = utils::compute_characters_frequencies(&decoded);
            let error = utils::vec_distance(&frequencies, reference_frequencies);
            if error < minimal_error {
                minimal_error = error;
                best_key = key;
            }
        }
    }

    let key_msg = vec![best_key; ciphertext.len()];
    if let Ok(decrypted) = String::from_utf8(utils::vec_xor(ciphertext, &key_msg)) {
        Ok((char::from(best_key), decrypted, minimal_error))
    } else {
        Err(())
    }
}

pub fn run() {
    let reference_frequencies = utils::generate_reference_frequencies();

    let ciphertext =
        hex_string_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let (key, plaintext, _) = single_byte_xor_decrypt(&ciphertext, &reference_frequencies).unwrap();
    assert_eq!(key, 'X');
    assert_eq!(plaintext, "Cooking MC's like a pound of bacon");
}
