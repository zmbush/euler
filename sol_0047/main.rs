#[macro_use] extern crate libeuler;

use libeuler::SieveOfAtkin;
/// The first two consecutive numbers to have two distinct prime factors are:
///
/// 14 = 2 × 7
/// 15 = 3 × 5
///
/// The first three consecutive numbers to have three distinct prime factors are:
///
/// 644 = 2² × 7 × 23
/// 645 = 3 × 5 × 43
/// 646 = 2 × 17 × 19.
///
/// Find the first four consecutive integers to have four distinct prime factors. What is the first
/// of these numbers?
fn main() {
    solutions! {
        inputs: (num: usize = 4)

        sol naive {
            let sieve = SieveOfAtkin::new(1_000_000);
            let mut first = None;
            let mut count = 0;
            for i in 0..1_000_000 {
                if sieve.factors(i).len() == num {
                    first = match first {
                        Some(v) => Some(v),
                        None => Some(i)
                    };
                    count += 1;
                } else {
                    first = None;
                    count = 0;
                }

                if count == num {
                    return first;
                }
            }

            None
        }
    }
}
