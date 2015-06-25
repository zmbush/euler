#[macro_use] extern crate libeuler;

extern crate num;
use num::bigint::{BigInt, ToBigInt};
use num::traits::{One, Zero};
/// n! means n × (n − 1) × ... × 3 × 2 × 1
///
/// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
/// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
///
/// Find the sum of the digits in the number 100!
fn main() {
    solutions! {
        sol naive {
            let mut sum = 0;
            for c in format!("{}", kinda_fact(100)).chars() {
                let v: i64 = c as i64 - '0' as i64;
                sum += v;
            }

            sum
        }
    }
}

fn kinda_fact(n: i64) -> BigInt {
    let ten = 10.to_bigint().unwrap();
    let nb = n.to_bigint().unwrap();

    match n {
        1 => BigInt::one(),
        n => match nb * kinda_fact(n - 1) {
            ref ret if ret % &ten == BigInt::zero() => ret / &ten,
            ret => ret
        }
    }
}
