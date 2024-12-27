pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

pub fn is_prime(num: u128) -> bool {
    if num <= 1 {
        return false;
    }
    if num <= 3 {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn next_prime(num: u128) -> u128 {
    let mut i = num.wrapping_add(1);
    while !is_prime(i) {
        i = i.wrapping_add(1);
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(10, 5), 5);
        assert_eq!(gcd(14, 21), 7);
        assert_eq!(gcd(3, 5), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(10, 5), 10);
        assert_eq!(lcm(14, 21), 42);
        assert_eq!(lcm(3, 5), 15);
    }

    #[test]
    fn test_is_prime(){
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7320), false);
        assert_eq!(is_prime(7321), true);
        assert_eq!(is_prime(23937082), false);
        assert_eq!(is_prime(23937083), true);
        assert_eq!(is_prime(45348948862), false);
        assert_eq!(is_prime(45348948863), true);
        assert_eq!(is_prime(100000042590), false);
        assert_eq!(is_prime(100000042591), true);
        assert_eq!(is_prime(546846844672), false);
        assert_eq!(is_prime(546846844673), true);
    }
    
    #[test]
    fn test_next_prime(){
        assert_eq!(next_prime(0), 2);
        assert_eq!(next_prime(1), 2);
        assert_eq!(next_prime(2), 3);
        assert_eq!(next_prime(3), 5);
        assert_eq!(next_prime(4), 5);
        assert_eq!(next_prime(5), 7);
        assert_eq!(next_prime(7320), 7321);
        assert_eq!(next_prime(7321), 7331);
        assert_eq!(next_prime(23937082), 23937083);
        assert_eq!(next_prime(23937083), 23937107);
        assert_eq!(next_prime(45348948862), 45348948863);
        assert_eq!(next_prime(45348948863), 45348948881);
        assert_eq!(next_prime(100000042590), 100000042591);
        assert_eq!(next_prime(100000042591), 100000042619);
        assert_eq!(next_prime(546846844672), 546846844673);
        assert_eq!(next_prime(546846844673), 546846844681);
    }
}
