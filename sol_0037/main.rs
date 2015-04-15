#![feature(core)]
#[macro_use] extern crate libeuler;
extern crate num;

use libeuler::prime::SieveOfAtkin;
use num::traits::PrimInt;

/// The number 3797 has an interesting property. Being prime itself, it is possible to continuously
/// remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7.
/// Similarly we can work from right to left: 3797, 379, 37, and 3.
///
/// Find the sum of the only eleven primes that are both truncatable from left to right and right
/// to left.
///
/// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
fn main() {
    solutions! {
        sol naive {
            let sieve = SieveOfAtkin::new(1_000_000);
            sieve.iter().filter(|s| truncable(s, &sieve)).sum::<u64>()
        }
    }
}

fn truncable(i: &u64, sieve: &SieveOfAtkin) -> bool {
    if *i < 10 { return false }

    let mut a = i / 10;
    let mut digits = 1;

    while a > 0 {
        digits += 1;

        if !sieve.is_prime(a) {
            return false;
        }

        a /= 10;
    }

    let mut b = i.clone();
    for d in 0..(digits-1) {
        let pow = 10.pow(digits - d - 1);
        let dig = b / pow * pow;
        b -= dig;

        if !sieve.is_prime(b) {
            return false;
        }
    }

    true
}
