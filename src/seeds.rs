use std::{iter::Map, ops::Range};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

const ALPHABET: [u8; 36] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
    b'W', b'X', b'Y', b'Z',
];

const SEED_LENGTH: usize = 8;

// pub struct SeedGenerator {
//     current: u64,
//     max: u64,
// }

fn idx_to_seed(mut i: usize) -> String {
    let mut seed = String::with_capacity(SEED_LENGTH);

    for _ in 0..SEED_LENGTH {
        let rem: usize;
        (i, rem) = (i / 36, i % 36);
        seed.push(ALPHABET[rem as usize] as char);
    }

    seed
}

// impl Iterator for SeedGenerator {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.current >= self.max {
//             return None;
//         }

//         let seed = idx_to_seed(self.current);

//         self.current += 1;
//         Some(seed)
//     }
// }

// impl Default for SeedGenerator {
//     fn default() -> Self {
//         SeedGenerator {
//             current: 0,
//             max: 36u64.pow(SEED_LENGTH as u32),
//         }
//     }
// }

// impl ParallelIterator for SeedGenerator {
//     type Item = String;

//     fn drive_unindexed<C>(self, consumer: C) -> C::Result
//     where
//         C: rayon::iter::plumbing::Consumer<Self::Item>,
//     {
//         bridge(self, consumer)
//     }
// }

// impl IndexedParallelIterator for SeedGenerator {
//     fn len(&self) -> usize {
//         (self.max - self.current - 1) as usize
//     }

//     fn drive<C: rayon::iter::plumbing::Consumer<Self::Item>>(self, consumer: C) -> C::Result {
//         bridge(self, consumer)
//     }

//     fn with_producer<CB: rayon::iter::plumbing::ProducerCallback<Self::Item>>(
//         self,
//         callback: CB,
//     ) -> CB::Output {
//         callback.callback(SeedProducer { generator: self })
//     }
// }

// impl ExactSizeIterator for SeedGenerator {}

// struct SeedProducer {
//     generator: SeedGenerator,
// }

// impl Producer for SeedProducer {
//     type Item = String;

//     fn split_at(self, index: usize) -> (Self, Self) {
//         debug_assert!(index <= self.generator.max as usize);

//         let mid = self.generator.current + index as u64;
//         let left = SeedGenerator {
//             current: self.generator.current,
//             max: mid,
//         };
//         let right = SeedGenerator {
//             current: mid,
//             max: self.generator.max,
//         };

//         (
//             SeedProducer { generator: left },
//             SeedProducer { generator: right },
//         )
//     }

//     type IntoIter = SeedGenerator;

//     fn into_iter(self) -> Self::IntoIter {
//         self.generator
//     }
// }

// impl DoubleEndedIterator for SeedGenerator {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         if self.current >= self.max {
//             return None;
//         }

//         let seed = idx_to_seed(self.max - 1);

//         self.max -= 1;
//         Some(seed)
//     }
// }

pub fn seeds() -> Map<Range<usize>, fn(usize) -> String> {
    (0..usize::pow(36, SEED_LENGTH as u32)).map(idx_to_seed)
}

pub fn par_seeds() -> rayon::iter::Map<rayon::range::Iter<usize>, fn(usize) -> String> {
    (0..usize::pow(36, SEED_LENGTH as u32))
        .into_par_iter()
        .map(idx_to_seed)
}

#[cfg(test)]
mod tests {
    use rayon::iter::IntoParallelIterator;

    use super::*;

    #[test]
    fn test_seed_generator() {
        {
            let seeds = seeds().take(40);

            let expected = vec![
                "00000000", "10000000", "20000000", "30000000", "40000000", "50000000", "60000000",
                "70000000", "80000000", "90000000", "A0000000", "B0000000", "C0000000", "D0000000",
                "E0000000", "F0000000", "G0000000", "H0000000", "I0000000", "J0000000", "K0000000",
                "L0000000", "M0000000", "N0000000", "O0000000", "P0000000", "Q0000000", "R0000000",
                "S0000000", "T0000000", "U0000000", "V0000000", "W0000000", "X0000000", "Y0000000",
                "Z0000000", "01000000", "11000000", "21000000", "31000000",
            ];
            assert_eq!(seeds.collect::<Vec<_>>(), expected);
        }

        assert_eq!(seeds().len(), 36u64.pow(SEED_LENGTH as u32) as usize);
        assert_eq!(seeds().next_back(), Some("ZZZZZZZZ".to_string()));
    }
}
