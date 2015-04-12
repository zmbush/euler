#![feature(collections, core)]
#[macro_use] extern crate libeuler;

/// The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the
/// digits 0 to 9 in some order, but it also has a rather interesting sub-string divisibility
/// property.
///
/// Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:
///
///   d2d3d4=406 is divisible by 2
///   d3d4d5=063 is divisible by 3
///   d4d5d6=635 is divisible by 5
///   d5d6d7=357 is divisible by 7
///   d6d7d8=572 is divisible by 11
///   d7d8d9=728 is divisible by 13
///   d8d9d10=289 is divisible by 17
///
/// Find the sum of all 0 to 9 pandigital numbers with this property.
fn main() {
    solutions! {
        sol naive {
            let digits: Vec<i64> = (0..10).collect();

            digits.permutations()
                .filter(|p| has_property(p))
                .map(|p| p.iter().fold(0, |v, &d| v*10 + d))
                .sum::<i64>()
        }
    }
}

fn divisible_by(num: i64, divisor: i64) -> bool {
    let n = (num as f64) / (divisor as f64);

    n.floor() == n
}

fn combine(num: &Vec<i64>, ix: usize) -> i64 {
    num[ix]*100 + num[ix+1]*10 + num[ix+2]
}

fn has_property(num: &Vec<i64>) -> bool {
    num[0] != 0 &&
    divisible_by(combine(num, 1), 2) &&
    divisible_by(combine(num, 2), 3) &&
    divisible_by(combine(num, 3), 5) &&
    divisible_by(combine(num, 4), 7) &&
    divisible_by(combine(num, 5), 11) &&
    divisible_by(combine(num, 6), 13) &&
    divisible_by(combine(num, 7), 17)
}
