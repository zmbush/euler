#![feature(iter_arith)]
#[macro_use] extern crate libeuler;
extern crate num;

use num::rational::BigRational;
use num::bigint::ToBigInt;
use libeuler::traits::DigitsHelper;
/// What is most surprising is that the important mathematical constant,
/// e = [2; 1,2,1, 1,4,1, 1,6,1 , ... , 1,2k,1, ...].
///
/// The first ten terms in the sequence of convergents for e are:
/// 2, 3, 8/3, 11/4, 19/7, 87/32, 106/39, 193/71, 1264/465, 1457/536, ...
///
/// The sum of digits in the numerator of the 10th convergent is 1+4+5+7=17.
///
/// Find the sum of digits in the numerator of the 100th convergent of the continued fraction for
/// e.
fn main() {
    solutions! {
        inputs: (convergent: usize = 100)

        sol naive {
            let mut items = {
                let mut i = (1..)
                    .flat_map(|k| vec![1, 2*k, 1].into_iter())
                    .take(convergent - 1)
                    .collect::<Vec<i64>>();
                i.reverse();

                i.into_iter()
            };

            let mut num = BigRational::from_integer(items.next().unwrap().to_bigint().unwrap());
            for i in items {
                num = num.recip() + BigRational::from_integer(i.to_bigint().unwrap());
            }

            num = num.recip() + BigRational::from_integer(2.to_bigint().unwrap());

            num.numer().digits().0.iter().map(|&d| d as i64).sum::<i64>()
        }
    }
}
