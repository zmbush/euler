#[macro_use] extern crate libeuler;
use libeuler::prime::{PrimeIterator, SieveOfAtkinIterator};

/// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
///
/// What is the 10,001st prime number?
fn main() {
    solutions!{
        inputs: (nth_prime: usize = 10_001)

        sol naive {
            let mut primes_found = 0;
            let mut last_prime = 0;
            let mut value = 2;

            fn is_prime(v: i64) -> bool {
                for i in 2..v {
                    if v % i == 0 {
                        return false;
                    }
                }

                true
            }

            while primes_found < nth_prime {
                if is_prime(value) {
                    last_prime = value;
                    primes_found += 1;
                }
                value += 1;
            }

            last_prime
        }

        sol better_is_prime {
            let mut previous_primes = Vec::new();
            let mut value = 2;

            fn is_prime(v: i64, prev: &Vec<i64>) -> bool {
                for pp in prev.iter() {
                    if &v % pp == 0 {
                        return false;
                    }
                }

                true
            }

            while previous_primes.len() < nth_prime {
                if is_prime(value, &previous_primes) {
                    previous_primes.push(value)
                }
                value += 1;
            }

            let v = previous_primes.last().unwrap().clone();

            v
        }

        sol iterator {
            PrimeIterator::new().nth(nth_prime - 1).unwrap_or(0 - 1)
        }

        sol sieve_of_atkin {
            SieveOfAtkinIterator::new(500_000).nth(nth_prime - 1).unwrap_or(0)
        }
    };
}
