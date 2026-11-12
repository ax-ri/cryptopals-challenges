use crate::{set1, utils};
use rand::{Rng, RngExt, SeedableRng};

fn encryption_oracle(bytes: &[u8], rng: &mut impl Rng) -> (Vec<u8>, bool) {
    let (count_before, count_after) = (rng.random_range(5..=10), rng.random_range(5..=10));
    let (mut bytes_before, mut bytes_after) = (vec![0; count_before], vec![0; count_after]);
    rng.fill(&mut bytes_before);
    rng.fill(&mut bytes_after);
    let bytes = [&bytes_before, bytes, &bytes_after].concat();

    let use_ecb = rng.random_bool(0.5);
    let key = utils::generate_random_key(rng);
    let encrypted = {
        if use_ecb {
            utils::aes::ecb::encrypt(&bytes, &key)
        } else {
            let iv = rng.random::<[u8; 16]>();
            utils::aes::cbc::encrypt(&bytes, &key, &iv)
        }
    };
    (encrypted, use_ecb)
}

fn detect_mode(bytes: &[u8]) -> bool {
    set1::chall8::detect_aes_ecb(bytes)
}

pub fn run() {
    let mut rng = rand::rngs::SmallRng::from_seed([0; 32]);

    for _ in 0..50 {
        let plaintext = [rng.random::<u8>(); 3 * 16];
        let (ciphertext, used_mode) = encryption_oracle(&plaintext, &mut rng);
        let detected_mode = detect_mode(&ciphertext);
        assert_eq!(detected_mode, used_mode);
    }
}
