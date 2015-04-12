#[macro_use] extern crate libeuler;

use libeuler::DigitsHelper;

/// An irrational decimal fraction is created by concatenating the positive integers:
///
/// 0.123456789101112131415161718192021...
///              ^
///
/// It can be seen that the 12th digit of the fractional part is 1.
///
/// If dn represents the nth digit of the fractional part, find the value of the following
/// expression.
///
/// d1 × d10 × d100 × d1_000 × d10_000 × d100_000 × d1_000_000
fn main() {
    solutions! {
        sol naive {
            let mut num = 1i64;
            let mut i = 0;
            let mut need = 1;
            let mut prod = 1;

            while i <= 1_000_000 {
                let (digits, _) = num.digits();
                i += digits.len();

                if i >= need {
                    let dex = digits.len() - (i - need) - 1;
                    prod *= digits[dex];
                    need *= 10;
                }

                num += 1;
            }

            prod
        }
    }
}
