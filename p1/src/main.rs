
fn main() {
    println!("{}", resolve(1000))
}

fn resolve(target: usize) -> usize {
    (1..target).into_iter()
        .filter(|num|  (num % 3) == 0 || (num % 5) == 0)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::resolve;

    #[test]
    fn test_resolve() {
        assert_eq!(resolve(10), 23)
    }
}