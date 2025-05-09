use std::f64;

pub struct LuaRandom {
    state: [u64; 4],
}

impl LuaRandom {
    pub fn seed(mut seed: f64) -> Self {
        let mut lr = Self { state: [0; 4] };
        // const K_VALS: [u8; 4] = [0x01, 0x06, 0x09, 0x11];
        const K_VALS: [u8; 4] = [0x11, 0x09, 0x06, 0x01];

        for i in 0..4 {
            let m = 1u32 << K_VALS[i];

            seed = seed * f64::consts::PI + f64::consts::E;

            let mut u = seed.to_bits();
            if u < m as u64 {
                u += m as u64;
            }
            lr.state[i] = u;
        }

        for _ in 0..10 {
            lr.random_step();
        }

        lr
    }

    pub fn random(&mut self) -> f64 {
        let out = self.random_step();
        f64::from_bits(out) - 1.0
    }

    pub fn range(&mut self, min: u64, max: u64) -> u64 {
        (self.random() * (max - min + 1) as f64 + min as f64) as u64
    }

    fn random_step(&mut self) -> u64 {
        let mut r = 0u64;
        let mut tw223_gen = |i: usize, k: i32, q: i32, s: i32| {
            let mut z = self.state[i];
            z = (((z << q) ^ z) >> (k - s)) ^ ((z & (u64::MAX << (64 - k))) << s);
            r ^= z;
            self.state[i] = z;
        };

        tw223_gen(0, 63, 31, 18);
        tw223_gen(1, 58, 19, 28);
        tw223_gen(2, 55, 24, 7);
        tw223_gen(3, 47, 21, 8);
        let out = (r & 0x000FFFFFFFFFFFFF) | 0x3FF0000000000000;

        out
    }
}

#[test]
fn test_basic() {
    // for (seed, max, expected) in [(0.0, 2, 2), (0.0, 3, 2), (10.0, 10, 6)] {
    //     let mut lr = LuaRandom::seed(seed);
    //     assert_eq!(lr.range(1, max), expected, "seed: {seed}, max: {max}");
    // }
    for (seed, expected) in [(0.0, 0.794206292431241), (3.0, 0.5044953415063862)] {
        let mut lr = LuaRandom::seed(seed);
        assert_eq!(lr.random(), expected, "seed: {seed}");
    }
}
