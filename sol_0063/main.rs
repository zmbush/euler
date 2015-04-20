#[macro_use] extern crate libeuler;
extern crate num;

use num::traits::PrimInt;
use num::bigint::{BigInt, ToBigInt};
/// The 5-digit number, 16807=7^5, is also a fifth power. Similarly, the 9-digit number,
/// 134217728=8^9, is a ninth power.
///
/// How many n-digit positive integers exist which are also an nth power?
fn main() {
    solutions! {
        sol naive {
            let mut count = 0;

            for pow in 1..30 {
                let (min, max) = range(pow);

                for base in 0..10 {
                    let n = num::pow(base.to_bigint().unwrap(), pow);

                    if n >= min && n < max {
                        count += 1;
                    }
                }
            }

            count
        }
    }
}

fn range(digits: usize) -> (BigInt, BigInt) {
    let ten = 10.to_bigint().unwrap();
    (num::pow(ten.clone(), digits - 1), num::pow(ten, digits))
}
