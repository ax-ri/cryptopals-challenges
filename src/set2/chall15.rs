fn try_remove_pkcs_7_padding<const N: usize>(bytes: &[u8]) -> Result<&[u8], &str> {
    const {
        assert!(N > 0);
    }
    if bytes.len() != N {
        return Err("Block has not expected length");
    }

    let last_byte = *bytes.last().unwrap();
    let expected_padding_length = last_byte as usize;
    if expected_padding_length >= N {
        return Err("Expected padding length is too long");
    }

    let end_idx = N - expected_padding_length;
    for b in &bytes[end_idx..] {
        if *b != last_byte {
            return Err("Invalid padding");
        }
    }

    Ok(&bytes[..end_idx])
}

pub fn run() {
    assert_eq!(
        try_remove_pkcs_7_padding::<16>(b"ICE ICE BABY\x04\x04\x04\x04"),
        Ok(b"ICE ICE BABY" as &[u8])
    );

    assert!(try_remove_pkcs_7_padding::<16>(b"ICE ICE BABY\x05\x05\x05\x05").is_err());
    assert!(try_remove_pkcs_7_padding::<16>(b"ICE ICE BABY\x01\x02\x03\x04").is_err());
    assert!(try_remove_pkcs_7_padding::<16>(b"ICE ICE BABY\x01\x04\x04\x04").is_err());
}
