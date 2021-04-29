fn main() {
    println!("{}", resolve(100));
}

fn resolve(max: usize) -> usize {
    sum_then_square(max) - square_then_sum(max)
}

fn square_then_sum(max: usize) -> usize {
    (1..=max).into_iter()
        .map(|n| n.pow(2))
        .sum()
}

fn sum_then_square(max: usize) -> usize {
    (1..=max)
        .sum::<usize>().pow(2)
}

#[cfg(test)]
mod test {
    use crate::{sum_then_square, square_then_sum, resolve};

    #[test]
    fn test_resolve() {
        assert_eq!(resolve(10), 2640);
    }

    #[test]
    fn test_square_then_sum() {
        assert_eq!(square_then_sum(10), 385);
    }

    #[test]
    fn test_sum_then_square() {
        assert_eq!(sum_then_square(10), 3025);
    }
}