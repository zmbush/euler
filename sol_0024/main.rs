#![feature(collections)]
#[macro_use] extern crate libeuler;

use std::collections::HashMap;
use std::slice::SliceConcatExt;

/// A permutation is an ordered arrangement of objects. For example, 3124 is one possible
/// permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or
/// alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2
/// are:
///
///                              012   021   102   120   201   210
///
/// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
fn main() {
    solutions! {
        inputs: (max_digit: u64 = 9)

        sol naive {
            let digits: Vec<u64> = (0..max_digit+1).collect();

            let result: Vec<String> = find_perm(&digits, 1_000_000, 0).ok().expect("FUUUU").iter().map(|a| format!("{}", a)).collect();

            result.connect("").parse::<u64>()
        }
    }
}

fn find_perm(vals: &Vec<u64>, target: u64, mut current: u64) -> Result<Vec<u64>, u64> {
    if vals.len() <= 1 {
        if current + 1 == target {
            return Ok(vals.clone());
        } else {
            return Err(current + 1);
        }
    }

    for i in 0..vals.len() {
        let mut remain = vals.clone();
        let v = remain.remove(i);
        match find_perm(&remain, target, current) {
            Ok(result) => {
                let mut r = result.clone();
                r.insert(0, v);
                return Ok(r);
            },
            Err(new_curr) => {
                current = new_curr
            }
        }
    }

    Err(current)
}

