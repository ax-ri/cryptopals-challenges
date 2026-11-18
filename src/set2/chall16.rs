use crate::utils;
use crate::utils::static_array_xor;
use rand::{Rng, RngExt, SeedableRng};

fn make_token_forge_and_checker(
    rng: &mut impl Rng,
) -> (impl Fn(&[u8]) -> Vec<u8>, impl Fn(&[u8]) -> bool) {
    let key = rng.random();
    let iv = rng.random();

    let create_token = move |user_data: &[u8]| -> Vec<u8> {
        let user_data = utils::url_encoding::encode(user_data);

        let input = [
            b"comment1=cooking%20MCs;userdata=" as &[u8],
            &user_data,
            b";comment2=%20like%20a%20pound%20of%20bacon" as &[u8],
        ]
        .concat();

        utils::aes::cbc::encrypt(&input, &key, &iv)
    };

    let check_token = move |token: &[u8]| -> bool {
        let decrypted = utils::aes::cbc::decrypt(token, &key, &iv);
        match decrypted {
            Err(_) => false,
            Ok(decrypted) => {
                let found = decrypted
                    .split(|b| *b == b';')
                    .find(|s| *s == b"admin=true");
                found.is_some()
            }
        }
    };

    (create_token, check_token)
}

pub fn run() {
    let mut rng = rand::rngs::SmallRng::from_seed([0; 32]);

    let (create_token, check_token) = make_token_forge_and_checker(&mut rng);

    assert!(!check_token(&create_token(b"hello")));
    assert!(!check_token(&create_token(b";admin=true;")));
    assert!(!check_token(&create_token(b"admin=true")));

    let target_plaintext = utils::aes::pad_pkcs_7_static::<16>(b";admin=true;");
    let encoded_target_plaintext = utils::aes::pad_pkcs_7_static::<16>(b"%3Badmin%3Dtrue%");

    let encrypted_blocks = create_token(&[[b'A'; 16], target_plaintext].concat());
    assert!(!check_token(&encrypted_blocks));

    let (encrypted_blocks, _) = encrypted_blocks.as_chunks::<16>();

    let e2_prime = static_array_xor(
        &encrypted_blocks[2],
        &static_array_xor(&target_plaintext, &encoded_target_plaintext),
    );

    let mut forged_blocks = Vec::from(encrypted_blocks);
    forged_blocks[2] = e2_prime;

    assert!(check_token(&forged_blocks.concat()));
}
