use prime_number::is_prime_number;

fn main() {
    println!("{}", (2_usize..).into_iter()
        .filter(|n| is_prime_number(*n))
        .skip(10000)
        .next().unwrap());
}
