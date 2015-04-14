#[macro_use] extern crate libeuler;
extern crate num;

use num::bigint::{BigInt, ToBigInt};
use num::traits::{Zero, One, ToPrimitive, PrimInt};

/// The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
///
/// Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
fn main() {
    solutions! {
        sol naive {
            let tenten = 10.pow(10);
            let sum = (1..1001)
                .map(|i| powmod(i, i, tenten))
                .fold(0, |c, v| {
                    c + v
                });

            sum % tenten
        }
    }
}

// Based on algorithm found here:
// http://stackoverflow.com/questions/8287144/modulus-power-of-big-numbers
fn powmod(b: i64, e: i64, m: i64) -> i64 {
    if b < 1 || e < 0 || m < 1 {
        return -1;
    }

    let mut base = b.to_bigint().unwrap();
    let mut exponent = e.to_bigint().unwrap();
    let modulus = m.to_bigint().unwrap();

    let two = BigInt::one() + BigInt::one();

    let mut result = BigInt::one();
    while exponent > BigInt::zero() {
        if (&exponent % &two) == BigInt::one() {
            result = (result * &base) % &modulus;
        }
        base = (&base * &base) % &modulus;
        exponent = &exponent / &two;
    }

    result.to_i64().unwrap()
}
