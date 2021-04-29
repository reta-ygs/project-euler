use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::{is_prime_number, prime_factors};
    use std::ops::Not;

    #[test]
    fn test_is_prime_number() {
        assert!(is_prime_number(1).not());
        assert!(is_prime_number(2));
        assert!(is_prime_number(3));
        assert!(is_prime_number(4).not());
        assert!(is_prime_number(19));
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(2), vec![(2, 1)].iter().copied().collect());
        assert_eq!(prime_factors(4), vec![(2, 2)].iter().copied().collect());
        assert_eq!(prime_factors(8), vec![(2, 3)].iter().copied().collect());
        assert_eq!(
            prime_factors(10),
            vec![(2, 1), (5, 1)].iter().copied().collect()
        );
        assert_eq!(
            prime_factors(20),
            vec![(2, 2), (5, 1)].iter().copied().collect()
        );
    }
}

pub fn is_prime_number(target: usize) -> bool {
    if target <= 1 {
        return false;
    }
    (2..=target)
        .into_iter()
        .filter(|num| num.pow(2) <= target)
        .all(|num| target % num != 0)
}

pub fn prime_factors(number: usize) -> HashMap<usize, u32> {
    let mut factors = HashMap::new();
    let mut remaining = number;
    for n in (2..=number)
        .into_iter()
        .filter(|n| is_prime_number(*n))
        .filter(|n| number % n == 0)
    {
        while remaining % n == 0 {
            match factors.get_mut(&n) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    factors.insert(n, 1);
                }
            }
            remaining /= n;
        }
        if remaining == 1 {
            break;
        }
    }
    factors
}
