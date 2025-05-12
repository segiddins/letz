use std::{collections::HashMap, f64};

use crate::{game, lua_random::LuaRandom};

#[derive(Debug, Clone)]
pub struct Rng<'a> {
    pub(crate) seed: &'a str,
    hashed_seed: f64,
    states: HashMap<Key, f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum KeyPart {
    #[default]
    Empty,
    Ante(i8),
    Source(game::Source),
    Type(game::Type),
    Resample(u8),
}

const ANTE_STRS: [&str; 52] = [
    "-1", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
    "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
    "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47",
    "48", "49", "50",
];

// impl AsRef<str> for KeyPart<'_> {
//     #[inline]
//     fn as_ref(&self) -> &str {
//         match self {
//             KeyPart::Empty => "",
//             KeyPart::Ante(i) => {
//                 assert!(*i >= -1 && *i < ANTE_STRS.len() as i8 - 1);
//                 ANTE_STRS[*i as usize + 1]
//             }
//             KeyPart::Source(source) => source.as_ref(),
//             KeyPart::Type(typ) => typ.as_ref(),
//             KeyPart::Resample(i) => {
//                 assert!(
//                     *i < RESAMPLE_STRS.len() as u8,
//                     "Resample index {i} out of bounds"
//                 );
//                 RESAMPLE_STRS[*i as usize]
//             }
//             KeyPart::Seed(seed) => seed,
//         }
//     }
// }

impl KeyPart {
    #[inline]
    const fn len(&self) -> usize {
        match self {
            KeyPart::Empty => 0,
            KeyPart::Ante(i) => {
                debug_assert!(*i >= -1 && *i < ANTE_STRS.len() as i8 - 1);
                ANTE_STRS[*i as usize + 1].len()
            }
            KeyPart::Source(source) => source.into_str().len(),
            KeyPart::Type(typ) => typ.into_str().len(),
            KeyPart::Resample(i) => "_resample".len() + ANTE_STRS[*i as usize + 3].len(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Key([KeyPart; 4]);

impl Into<KeyPart> for game::Source {
    fn into(self) -> KeyPart {
        KeyPart::Source(self)
    }
}

impl Into<KeyPart> for game::Type {
    fn into(self) -> KeyPart {
        KeyPart::Type(self)
    }
}

impl Key {
    pub fn resample(&mut self, i: u8) {
        #[cfg(debug_assertions)]
        match self.0[3] {
            KeyPart::Empty | KeyPart::Resample(_) => {}
            _ => {
                panic!(
                    "Attempting to resample a key that is not empty or resampled: {:?}",
                    self
                );
            }
        }
        self.0[3] = KeyPart::Resample(i);
    }

    pub const fn len(&self) -> usize {
        self.0[0].len() + self.0[1].len() + self.0[2].len() + self.0[3].len()
    }

    // Example: "test_seed" -> 0.4112901442931616
    //           (i, c) = 9: d, 8: e, 7: e, 6: s, 5: t, 4: _, 3: s, 2: e, 1: t
    fn pseudohash(&self, seed: &str) -> f64 {
        let mut num = 1.0f64;
        let mut len = self.len() + seed.len();

        let mut inner = |str: &str| {
            let part = str.as_bytes().iter().rev();
            for c in part {
                let c = *c as f64;
                num = ((1.1239285023 / num) * c * f64::consts::PI + f64::consts::PI * (len) as f64)
                    .fract();
                len -= 1;
            }
        };

        inner(seed);
        for part in self.0.iter().rev() {
            match part {
                KeyPart::Resample(i) => inner(ANTE_STRS[*i as usize + 3]),
                _ => {}
            }

            inner(match part {
                KeyPart::Empty => continue,
                KeyPart::Ante(i) => ANTE_STRS[*i as usize + 1],
                KeyPart::Source(source) => source.into_str(),
                KeyPart::Type(typ) => typ.into_str(),
                KeyPart::Resample(_) => "_resample",
            });
        }

        debug_assert_eq!(len, 0);
        num
    }
}

pub trait IntoKey {
    fn key(self) -> Key;
}

impl IntoKey for Key {
    fn key(self) -> Key {
        self
    }
}

impl IntoKey for [KeyPart; 1] {
    fn key(self) -> Key {
        Key([self[0], KeyPart::Empty, KeyPart::Empty, KeyPart::Empty])
    }
}

impl IntoKey for [KeyPart; 2] {
    fn key(self) -> Key {
        Key([self[0], self[1], KeyPart::Empty, KeyPart::Empty])
    }
}
impl IntoKey for [KeyPart; 3] {
    fn key(self) -> Key {
        Key([self[0], self[1], self[2], KeyPart::Empty])
    }
}
impl IntoKey for [KeyPart; 4] {
    fn key(self) -> Key {
        Key(self)
    }
}

impl<'a> Rng<'a> {
    pub fn new(seed: &'a str) -> Self {
        let hashed_seed = Key::default().pseudohash(seed);
        Rng {
            seed,
            hashed_seed,
            states: HashMap::new(),
        }
    }

    pub fn reseed(&mut self, seed: &'a str) {
        self.seed = seed;
        self.hashed_seed = Key::default().pseudohash(self.seed);
        self.states.clear();
    }

    pub fn roll<K: IntoKey>(&mut self, key: K) -> LuaRandom {
        let state = self
            .states
            .entry(key.key())
            .or_insert_with_key(|key| key.pseudohash(self.seed));

        let value = (*state * 1.72431234 + 2.134453429141).fract().abs();
        let value = round13(value);
        *state = value;
        let seed = (value + self.hashed_seed) / 2.0;
        LuaRandom::seed(seed)
    }
}

#[inline]
fn round13(f: f64) -> f64 {
    // format!("{:.13}", f).parse::<f64>().unwrap()
    (f * 1e13).round() / 1e13
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pseudohash() {
        // assert_eq!(pseudohash("test_seed"), 0.4112901442931616);
        assert_eq!(Key::default().pseudohash("test_seed"), 0.4112901442931616);
    }
}
