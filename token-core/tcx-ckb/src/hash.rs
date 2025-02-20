use blake2b_rs::{Blake2b, Blake2bBuilder};

pub const CKB_HASH_PERSONALIZATION: &[u8] = b"ckb-default-hash";
pub const BLANK_HASH: [u8; 32] = [
    68, 244, 198, 151, 68, 213, 248, 197, 93, 100, 32, 98, 148, 157, 202, 228, 155, 196, 231, 239,
    67, 211, 136, 197, 161, 47, 66, 181, 99, 61, 22, 62,
];

pub fn new_blake2b() -> Blake2b {
    Blake2bBuilder::new(32)
        .personal(CKB_HASH_PERSONALIZATION)
        .build()
}

pub fn blake2b_256<T: AsRef<[u8]>>(s: T) -> Vec<u8> {
    if s.as_ref().is_empty() {
        return BLANK_HASH.to_vec();
    }

    inner_blake2b_256(s).to_vec()
}

pub fn blake2b_160<T: AsRef<[u8]>>(s: T) -> Vec<u8> {
    if s.as_ref().is_empty() {
        return BLANK_HASH[..20].to_vec();
    }

    inner_blake2b_256(s)[..20].to_vec()
}

fn inner_blake2b_256<T: AsRef<[u8]>>(s: T) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut blake2b = new_blake2b();
    blake2b.update(s.as_ref());
    blake2b.finalize(&mut result);
    result
}

#[cfg(test)]
mod test {
    use crate::hash::{blake2b_160, blake2b_256};
    #[test]
    fn test_blake2b_256_param_is_empty() {
        let hash = blake2b_256(vec![]);
        assert_eq!(
            hash,
            [
                68, 244, 198, 151, 68, 213, 248, 197, 93, 100, 32, 98, 148, 157, 202, 228, 155,
                196, 231, 239, 67, 211, 136, 197, 161, 47, 66, 181, 99, 61, 22, 62
            ]
        );
    }

    #[test]
    fn test_blake2b_160_param_is_empty() {
        let hash = blake2b_160(vec![]);
        assert_eq!(
            hash,
            [
                68, 244, 198, 151, 68, 213, 248, 197, 93, 100, 32, 98, 148, 157, 202, 228, 155,
                196, 231, 239
            ]
        );
    }
}
