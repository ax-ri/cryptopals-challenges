use crate::utils;

pub fn run() {
    let a = utils::hex::string_to_bytes("1c0111001f010100061a024b53535009181c");
    let b = utils::hex::string_to_bytes("686974207468652062756c6c277320657965");
    assert_eq!(
        utils::vec_xor(&a, &b),
        utils::hex::string_to_bytes("746865206b696420646f6e277420706c6179")
    )
}
