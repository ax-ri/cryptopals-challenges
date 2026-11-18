//! Hex strings utility functions.

use std::num::ParseIntError;

// from https://stackoverflow.com/a/52992629
pub fn string_to_bytes(hex_string: &str) -> Vec<u8> {
    assert_eq!(hex_string.len() & 1, 0);
    let result: Result<Vec<u8>, ParseIntError> = (0..hex_string.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_string[i..i + 2], 16))
        .collect();
    result.expect("Integer parse error: unable to parse byte")
}
