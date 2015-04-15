#![feature(core)]
#[macro_use] extern crate libeuler;
use libeuler::prime::SieveOfAtkinIterator;

/// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
///
/// Find the sum of all the primes below two million.
fn main() {
    solutions!{
        inputs: (ceiling: u64 = 2_000_000)

        sol naive {
            SieveOfAtkinIterator::new(ceiling)
                .take_while(|&a| a < ceiling)
                .sum::<u64>()
        }
    };
}
