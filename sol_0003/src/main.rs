#[macro_use] extern crate libeuler;
use libeuler::SieveOfAtkin;
use std::num::Float;

/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143?
fn main() {
    solutions!{
        inputs: (number: u64 = 600851475143)

        sol naive {
            let mut factored = number.clone();
            let mut i = 2;

            while i < factored {
                if factored % i == 0 {
                    factored /= i;
                } else {
                    i += 1;
                }
            }

            factored
        }

        sol sieve {
            let mut factors = SieveOfAtkin::new((number as f64).sqrt() as u64)
                .factorize(number);

            println!("factors: {:?}", factors);
            factors.pop()
        }
    };
}
