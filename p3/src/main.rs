use std::collections::HashSet;
use prime_number::is_prime_number;

extern crate prime_number;

fn main() {
    println!("{}", prime_factors(600851475143).iter().max().unwrap());
}

#[cfg(test)]
mod test {
    use crate::prime_factors;
    use std::collections::HashSet;

    #[test]
    fn test() {
        let mut expected: HashSet<usize> = HashSet::new();
        expected.insert(5);
        expected.insert(7);
        expected.insert(13);
        expected.insert(29);
        assert_eq!(prime_factors(13195), expected)
    }
}

fn prime_factors(target: usize) -> HashSet<usize> {
    let mut remaining = target;
    let mut prime_factors = HashSet::new();
    for num in (2..target).into_iter().filter(|num| num.pow(2) <= target) {
        if is_prime_number(num) {
            while remaining != 1 && remaining % num == 0 {
                prime_factors.insert(num);
                remaining = remaining / num;
            }
        }
        if remaining == 1 {
            return prime_factors;
        }
    }
    prime_factors
}