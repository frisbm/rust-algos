use std::time::{SystemTime, UNIX_EPOCH};

struct Random {
    pub seed: [u128; 2],
}

impl Random {
    pub fn new() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let base = now
            .as_micros()
            .wrapping_shl(now.as_secs().wrapping_rem(11) as u32);
        let mut s = Self { seed: [0, 0] };
        s.set_seed(base);
        s
    }

    pub fn set_seed(&mut self, seed: u128) {
        let seed = seed ^ seed.wrapping_shl(11);
        let length = seed.checked_ilog10().unwrap_or(0) + 1;

        let mut seed_arr = [0; 2];
        seed_arr[0] = seed.wrapping_shr(length / 2) | 1;
        seed_arr[1] = seed.wrapping_shr(length) | 1;
        self.seed = seed_arr;
    }

    pub fn xorshiftr128plus(&mut self) -> u128 {
        let mut x = self.seed[0];
        let y = self.seed[1];
        self.seed[0] = y;
        x ^= x.wrapping_shl(23);
        x ^= x.wrapping_shr(17);
        x ^= y;
        self.seed[1] = x.wrapping_add(y);
        x
    }

    pub fn random(&mut self, min: u128, max: u128) -> u128 {
        let result = self.xorshiftr128plus();
        result.wrapping_rem(max.wrapping_sub(min)).wrapping_add(min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_many_calls() {
        let mut rng = Random::new();
        let min = 0;
        let max = 100;
        let mut results: Vec<u128> = Vec::new();
        for _ in 0..1_000_000 {
            results.push(rng.random(min, max));
        }

        results.iter().for_each(|x| assert!(*x >= min && *x < max));
        assert_eq!(results.len(), 1_000_000);
        assert_ne!(results.iter().sum::<u128>(), results[0] * 1_000_000);
    }

    #[test]
    fn test_random() {
        let mut rng = Random::new();
        rng.set_seed(4559398);
        let min = 10;
        let max = 20;
        let result = rng.random(min, max);
        assert!(result >= min && result < max);
        assert_eq!(result, 19);
    }
}
