#[macro_use] extern crate libeuler;

/// Surprisingly there are only three numbers that can be written as the sum of fourth powers of
/// their digits:
///
///     1634 = 1^4 + 6^4 + 3^4 + 4^4
///     8208 = 8^4 + 2^4 + 0^4 + 8^4
///     9474 = 9^4 + 4^4 + 7^4 + 4^4
///
/// As 1 = 1^4 is not a sum it is not included.
///
/// The sum of these numbers is 1634 + 8208 + 9474 = 19316.
///
/// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
fn main() {
    solutions! {
        sol naive {
            let mut sum = 0;
            // 10 million should be enough.
            for i in 2..10_000_000 {
                if sum_of_fifths(i) == i {
                    sum += i;
                }
            }

            sum
        }
    }
}

fn sum_of_fifths(mut n: i64) -> i64 {
    let mut sum = 0;
    while n > 0 {
        let digit = n % 10;
        n /= 10;

        sum += digit.pow(5);
    }

    sum
}
