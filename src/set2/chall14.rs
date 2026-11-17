use crate::{set2, utils};
use rand::{Rng, RngExt, SeedableRng};
use std::iter::zip;

static UNKNOWN_STRING: &[u8; 184] = b"Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";

fn make_oracle(rng: &mut impl Rng) -> (impl Fn(&[u8]) -> Vec<u8>, usize) {
    let prefix_length = rng.random_range(5..128);
    let mut prefix = vec![0_u8; prefix_length];
    rng.fill(&mut prefix);
    let secret_key = utils::generate_random_key(rng);
    let unknown_string = utils::base64::decode(UNKNOWN_STRING);

    (
        move |bytes: &[u8]| {
            utils::aes::ecb::encrypt(&[&prefix, bytes, &*unknown_string].concat(), &secret_key)
        },
        prefix_length,
    )
}

pub fn run() {
    let mut rng = rand::rngs::SmallRng::from_seed([78; 32]);

    let (oracle, secret_prefix_length) = make_oracle(&mut rng);

    let block_size = 16;

    let block_idx = {
        let blocks_0 = oracle(&[]);
        let blocks_1 = oracle(b"A");

        let idx = zip(
            blocks_0.chunks_exact(block_size),
            blocks_1.chunks_exact(block_size),
        )
        .position(|(b0, b1)| b0 != b1);
        match idx {
            None => 0,
            Some(idx) => idx + 1,
        }
    };

    let prefix_len = {
        let mut prefix_len = 0;

        let similar_section_len = block_idx * block_size;
        let mut last_output = vec![];
        for length in 0..100 {
            let input = vec![b'A'; length];
            let output = oracle(&input);
            if (output.len() >= similar_section_len && last_output.len() >= similar_section_len)
                && (output[..similar_section_len] == last_output[..similar_section_len])
            {
                prefix_len = similar_section_len - length + 1;
                break;
            }
            last_output = output;
        }
        prefix_len
    };

    assert_eq!(prefix_len, secret_prefix_length);

    let deciphered_bytes = {
        // make a new oracle that adds a prefix to the input so that the random prefix is in a fixed number of blocks
        // and so the input bytes start in a fresh block
        // then truncate the oracle output, by removing the first N blocks
        // that correspond to the encrypted random prefix + prefix
        let oracle = |bytes: &[u8]| {
            let len = block_size - (prefix_len % block_size);
            let output = oracle(&[&vec![b'A'; len], bytes].concat());
            Vec::from(&output[(block_idx * block_size)..])
        };
        set2::chall12::break_ecb_byte_at_a_time(block_size, &oracle)
    };

    assert_eq!(deciphered_bytes, utils::base64::decode(UNKNOWN_STRING));
}
