#![feature(collections)]
#[macro_use] extern crate libeuler;

use libeuler::DigitsHelper;
use std::collections::HashSet;

/// Take the number 192 and multiply it by each of 1, 2, and 3:
///
///     192 × 1 = 192
///     192 × 2 = 384
///     192 × 3 = 576
///
/// By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576
/// the concatenated product of 192 and (1,2,3)
///
/// The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5,
/// giving the pandigital, 918273645, which is the concatenated product of 9 and
/// (1,2,3,4,5).
///
/// What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated
/// product of an integer with (1,2, ... , n) where n > 1?
fn main() {
    solutions! {
        sol naive {
            let mut max = 0;
            for i in 0..1_000_000 {
                match get_pandigital_concat(i) {
                    Some(v) => {
                        if v > max {
                            println!("{} => {}", i, v);
                            max = v
                        }
                    },
                    _ => {}
                }
            }

            max
        }
    }
}

fn get_pandigital_concat(num: i64) -> Option<i64> {
    if num <= 0 { return None }

    let mut i = 0;
    let mut udigits = HashSet::new();
    let mut adigits = Vec::new();

    while udigits.len() == adigits.len() && adigits.len() < 9 {
        i += 1;

        let p = num * i;
        let (mut v, s) = p.digits();
        udigits = udigits.union(&s).map(|&v| v).collect();
        adigits.append(&mut v);
    }

    if adigits.len() == 9 && udigits.len() == 9 && i > 1 && !udigits.contains(&0) {
        Some(adigits.iter().fold(0, |v, &d| v * 10 + (d as i64)))
    } else {
        None
    }
}
