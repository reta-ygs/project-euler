fn main() {
    for a in 1_i64..1000 {
        for b in (a + 1)..1000 {
            let c = 1000 - (a + b);
            debug_assert_eq!(a + b + c, 1000);
            if a < c && b < c && (a.pow(2) + b.pow(2)) == c.pow(2) {
                println!("{} * {} * {} = {}", a, b, c, a * b * c);
                return;
            }
        }
    }
}

