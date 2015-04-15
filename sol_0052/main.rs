#[macro_use] extern crate libeuler;

use libeuler::traits::DigitsHelper;

/// It can be seen that the number, 125874, and its double, 251748, contain exactly the same
/// digits, but in a different order.
///
/// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same
/// digits.
fn main() {
    solutions! {
        sol naive {
            for i in 1.. {
                if (2..7).all(|d| i.is_permutation_of(&(i*d))) {
                    return i
                }
            }

            unreachable!();
        }
    }
}
