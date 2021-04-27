fn main() {
    println!("{}", fibonacci(4_000_000usize).iter()
        .filter(|num| *num % 2 == 0)
        .fold(0, |total, num| total + *num))
}

fn fibonacci(max: usize) -> Vec<usize> {
    let mut sequence = vec![1, 2];
    while sequence.last().unwrap() < &max {
        let next = sequence.iter()
            .skip(sequence.len() - 2)
            .sum();
        sequence.push(next);
    }
    sequence
}

#[test]
fn test() {
    let first = vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    assert_eq!(first, fibonacci(*first.last().unwrap()))
}