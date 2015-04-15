#![feature(core)]
#[macro_use] extern crate libeuler;

use libeuler::traits::PalindromeHelper;
/// The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
///
/// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base
/// 2.
///
/// (Please note that the palindromic number, in either base, may not include leading zeros.)
fn main() {
    solutions! {
        sol naive {
            (0..1_000_000)
                .filter(|i| i.is_palindrome())
                .filter(|i| format!("{:b}", i).is_palindrome())
                .sum::<i64>()
        }
    }
}
