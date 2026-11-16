use crate::{set1, utils};
use rand::SeedableRng;

static UNKNOWN_STRING: &[u8; 184] = b"Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";

pub fn run() {
    let mut rng = rand::rngs::SmallRng::from_seed([0; 32]);

    let oracle = {
        let secret_key = utils::generate_random_key(&mut rng);
        let unknown_string = utils::base64::decode(UNKNOWN_STRING);

        move |bytes: &[u8]| {
            utils::aes::ecb::encrypt(&[bytes, &*unknown_string].concat(), &secret_key)
        }
    };

    // step 1
    let block_size = {
        let mut last_size = None;
        let mut n = 1;
        let block_size;
        loop {
            let encrypted = oracle(&vec![b'A'; n]);
            let new_size = encrypted.len();
            match last_size {
                None => last_size = Some(new_size),
                Some(last_size) => {
                    if new_size > last_size {
                        block_size = Some(encrypted.len() - last_size);
                        break;
                    }
                }
            }
            n += 1;
        }
        block_size.unwrap()
    };
    assert_eq!(block_size, 16);

    // step 2
    {
        assert!(set1::chall8::detect_aes_ecb(&oracle(&vec![
            b'A';
            block_size * 3
        ])))
    }

    // step 3
    {
        let block_count = oracle(&[]).len() / block_size;

        let mut deciphered_bytes = Vec::with_capacity(block_count * block_size);
        for block_idx in 0..block_count {
            let mut current_block = vec![b'A'; block_size];
            if block_idx > 0 {
                current_block[..(block_size - 1)].copy_from_slice(
                    &deciphered_bytes[(deciphered_bytes.len() - block_size + 1)..],
                );
            }

            for i in 1..=block_size {
                let oracle_output = oracle(&vec![b'A'; block_size - i]);
                let real_block = oracle_output
                    .chunks_exact(block_size)
                    .nth(block_idx)
                    .unwrap();

                for b in 0..=255 {
                    current_block[block_size - 1] = b;
                    let out = oracle(&current_block);
                    if &out[..block_size] == real_block {
                        current_block.rotate_left(1);
                        deciphered_bytes.push(b);
                        break;
                    }
                }
            }
        }
        deciphered_bytes.pop(); // remove last byte, which is padding

        assert_eq!(deciphered_bytes, utils::base64::decode(UNKNOWN_STRING));
    }
}
