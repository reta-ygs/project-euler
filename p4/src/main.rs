fn main() {
    println!("{}", resolve(3));
}

fn resolve(digit: u32) -> usize {
    let min = 10_usize.pow(digit - 1);
    let max = (min * 10) - 1;
    (min.pow(2)..max.pow(2)).into_iter().rev()
        .filter(|num| is_palindromic(*num))
        .filter(|num| (min..max).into_iter()
            .filter(|i| num % i == 0)
            .map(|i| num / i)
            .any(|i| min <= i && i <= max))
        .next().unwrap()
}

fn is_palindromic(number: usize) -> bool {
    let reversed = String::from_utf8(number.to_string().as_bytes().iter().rev().copied().collect()).unwrap();
    number.to_string() == reversed
}

#[cfg(test)]
mod test {
    use crate::{is_palindromic, resolve};

    #[test]
    fn test_is_palindromic() {
        assert!(is_palindromic(1));
        assert!(is_palindromic(11));
        assert!(is_palindromic(12321));
        assert!(!is_palindromic(155));
        assert!(!is_palindromic(15510));
    }

    #[test]
    fn test_resolve() {
        assert_eq!(resolve(2), 9009);
    }
}