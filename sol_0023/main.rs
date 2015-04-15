#![feature(core)]
#[macro_use] extern crate libeuler;

use libeuler::prime::SieveOfAtkin;
use std::collections::HashSet;

/// A perfect number is a number for which the sum of its proper divisors is exactly equal to the
/// number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28,
/// which means that 28 is a perfect number.
///
/// A number n is called deficient if the sum of its proper divisors is less than n and it is called
/// abundant if this sum exceeds n.
///
/// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be
/// written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that
/// all integers greater than 28123 can be written as the sum of two abundant numbers. However, this
/// upper limit cannot be reduced any further by analysis even though it is known that the greatest
/// number that cannot be expressed as the sum of two abundant numbers is less than this limit.
///
/// Find the sum of all the positive integers which cannot be written as the sum of two abundant
/// numbers.
fn main() {
    solutions! {
        sol naive {
            let max_value = 28123;
            let sieve = SieveOfAtkin::new(1_000_000);
            let abundant_nums: Vec<u64> = (1..max_value)
                .filter(|&i| sieve.sum_of_proper_divisors(i) > i)
                .collect();
            let mut candidates: HashSet<u64> = (1..max_value).collect();

            for i in 0..abundant_nums.len() {
                for j in i..abundant_nums.len() {
                    let v = abundant_nums[i] + abundant_nums[j];
                    if v > max_value { break }

                    candidates.remove(&v);
                }
            }

            candidates.iter().map(|&a| a).sum::<u64>()
        }
    }
}

