#![feature(iter_arith)]
#[macro_use] extern crate libeuler;

use std::collections::HashSet;
use libeuler::traits::DigitsHelper;
/// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n
/// exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
///
/// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand,
/// multiplier, and product is 1 through 9 pandigital.
///
/// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a
/// 1 through 9 pandigital.
///
/// HINT: Some products can be obtained in more than one way so be sure to only include it once in
/// your sum.
fn main() {
    solutions! {
        sol naive {
            let mut products = HashSet::new();

            for i in 1..2000 {
                if without_repeats(i) {
                    for j in i..2000 {
                        if without_repeats(j) && pandigital(i, j) {
                            products.insert(i*j);
                        }
                    }
                }
            }

            products.iter().map(|&a| a).sum::<i64>()
        }
    }
}

fn get_digits(num: i64) -> (HashSet<u8>, usize) {
    let (v, s) = num.digits();
    (s, v.len())
}

fn without_repeats(a: i64) -> bool {
    let (a_digits, a_count) = get_digits(a);

    a_digits.len() == a_count && !a_digits.contains(&0)
}

fn pandigital(a: i64, b: i64) -> bool {
    let (a_digits, a_count) = get_digits(a);
    let (b_digits, b_count) = get_digits(b);

    let c_count = ((a as f32).log10() + (b as f32).log10()) as usize + 1;

    if a_count + b_count + c_count != 9 {
        return false;
    }

    let (c_digits, c_count) = get_digits(a*b);

    if a_digits.len() != a_count ||
            b_digits.len() != b_count ||
            c_digits.len() != c_count ||
            a_digits.contains(&0) ||
            b_digits.contains(&0) ||
            c_digits.contains(&0) {
        false
    } else {
        a_digits.is_disjoint(&b_digits) &&
        a_digits.is_disjoint(&c_digits) &&
        b_digits.is_disjoint(&c_digits) &&
        a_count + b_count + c_count == 9
    }
}
