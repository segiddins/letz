use std::{
    iter::{Map, repeat_with},
    ops::Range,
};

use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

const ALPHABET: [u8; 35] = [
    b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F', b'G',
    b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W',
    b'X', b'Y', b'Z',
];
const ALPHABET_SIZE: usize = ALPHABET.len();

const SEED_LENGTH: usize = 8;

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
