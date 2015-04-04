#[macro_use] extern crate libeuler;
extern crate num;
use std::num::Int;
use std::iter::AdditiveIterator;
use libeuler::SieveOfAtkin;
use std::collections::HashMap;

use num::bigint::BigInt;
use num::bigint::ToBigInt;
use num::traits::One;

/// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
///
/// What is the sum of the digits of the number 2^1000?
fn main() {
    solutions!{
        inputs: (power: usize = 1_000)

        sol naive {
            let val = num::pow(2.to_bigint().unwrap(), power);

            let mut sum = 0;
            for c in format!("{}", val).chars() {
                sum += c.to_digit(10).unwrap();
            }

            sum
        }
    };
}
