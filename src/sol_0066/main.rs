#[macro_use] extern crate libeuler;
extern crate num;

use libeuler::irrational::continued_fraction;
use num::bigint::{ToBigInt, BigInt};
use num::traits::{Zero, One};
/// Consider quadratic Diophantine equations of the form:
///
/// x^2 – Dy^2 = 1
///
/// For example, when D=13, the minimal solution in x is 649^2 – 13×180^2 = 1.
///
/// It can be assumed that there are no solutions in positive integers when D is square.
///
/// By finding minimal solutions in x for D = {2, 3, 5, 6, 7}, we obtain the following:
///
/// 3^2 – 2×2^2 = 1
/// 2^2 – 3×1^2 = 1
/// 9^2 – 5×4^2 = 1
/// 5^2 – 6×2^2 = 1
/// 8^2 – 7×3^2 = 1
///
/// Hence, by considering minimal solutions in x for D ≤ 7, the largest x is obtained when D=5.
///
/// Find the value of D ≤ 1000 in minimal solutions of x for which the largest value of x is
/// obtained.
fn main() {
    solutions! {
        sol naive {
            let d_set = (2..1001)
                .filter(|&v| {
                    let m = (v as f64).sqrt();
                    m.floor() != m
                });

            let mut max = BigInt::zero();
            let mut maxd = 0;
            for d in d_set {
                let x = get_diophantine(d);

                if x > max {
                    max = x;
                    maxd = d;
                }
            }

            maxd
        }
    }
}

/// x - sqrt(d)y = 1
fn get_diophantine(d: i64) -> BigInt {
    let (start, numbers) = continued_fraction(d);

    let d = d.to_bigint().unwrap();
    let mut top = start.to_bigint().unwrap();
    let mut bot = BigInt::one();
    let mut previous_top = BigInt::one();
    let mut previous_bot = BigInt::zero();

    let mut nums = numbers.iter().cycle();

    while &top*&top - &d*&bot*&bot != BigInt::one() {
        let a = nums.next().unwrap().to_bigint().unwrap();

        let new_top = &a*&top + &previous_top;
        previous_top = top;
        top = new_top;

        let new_bot = &a*&bot + &previous_bot;
        previous_bot = bot;
        bot = new_bot;
    }

    top
}
