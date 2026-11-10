use crate::set1::utils;

pub(crate) fn single_byte_xor_decrypt(
    ciphertext: &[u8],
    reference_frequencies: &[f32; 256],
) -> (u8, Vec<u8>, f32) {
    let (mut best_key, mut decrypted_bytes, mut minimal_error) = (0_u8, vec![], f32::MAX);
    for key in 0..=255_u8 {
        let key_msg = vec![key; ciphertext.len()];
        let decrypted_tentative = utils::vec_xor(ciphertext, &key_msg);
        let frequencies = utils::compute_characters_frequencies(&decrypted_tentative);
        let error = utils::vec_distance(&frequencies, reference_frequencies);

        if error < minimal_error {
            best_key = key;
            decrypted_bytes = decrypted_tentative;
            minimal_error = error;
        }
    }

    (best_key, decrypted_bytes, minimal_error)
}

pub fn run() {
    let reference_frequencies = utils::generate_reference_frequencies();

    let ciphertext = utils::hex::string_to_bytes(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
    );
    let (key, plaintext, _) = single_byte_xor_decrypt(&ciphertext, &reference_frequencies);
    assert_eq!(key, b'X');
    assert_eq!(plaintext, b"Cooking MC's like a pound of bacon");
}
