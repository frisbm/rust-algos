use crate::utils::math::{is_prime, next_prime};
use std::time::{SystemTime, UNIX_EPOCH};

struct Random {
    pub seed: u128,
    pub p_1: u128,
    pub p_2: u128,
}

impl Random {
    pub fn new() -> Self {
        Self {
            seed: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u128,
            p_1: 237277,
            p_2: 53129081,
        }
    }

    pub fn set_seed(&mut self, seed: u128) {
        self.seed = seed;
    }

    fn blum_blum_shub_variant(&self, p_1: u128, p_2: u128, iterations: u128) -> u128 {
        let mut p1 = p_1;
        let mut p2 = p_2;
        while p1.wrapping_rem(4) != 3 && !is_prime(p1.wrapping_div(2)) {
            p1 = next_prime(p1);
        }
        while p2.wrapping_rem(4) != 3 && !is_prime(p2.wrapping_div(2)) {
            p2 = next_prime(p2);
        }

        let n = p1.wrapping_mul(p2);
        let mut numbers: Vec<u128> = Vec::new();
        let mut seed = self.seed;
        for _ in 0..iterations {
            seed = seed.wrapping_pow(2).wrapping_rem(n);
            if numbers.contains(&seed) {
                return seed;
            }
            numbers.push(seed);
        }
        seed
    }

    pub fn random(&mut self, min: u128, max: u128) -> u128 {
        let result = self
            .blum_blum_shub_variant(self.p_1, self.p_2, 100)
            .wrapping_rem((max.wrapping_sub(min)).wrapping_add(min));

        self.p_1 = next_prime(self.p_2 + (self.p_2 % self.seed)) + 1;
        self.p_2 = next_prime(self.p_1 + (self.p_1 % self.seed)) + 1;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_blum_blum_shub_known_seed() {
        let mut rng = Random::new();
        rng.set_seed(82229);
        // non primes handled
        let p1 = 60003582;
        let p2 = 48882920;
        let iterations = 20;
        let result = rng.blum_blum_shub_variant(p1, p2, iterations);
        assert_eq!(result, 1588456154907759);
    }

    #[test]
    fn test_blum_blum_shub_same_start_seeds() {
        let rng_1 = Random::new();
        let mut rng_2 = Random::new();
        rng_2.set_seed(rng_1.seed);
        let p1 = 11;
        let p2 = 19;
        let iterations = 10;
        let result_1 = rng_1.blum_blum_shub_variant(p1, p2, iterations);
        let result_2 = rng_2.blum_blum_shub_variant(p1, p2, iterations);
        assert_eq!(result_1, result_2);
    }

    #[test]
    fn test_blum_blum_shub_different_start_seeds() {
        let rng_1 = Random::new();
        sleep(std::time::Duration::from_secs(1));
        let rng_2 = Random::new();

        let p1 = 11;
        let p2 = 19;
        let iterations = 10;
        let result_1 = rng_1.blum_blum_shub_variant(p1, p2, iterations);
        let result_2 = rng_2.blum_blum_shub_variant(p1, p2, iterations);
        assert_ne!(result_1, result_2);
    }

    #[test]
    fn test_blum_blum_shub_can_change_seeds() {
        let rng_1 = Random::new();
        let mut rng_2 = Random::new();
        rng_2.set_seed(rng_1.seed);
        rng_2.set_seed(rng_1.seed + 1);
        let p1 = 11;
        let p2 = 19;
        let iterations = 10;
        let result_1 = rng_1.blum_blum_shub_variant(p1, p2, iterations);
        let result_2 = rng_2.blum_blum_shub_variant(p1, p2, iterations);
        assert_ne!(result_1, result_2);
    }

    #[test]
    fn test_random() {
        let mut rng = Random::new();
        rng.set_seed(29993827);
        let min = 10;
        let max = 20;
        let result = rng.random(min, max);
        assert!(result >= min && result < max);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_random_same_seed() {
        let mut rng_1 = Random::new();
        let mut rng_2 = Random::new();
        rng_2.set_seed(rng_1.seed);
        let min = 10;
        let max = 20;
        let result_1 = rng_1.random(min, max);
        let result_2 = rng_2.random(min, max);
        assert_eq!(result_1, result_2);
    }

    #[test]
    fn test_random_many_calls() {
        let mut rng = Random::new();
        rng.set_seed(29993827);
        let min = 0;
        let max = 100;
        let mut results: Vec<u128> = Vec::new();
        for _ in 0..1_000 {
            results.push(rng.random(min, max));
        }

        results.iter().for_each(|x| assert!(*x >= min && *x < max));
        results.iter().for_each(|x| print!("{} ", x));
        assert_eq!(results.len(), 1_000);
        assert_ne!(results.iter().sum::<u128>(), results[0] * 1_000);
    }

    #[test]
    fn test_random_different_seed() {
        let mut rng_1 = Random::new();
        let mut rng_2 = Random::new();
        rng_2.set_seed(10001);
        let min = 4;
        let max = 140052;
        let result_1 = rng_1.random(min, max);
        let result_2 = rng_2.random(min, max);
        assert_ne!(result_1, result_2);
    }

    #[test]
    fn test_random_can_change_seed() {
        let mut rng_1 = Random::new();
        let mut rng_2 = Random::new();
        rng_2.set_seed(rng_1.seed);
        rng_2.set_seed(rng_1.seed + 1);
        let min = 10;
        let max = 250;
        let result_1 = rng_1.random(min, max);
        let result_2 = rng_2.random(min, max);
        assert_ne!(result_1, result_2);
    }
}
