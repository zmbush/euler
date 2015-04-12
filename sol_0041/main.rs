#[macro_use] extern crate libeuler;

use libeuler::{SieveOfAtkin, DigitsHelper};

/// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n
/// exactly once. For example, 2143 is a 4-digit pandigital and is also prime.
///
/// What is the largest n-digit pandigital prime that exists?
fn main() {
    solutions! {
        sol naive {
            SieveOfAtkin::new(10_000_000)
                .iter()
                .filter(|p| p.is_pandigital())
                .max()
                .unwrap_or(2143) // Because we were given this one :P
        }
    }
}
