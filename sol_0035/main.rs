#[macro_use] extern crate libeuler;
extern crate num;

use libeuler::prime::SieveOfAtkin;
use std::collections::HashSet;
use num::traits::PrimInt;

/// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and
/// 719, are themselves prime.
///
/// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
///
/// How many circular primes are there below one million?
fn main() {
    solutions! {
        sol naive {
            let sieve = SieveOfAtkin::new(1_000_000);
            let mut vals = HashSet::new();

            for i in 1..1_000_000 {
                if vals.contains(&i) {
                    continue
                }

                let rots = rotations(i);
                let valid = rots.iter().all(|&v| sieve.is_prime(v as u64));
                if valid {
                    for v in rots.iter() {
                        vals.insert(v.clone());
                    }
                }
            }

            vals.len()
        }
    }
}

fn rotations(i: i64) -> Vec<i64> {
    let digits = (i as f64).log10().floor() as u32 + 1;
    (0..digits).map(|d| (10.pow(d), 10.pow(digits - d))).map(|(d, dig)| {
        let chop = i % d;
        let bottom = i / d;
        bottom + chop * dig
    }).collect()
}
