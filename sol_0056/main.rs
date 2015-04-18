#[macro_use] extern crate libeuler;
extern crate num;

use num::bigint::{BigInt, ToBigInt};
use num::traits::{Zero, ToPrimitive};
/// A googol (10^100) is a massive number: one followed by one-hundred zeros; 100^100 is almost
/// unimaginably large: one followed by two-hundred zeros. Despite their size, the sum of the
/// digits in each number is only 1.
///
/// Considering natural numbers of the form, ab, where a, b < 100, what is the maximum digital sum?
fn main() {
    solutions! {
        sol naive {
            let ten = 10.to_bigint().unwrap();
            let mut maxsum = 0;
            for a in 90..100 {
                let ab = a.to_bigint().unwrap();
                let mut ap = num::pow(ab.clone(), 89);
                for _ in 90..100 {
                    ap = &ap * &ab;

                    let mut d = ap.clone();
                    let mut sum = 0;

                    while d > BigInt::zero() {
                        sum += (&d % &ten).to_i64().unwrap();
                        d = &d / &ten;
                    }

                    if sum > maxsum {
                        maxsum = sum;
                        println!("{}", maxsum);
                    }
                }
            }

            maxsum
        }
    }
}
