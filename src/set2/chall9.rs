use crate::utils;

pub fn run() {
    assert_eq!(
        &utils::aes::pad_pkcs_7_static::<20>(b"YELLOW SUBMARINE"),
        b"YELLOW SUBMARINE\x04\x04\x04\x04"
    );
}
