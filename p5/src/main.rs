use prime_number::prime_factors;
use std::cmp::max;
use std::collections::HashMap;

fn main() {
    println!("{}", resolve(20));
}

fn resolve(target: usize) -> usize {
    let mut acc = HashMap::new();
    (2..=target)
        .into_iter()
        .map(prime_factors)
        .flat_map(HashMap::into_iter)
        .for_each(|(num, count)| {
            acc.insert(
                num,
                acc.get(&num).map_or(count, |current| max(count, *current)),
            );
        });
    acc.iter()
        .map(|(num, count)| num.pow(*count))
        .fold(1, |total, each| total * each)
}

#[cfg(test)]
mod test {
    use crate::resolve;

    #[test]
    fn test_resolve() {
        assert_eq!(resolve(10), 2520);
    }
}
