#![feature(core)]
#[macro_use] extern crate libeuler;

use libeuler::prime::SieveOfAtkin;
/// The prime 41, can be written as the sum of six consecutive primes:
///                 41 = 2 + 3 + 5 + 7 + 11 + 13
///
/// This is the longest sum of consecutive primes that adds to a prime below one-hundred.
///
/// The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21
/// terms, and is equal to 953.
///
/// Which prime, below one-million, can be written as the sum of the most consecutive primes?
fn main() {
    solutions! {
        inputs: (max: u64 = 1_000_000)

        sol naive {
            let mut window = Vec::new();
            let mut maxlen = 0;
            let mut thesum = 0;
            let sieve = SieveOfAtkin::new(max);

            for prime in sieve.iter() {
                window.push(prime);
                while window.iter().sum::<u64>() > max {
                    window.remove(0);
                }

                if window.len() > maxlen {
                    for start in 0..(window.len() - maxlen) {
                        let mut w = window.iter();
                        if start > 0 {
                            w.nth(start - 1);
                        }

                        let sum = w.sum();
                        if sieve.is_prime(sum) {
                            thesum = sum;
                            maxlen = window.len() - start;
                        }
                    }
                }
            }

            thesum
        }
    }
}
