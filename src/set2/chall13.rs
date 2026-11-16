use crate::utils;
use crate::utils::aes::Key;
use rand::SeedableRng;

fn profile_for(key: &Key, email: &[u8]) -> Vec<u8> {
    if email.contains(&b'=') || email.contains(&b'&') {
        panic!("Email cannot contain metacharacters");
    }
    let cookie = [b"email=", email, b"&uid=10&role=user"].concat();
    utils::aes::ecb::encrypt(&cookie, key)
}

fn read_profile(key: &Key, profile: &[u8]) -> Result<Vec<String>, String> {
    let decrypted = utils::aes::ecb::decrypt(profile, key);
    match decrypted {
        Err(e) => Err(format!("Invalid profile: {:?}", e)),
        Ok(bytes) => Ok(bytes
            .split(|c| *c == b'&')
            .map(|s| String::from_utf8(Vec::from(s)).unwrap())
            .collect::<Vec<_>>()),
    }
}

pub fn run() {
    let (profile_for, read_profile) = {
        let mut rng = rand::rngs::SmallRng::from_seed([0; 32]);
        let key = utils::generate_random_key(&mut rng);
        (
            move |email| profile_for(&key, email),
            move |profile: &[u8]| read_profile(&key, profile),
        )
    };

    {
        let profile = profile_for(b"foo@bar.com");
        assert_eq!(
            read_profile(&profile),
            Ok(vec![
                String::from("email=foo@bar.com"),
                String::from("uid=10"),
                String::from("role=user")
            ])
        );
    }

    {
        let block_size = 16;

        let padding_length = block_size - ("email=".len());
        let input = [
            vec![b'b'; padding_length],
            utils::aes::pad_pkcs_7_dynamic(block_size, b"admin"),
        ]
        .concat();
        let admin_block = &profile_for(&input)[block_size..(2 * block_size)];
        debug_assert_eq!(admin_block.len(), block_size);

        let additional_text_len = "email=".len() + "&uid=10&role=".len();
        let required_email_length =
            (additional_text_len / block_size + 1) * block_size - additional_text_len;
        let email = vec![b'a'; required_email_length];
        let profile = profile_for(&email);
        assert_eq!(
            read_profile(&profile),
            Ok(vec![
                String::from("email=aaaaaaaaaaaaa"),
                String::from("uid=10"),
                String::from("role=user")
            ])
        );

        let altered_profile = [
            &profile[..(required_email_length + additional_text_len)],
            admin_block,
        ]
        .concat();
        assert_eq!(
            read_profile(&altered_profile),
            Ok(vec![
                String::from("email=aaaaaaaaaaaaa"),
                String::from("uid=10"),
                String::from("role=admin")
            ])
        );
    }
}
