#[macro_use] extern crate libeuler;

use libeuler::prime::SieveOfAtkin;
use libeuler::traits::DigitsHelper;
use std::f64;
/// Euler's Totient function, φ(n) [sometimes called the phi function], is used to determine the
/// number of positive numbers less than or equal to n which are relatively prime to n. For
/// example, as 1, 2, 4, 5, 7, and 8, are all less than nine and relatively prime to nine, φ(9)=6.
/// The number 1 is considered to be relatively prime to every positive number, so φ(1)=1.
///
/// Interestingly, φ(87109)=79180, and it can be seen that 87109 is a permutation of 79180.
///
/// Find the value of n, 1 < n < 107, for which φ(n) is a permutation of n and the ratio n/φ(n)
/// produces a minimum.
fn main() {
    solutions! {
        sol naive {
            let sieve = SieveOfAtkin::new(10_000);
            let limit = 10_000_000;
            let mut min = f64::MAX;
            let mut minnum = 0;

            for p1 in sieve.iter().filter(|&p| p >= 2000 && p <= 5000) {
                for p2 in sieve.iter().filter(|&p| p > p1 && p <= 5000) {
                    let num = p1*p2;

                    if num > limit {
                        continue;
                    }

                    let phi = (p1 - 1)*(p2 - 1);

                    if num.is_permutation_of(&phi) {
                        let rat = num as f64 / phi as f64;
                        if rat < min {
                            min = rat;
                            minnum = num;
                            println!("φ({}) = {} ({})", num, phi, min);
                        }
                    }
                }
            }

            minnum
        }
    }
}
