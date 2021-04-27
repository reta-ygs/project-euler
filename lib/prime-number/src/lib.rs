#[cfg(test)]
mod tests {
    use crate::is_prime_number;
    use std::ops::Not;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_is_prime_number() {
        assert!(is_prime_number(1).not());
        assert!(is_prime_number(2));
        assert!(is_prime_number(3));
        assert!(is_prime_number(4).not());
        assert!(is_prime_number(19));
    }
}

pub fn is_prime_number(target: usize) -> bool {
    if target <= 1 {
        return false;
    }
    (2..(target + 1)).into_iter()
        .filter(|num| num.pow(2) <= target)
        .all(|num| target % num != 0)
}