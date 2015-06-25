#![feature(iter_arith)]
#[macro_use] extern crate libeuler;

use std::collections::HashMap;
/// 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
///
/// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
///
/// Note: as 1! = 1 and 2! = 2 are not sums they are not included.
fn main() {
    solutions! {
        sol naive {
            let (_, factorials) = (0..10).fold((1, HashMap::new()), |(n, mut map), v| {
                let next = if v == 0 { 1 } else { n * v };
                map.insert(v, next);

                (next, map)
            });

            (3i64..1_000_000).filter(|&i| i == digits(i).iter().fold(0, |i, v| i + factorials[v])).sum::<i64>()
        }
    }
}

fn digits(mut i: i64) -> Vec<i64> {
    let mut ret = Vec::new();

    while i > 0 {
        ret.push(i % 10);
        i /= 10;
    }

    ret
}
