use crate::utils;

pub(crate) fn repeating_key_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    let mut cur_key_idx = 0;

    plaintext
        .iter()
        .map(|b| {
            let k = key[cur_key_idx];
            cur_key_idx += 1;
            if cur_key_idx == key.len() {
                cur_key_idx = 0;
            }
            b ^ k
        })
        .collect::<Vec<_>>()
}

pub fn run() {
    let result = repeating_key_xor(
        b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
        b"ICE",
    );
    assert_eq!(
        result,
        utils::hex::string_to_bytes(
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        )
    )
}
