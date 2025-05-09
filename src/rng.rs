use std::{collections::HashMap, f64};

use crate::lua_random::LuaRandom;

#[derive(Debug, Clone)]
pub struct Rng {
    seed: String,
    hashed_seed: f64,
    states: HashMap<String, f64>,
}

impl Rng {
    pub fn new(seed: String) -> Self {
        let hashed_seed = pseudohash(&seed);
        Rng {
            seed,
            hashed_seed,
            states: HashMap::new(),
        }
    }

    pub fn reseed<S: Into<String>>(&mut self, seed: S) {
        let seed = seed.into();
        self.seed = seed;
        self.hashed_seed = pseudohash(&self.seed);
        self.states.clear();
    }

    pub fn roll<S: AsRef<str>>(&mut self, key: S) -> LuaRandom {
        let state = self
            .states
            .entry(key.as_ref().to_string())
            .or_insert_with_key(|key| pseudohash(format!("{}{}", key, self.seed)));

        let value = (*state * 1.72431234 + 2.134453429141).fract().abs();
        let value = round13(value);
        *state = value;
        let seed = (value + self.hashed_seed) / 2.0;
        LuaRandom::seed(seed)
    }
}

fn pseudohash<S: AsRef<str>>(s: S) -> f64 {
    let mut num = 1.0;
    for i in (0..s.as_ref().len()).rev() {
        let c = s.as_ref().as_bytes()[i] as f64;
        num =
            ((1.1239285023 / num) * c * f64::consts::PI + f64::consts::PI * (i + 1) as f64).fract();
    }
    if num.is_nan() {
        return f64::NAN;
    }
    num
}

fn round13(f: f64) -> f64 {
    format!("{:.13}", f).parse::<f64>().unwrap()
}
