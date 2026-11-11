mod core;
mod mode;

pub(crate) use core::Key;

pub(crate) use mode::ecb;

pub(crate) fn pad_pkcs_7<const N: usize>(bytes: &[u8]) -> [u8; N] {
    debug_assert!(bytes.len() <= N);

    let padding_count = N - (bytes.len() % N);
    let mut padded = [padding_count as u8; N];
    padded[..bytes.len()].copy_from_slice(bytes);

    padded
}
