use crate::set1::chall3;
use crate::utils;
use std::fs;

fn find_key_size(ciphertext: &[u8], max_size: usize, m: usize) -> usize {
    let mut minimal_mean = f32::MAX;
    let mut best_key_size = 0;

    for key_size in 2..max_size {
        let chunks = ciphertext.chunks_exact(key_size).collect::<Vec<_>>();

        let global_mean =
      // for the m first chunks, compute the mean Hamming distance between them and the other chunks
      ((0..m)
        .map(|idx| {
          let ref_chunk = chunks[idx];
          chunks
            .iter()
            .map(|chunk| utils::hamming_distance(ref_chunk, chunk) as f32)
            .sum::<f32>()
            / (chunks.len() - 1) as f32
        })
        // then compute the mean of these m means
        .sum::<f32>()
        / (m as f32))
        // and finally normalize by the key size
        / key_size as f32;

        // compute the minimal mean
        if global_mean < minimal_mean {
            minimal_mean = global_mean;
            best_key_size = key_size;
        }
    }

    best_key_size
}

pub(crate) fn run() {
    let content = fs::read_to_string("assets/chall_1.6.txt")
        .unwrap()
        .split("\n")
        .take(10)
        .collect::<Vec<_>>()
        .join("");
    let ciphertext = utils::base64::decode(content.as_bytes());
    let key_size = find_key_size(&ciphertext, 40, 4);
    assert_eq!(key_size, 29);

    let cipher_chunks = ciphertext.chunks_exact(key_size).collect::<Vec<_>>();
    let chunk_count = cipher_chunks.len();

    let single_byte_blocks = (0..key_size)
        .map(|idx| {
            cipher_chunks
                .iter()
                .map(|chunk| chunk[idx])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let reference_frequencies = utils::generate_reference_frequencies();

    let whole_key = single_byte_blocks
        .into_iter()
        .map(|block| {
            let (single_key, _, _) =
                chall3::single_byte_xor_decrypt(&block, &reference_frequencies);
            single_key
        })
        .collect::<Vec<_>>();

    // TODO find out why we find 'nodse' and not 'noise'
    assert_eq!(whole_key, b"Terminator X: Bring the nodse");

    let mut repeated_key = whole_key.repeat(chunk_count);
    repeated_key.append(&mut Vec::from(&whole_key[..(ciphertext.len() % key_size)]));
    assert_eq!(repeated_key.len(), ciphertext.len());

    let result = utils::vec_xor(&ciphertext, &repeated_key);
    let _ = String::from_utf8(result);
}
