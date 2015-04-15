#[macro_use] extern crate libeuler;
extern crate num;

use libeuler::prime::SieveOfAtkin;
use num::integer::Integer;

/// It was proposed by Christian Goldbach that every odd composite number can be written as the sum
/// of a prime and twice a square.
///
/// 9 = 7 + 2×1^2
/// 15 = 7 + 2×2^2
/// 21 = 3 + 2×3^2
/// 25 = 7 + 2×3^2
/// 27 = 19 + 2×2^2
/// 33 = 31 + 2×1^2
///
/// It turns out that the conjecture was false.
///
/// What is the smallest odd composite that cannot be written as the sum of a prime and twice a
/// square?
fn main() {
    solutions! {
        sol naive {
            let sieve = SieveOfAtkin::new(10_000);
            let odd_composites = (2..10_000)
                .filter(|a| a.is_odd())
                .filter(|&a| !sieve.is_prime(a));

            for c in odd_composites {
                let twice_squares: Vec<u64> = (0..1000)
                    .map(|v| 2*v*v)
                    .take_while(|&v| v < c)
                    .collect();

                let no_primes = twice_squares.iter().all(|sq| {
                    !sieve.is_prime(c - sq)
                });

                if no_primes {
                    return Some(c);
                }
            }

            return None;
        }
    }
}
