use std::{
    iter::{Map, repeat_with},
    ops::Range,
    slice,
};

use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Seed {
    bytes: [u8; SEED_LENGTH],
    len: usize,
}

impl Seed {
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.bytes.as_ptr(), self.len) }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_slice()) }
    }

    /// Converts an index in the range [0, SEED_COUNT) to a Seed.
    pub fn from_index(mut index: usize) -> Self {
        assert!(
            index < SEED_COUNT,
            "Index {index} out of range for SEED_COUNT"
        );

        // Determine the length by iterating through possible lengths
        let mut len = 0;
        let mut remaining = index;
        for l in 0..=SEED_LENGTH {
            let count: usize = ALPHABET_SIZE.pow(l as u32);
            if remaining < count {
                len = l;
                break;
            }
            remaining -= count;
        }

        // Generate the bytes for the seed
        let mut bytes = [0; SEED_LENGTH];
        for i in (0..len).rev() {
            bytes[i] = ALPHABET[remaining % ALPHABET_SIZE] as u8;
            remaining /= ALPHABET_SIZE;
        }
        assert_eq!(
            remaining, 0,
            "Remaining index {remaining} should be 0 after conversion from {index}"
        );

        Seed { len, bytes }
    }

    /// Converts a Seed back to its corresponding index in the range [0, SEED_COUNT).
    pub fn to_index(&self) -> usize {
        let mut index = 0;

        // Encode the bytes into the index
        for &byte in &self.bytes[..self.len] {
            index = index * ALPHABET_SIZE + ALPHABET.iter().position(|&b| b == byte).unwrap();
        }

        for l in 0..self.len {
            index += ALPHABET_SIZE.pow(l as u32);
        }

        index
    }
}

// 49..=57, 65..=90
const ALPHABET: [u8; 35] = [
    b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F', b'G',
    b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W',
    b'X', b'Y', b'Z',
];
const ALPHABET_SIZE: usize = ALPHABET.len();

const SEED_LENGTH: usize = 8;

pub const SEED_COUNT: usize =
    (ALPHABET_SIZE as u64).pow(SEED_LENGTH as u32 + 1) as usize / (ALPHABET_SIZE as usize - 1);

static_assertions::const_assert_eq!(SEED_COUNT, 2_318_107_019_761);

impl From<&str> for Seed {
    fn from(seed: &str) -> Self {
        let bytes = seed.as_bytes();
        let len = bytes.len();

        assert!(
            len <= SEED_LENGTH,
            "Seed length exceeds maximum length of {SEED_LENGTH}"
        );

        let mut seed_bytes = [0; SEED_LENGTH];
        for i in 0..len {
            let c = bytes[i];
            assert!(ALPHABET.contains(&c), "Invalid character in seed: {c}");
            seed_bytes[i] = c;
        }

        Seed {
            bytes: seed_bytes,
            len,
        }
    }
}

// pub struct SeedGenerator {
//     current: u64,
//     max: u64,
// }

fn idx_to_seed(mut i: usize) -> String {
    let mut seed = String::with_capacity(SEED_LENGTH);

    for _ in 0..SEED_LENGTH {
        let rem: usize;
        (i, rem) = (i / ALPHABET_SIZE, i % ALPHABET_SIZE);
        seed.push(ALPHABET[rem] as char);
    }
    assert_eq!(i, 0);

    seed
}

pub fn seeds() -> Map<Range<usize>, fn(usize) -> String> {
    (0..usize::pow(ALPHABET_SIZE, SEED_LENGTH as u32)).map(idx_to_seed)
}

pub fn par_seeds() -> rayon::iter::Map<rayon::range::Iter<usize>, fn(usize) -> String> {
    (0..usize::pow(ALPHABET_SIZE, SEED_LENGTH as u32))
        .into_par_iter()
        .map(idx_to_seed)
}

pub fn par_random_seeds() -> impl rayon::iter::ParallelIterator<Item = String> {
    repeat_with(|| rand::random_range(0..usize::pow(ALPHABET_SIZE, SEED_LENGTH as u32)))
        .par_bridge()
        .map(idx_to_seed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_conversion() {
        for i in 0..10000 {
            let seed = Seed::from_index(i);
            assert_eq!(
                seed.to_index(),
                i,
                "Failed to convert seed {i} ({seed:?} {:?}) to index",
                seed.as_str()
            );
        }
        for i in SEED_COUNT - 10000..SEED_COUNT {
            let seed = Seed::from_index(i);
            assert_eq!(
                seed.to_index(),
                i,
                "Failed to convert seed {i} ({seed:?} {:?}) to index",
                seed.as_str()
            );
        }
    }

    #[test]
    #[should_panic]
    fn test_seed_from_index_out_of_bounds() {
        Seed::from_index(SEED_COUNT);
    }

    #[test]
    fn test_seed_from_str() {
        let seed = Seed::from("TESTSEED");
        assert_eq!(
            seed.as_str(),
            "TESTSEED",
            "Failed to convert from str: {:?}",
            seed
        );
    }

    #[test]
    fn test_seed_index_9() {
        let seed = Seed::from_index(9);
        assert_eq!(seed.to_index(), 9, "Failed for index 9: {:?}", seed);
    }
}
