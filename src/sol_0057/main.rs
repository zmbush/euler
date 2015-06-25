#[macro_use] extern crate libeuler;
extern crate num;

use num::bigint::BigInt;
use num::traits::One;
use num::rational::BigRational;
use libeuler::traits::DigitsHelper;
/// It is possible to show that the square root of two can be expressed as an infinite continued
/// fraction.
///
///                  √ 2 = 1 + 1/(2 + 1/(2 + 1/(2 + ... ))) = 1.414213...
///
/// By expanding this for the first four iterations, we get:
///
/// 1 + 1/2 = 3/2 = 1.5
/// 1 + 1/(2 + 1/2) = 7/5 = 1.4
/// 1 + 1/(2 + 1/(2 + 1/2)) = 17/12 = 1.41666...
/// 1 + 1/(2 + 1/(2 + 1/(2 + 1/2))) = 41/29 = 1.41379...
///
/// The next three expansions are 99/70, 239/169, and 577/408, but the eighth expansion, 1393/985,
/// is the first example where the number of digits in the numerator exceeds the number of digits
/// in the denominator.
///
/// In the first one-thousand expansions, how many fractions contain a numerator with more digits
/// than denominator?
fn main() {
    solutions! {
        inputs: (expansions: i64 = 1_000)

        sol naive {
            let mut val = BigRational::new(BigInt::one(), BigInt::one() + BigInt::one());
            let mut count = 0;

            for _ in 1..expansions {
                let a = BigRational::from_integer(BigInt::one()) + &val;
                let (nd, _) = a.numer().digits();
                let (dd, _) = a.denom().digits();

                if nd.len() > dd.len() {
                    count += 1;
                }

                val = (
                    BigRational::from_integer(BigInt::one()) /
                    (BigRational::from_integer(BigInt::one() + BigInt::one()) + val)).reduced();
            }

            count
        }
    }
}
