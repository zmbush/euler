#[macro_use] extern crate libeuler;

use libeuler::prime::SieveOfAtkin;
use libeuler::traits::DigitsHelper;

/// By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine possible
/// values: 13, 23, 43, 53, 73, and 83, are all prime.
///
/// By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number is the
/// first example having seven primes among the ten generated numbers, yielding the family: 56003,
/// 56113, 56333, 56443, 56663, 56773, and 56993. Consequently 56003, being the first member of
/// this family, is the smallest prime with this property.
///
/// Find the smallest prime which, by replacing part of the number (not necessarily adjacent
/// digits) with the same digit, is part of an eight prime value family.
fn main() {
    solutions! {
        inputs: (family_size: usize = 8)

        sol naive {
            let sieve = SieveOfAtkin::new(1_000_000);

            for prime in sieve.iter() {
                let (_, digits) = prime.digits();
                for d in digits {
                    let primes = (0..10)
                        .map(|i| prime.replace_all(d, i))
                        .filter(|&i| i >= prime && sieve.is_prime(i))
                        .count();

                    if primes >= family_size {
                        return prime;
                    }
                }
            }

            unreachable!();
        }
    }
}
