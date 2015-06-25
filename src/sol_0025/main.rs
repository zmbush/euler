#[macro_use] extern crate libeuler;
extern crate num;

use num::traits::Zero;
use num::traits::One;
use num::bigint::BigUint;
use std::mem::replace;

/// The Fibonacci sequence is defined by the recurrence relation:
///
///    Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
///
/// Hence the first 12 terms will be:
///
///    F1 = 1
///    F2 = 1
///    F3 = 2
///    F4 = 3
///    F5 = 5
///    F6 = 8
///    F7 = 13
///    F8 = 21
///    F9 = 34
///    F10 = 55
///    F11 = 89
///    F12 = 144
///
/// The 12th term, F12, is the first term to contain three digits.
///
/// What is the first term in the Fibonacci sequence to contain 1000 digits?
fn main() {
    solutions! {
        inputs: (digits: usize = 1000)

        sol naive {
            let min_bits = digits as f32 * 3.319f32;
            let mut current = BigUint::zero();
            let mut previous = BigUint::one();
            let mut i = 0;

            println!("Minimum bits needed to represent {} digits is {}", digits, min_bits);

            loop {
                let new = current + &previous;
                current = replace(&mut previous, new);

                i += 1;

                if current.bits() as f32 >= min_bits {
                    if format!("{}", current).len() >= digits {
                        break;
                    }
                }
            }

            i
        }
    }
}

