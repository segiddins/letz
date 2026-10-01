use std::{collections::HashMap, f64, num::NonZero};

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
    Resample(NonZero<u8>),
}

const ANTE_STRS: [&str; 52] = [
    "-1", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
    "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
    "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47",
    "48", "49", "50",
];

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
            KeyPart::Resample(i) => "_resample".len() + ANTE_STRS[i.get() as usize + 2].len(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Key([KeyPart; 4]);

static_assertions::assert_eq_size!(KeyPart, u16);
static_assertions::assert_eq_size!(Key, u64);

impl From<game::Source> for KeyPart {
    fn from(val: game::Source) -> Self {
        KeyPart::Source(val)
    }
}

impl From<game::Type> for KeyPart {
    fn from(val: game::Type) -> Self {
        KeyPart::Type(val)
    }
}

impl Key {
    pub fn resample(&mut self, i: NonZero<u8>) {
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
            if let KeyPart::Resample(i) = part {
                inner(ANTE_STRS[i.get() as usize + 2])
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
    // RNG states are fractions in [0, 1). Round the *exact* binary value
    // times 10^13 using integers, then parse the resulting decimal fraction.
    // This matches Lua's tonumber(string.format("%.13f", f)) without allocating
    // or first rounding the floating-point product f * 1e13.
    debug_assert!((0.0..1.0).contains(&f));
    let bits = f.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as u32;
    if exponent == 0 {
        return 0.0; // even the largest subnormal rounds to zero
    }
    let significand = (bits & ((1_u64 << 52) - 1)) | (1_u64 << 52);
    let numerator = significand as u128 * 10_000_000_000_000_u128;
    let shift = 1075 - exponent;
    if shift >= 128 {
        return 0.0;
    }
    let mut digits = numerator >> shift;
    let remainder = numerator & ((1_u128 << shift) - 1);
    let halfway = 1_u128 << (shift - 1);
    if remainder > halfway || (remainder == halfway && digits & 1 != 0) {
        digits += 1;
    }
    digits as f64 / 10_000_000_000_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pseudohash() {
        // assert_eq!(pseudohash("test_seed"), 0.4112901442931616);
        assert_eq!(Key::default().pseudohash("test_seed"), 0.4112901442931616);
    }

    #[test]
    fn decimal_rounding_regression() {
        assert_eq!(round13(0.26587044836955), 0.2658704483695);
    }

    #[test]
    fn tag1_state_regression() {
        let key = [KeyPart::Type(game::Type::Tags), KeyPart::Ante(1)];
        let mut rng = Rng::new("1111126L");
        rng.roll(key);
        assert_eq!(rng.states[&key.key()], 0.8830814427953);
    }

    #[test]
    fn tag_resampling_regression() {
        let mut game = game::Game::new("111111SJ");
        assert_eq!(game.next_tag(), game::Tag::Boss_Tag);
        assert_eq!(game.next_tag(), game::Tag::Charm_Tag);
    }

    #[cfg(unix)]
    #[test]
    fn decimal_rounding_matches_c_printf() {
        use std::ffi::{c_char, c_int};
        unsafe extern "C" {
            fn snprintf(buf: *mut c_char, len: usize, fmt: *const c_char, ...) -> c_int;
        }
        let mut x = 0x8422_7f93_af10_97d1_u64;
        let random_values = (0..100_000).map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            (x >> 11) as f64 / (1_u64 << 53) as f64
        });
        let edge_values = [
            0.0,
            f64::from_bits(1),
            f64::MIN_POSITIVE,
            f64::from_bits(1.0_f64.to_bits() - 1),
        ];
        let near_ties = (0..1000).map(|i| (i as f64 + 0.5) / 1e13);
        for value in random_values.chain(edge_values).chain(near_ties) {
            let mut buf = [0_u8; 64];
            let len =
                unsafe { snprintf(buf.as_mut_ptr().cast(), buf.len(), c"%.13f".as_ptr(), value) }
                    as usize;
            let printed = std::str::from_utf8(&buf[..len]).unwrap();
            let expected: f64 = printed.parse().unwrap();
            assert_eq!(round13(value).to_bits(), expected.to_bits(), "{value:?}");
        }
    }
}
