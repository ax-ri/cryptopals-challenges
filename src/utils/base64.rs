//! Base64 encoding and decoding.

static ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
static ALPHABET_REVERSE: &[u8; 256] = &[
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 62, 64, 64, 64, 63,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 64, 64, 64, 64, 64, 64, 64, 0, 1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 64, 64, 64, 64, 64, 64, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
];
static PADDING: u8 = b'=';

pub fn encode(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len());

    let encoder = |chunk: &[u8; 3]| {
        let n0 = chunk[0] >> 2;
        let n1 = ((chunk[0] & 0b00000011) << 4) | (chunk[1] >> 4);
        let n2 = ((chunk[1] & 0b00001111) << 2) | (chunk[2] >> 6);
        let n3 = chunk[2] & 0b00111111;

        let n0 = n0 as usize;
        let n1 = n1 as usize;
        let n2 = n2 as usize;
        let n3 = n3 as usize;

        debug_assert!(n0 < ALPHABET.len());
        debug_assert!(n1 < ALPHABET.len());
        debug_assert!(n2 < ALPHABET.len());
        debug_assert!(n3 < ALPHABET.len());

        (n0, n1, n2, n3)
    };

    let (chunks, remainder) = bytes.as_chunks::<3>();
    for chunk in chunks {
        let (n0, n1, n2, n3) = encoder(chunk);
        out.push(ALPHABET[n0]);
        out.push(ALPHABET[n1]);
        out.push(ALPHABET[n2]);
        out.push(ALPHABET[n3]);
    }

    match *remainder {
        [] => (),
        [b0] => {
            let (n0, n1, _, _) = encoder(&[b0, 0, 0]);
            out.push(ALPHABET[n0]);
            out.push(ALPHABET[n1]);
            out.push(PADDING);
            out.push(PADDING);
        }
        [b0, b1] => {
            let (n0, n1, n2, _) = encoder(&[b0, b1, 0]);
            out.push(ALPHABET[n0]);
            out.push(ALPHABET[n1]);
            out.push(ALPHABET[n2]);
            out.push(PADDING);
        }
        _ => unreachable!(),
    }

    out
}

pub fn decode(bytes: &[u8]) -> Vec<u8> {
    if bytes.is_empty() {
        return vec![];
    }

    let mut out = Vec::with_capacity(bytes.len());

    let idx_converter = |b: u8| {
        let idx = ALPHABET_REVERSE[b as usize];
        debug_assert!(idx as usize != ALPHABET.len());
        idx
    };
    let decoder = |i0: u8, i1: u8, i2: u8, i3: u8| {
        let n0 = (i0 << 2) | ((i1 & 0b00110000) >> 4);
        let n1 = (i1 << 4) | ((i2 & 0b00111100) >> 2);
        let n2 = (i2 << 6) | (i3 & 0b00111111);
        (n0, n1, n2)
    };

    let (chunks, remainder) = bytes.as_chunks::<4>();
    debug_assert!(remainder.is_empty());

    for chunk in chunks {
        let (i0, i1, i2, i3) = (
            idx_converter(chunk[0]),
            idx_converter(chunk[1]),
            if chunk[2] == PADDING {
                0
            } else {
                idx_converter(chunk[2])
            },
            if chunk[3] == PADDING {
                0
            } else {
                idx_converter(chunk[3])
            },
        );
        let (n0, n1, n2) = decoder(i0, i1, i2, i3);
        out.push(n0);
        if chunk[2] != PADDING {
            out.push(n1);
        }
        if chunk[3] != PADDING {
            out.push(n2);
        }
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode(b""), b"");
        assert_eq!(encode(b"f"), b"Zg==");
        assert_eq!(encode(b"fo"), b"Zm8=");
        assert_eq!(encode(b"foo"), b"Zm9v");
        assert_eq!(encode(b"foob"), b"Zm9vYg==");
        assert_eq!(encode(b"fooba"), b"Zm9vYmE=");
        assert_eq!(encode(b"foobar"), b"Zm9vYmFy");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode(b""), b"");
        assert_eq!(decode(b"Zg=="), b"f");
        assert_eq!(decode(b"Zm8="), b"fo");
        assert_eq!(decode(b"Zm9v"), b"foo");
        assert_eq!(decode(b"Zm9vYg=="), b"foob");
        assert_eq!(decode(b"Zm9vYmE="), b"fooba");
        assert_eq!(decode(b"Zm9vYmFy"), b"foobar");
    }
}
