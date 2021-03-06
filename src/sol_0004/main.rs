#[macro_use] extern crate libeuler;
extern crate num;

use libeuler::traits::PalindromeHelper;

/// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
///
/// Find the largest palindrome made from the product of two 3-digit numbers.
fn main() {
    solutions!{
        inputs: (digits: i64 = 3)

        sol naive {
            // 1 digit = 10^0, 2 digit = 20^1, 3 digit = 10^2
            let min = num::pow(10, (digits - 1) as usize);
            let max = num::pow(10, digits as usize);
            let mut max_palindrome: i64 = 0;

            for first in min..max {
                for second in first..max {
                    let prod = first*second;
                    if prod > max_palindrome && prod.is_palindrome() {
                        max_palindrome = prod;
                    }
                }
            }

            max_palindrome
        }
    };
}
