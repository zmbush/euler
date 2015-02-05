fn main() {
    let mut sum = 0i64;

    for i in 0..1000 {
        sum += match i {
            i if (i % 3 == 0 || i % 5 == 0) => i,
            _ => 0
        }
    }

    println!("Solution: {}", sum);
}
