use crate::set1::chall3::single_byte_xor_decrypt;
use crate::set1::utils;
use std::fs;

pub fn run() {
    let reference_frequencies = utils::generate_reference_frequencies();

    let mut best_error = f32::MAX;
    let mut decrypted_line = String::new();

    for line in fs::read_to_string("assets/chall_1.4.txt")
        .unwrap()
        .split("\n")
        .map(utils::hex::string_to_bytes)
    {
        if let Ok((_, decrypted, minimal_error)) =
            single_byte_xor_decrypt(&line, &reference_frequencies)
            && minimal_error < best_error
        {
            best_error = minimal_error;
            decrypted_line = decrypted;
        }
    }
    assert_eq!(decrypted_line, "Now that the party is jumping\n");
}
