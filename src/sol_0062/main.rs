#[macro_use] extern crate libeuler;
extern crate num;

use libeuler::traits::DigitsHelper;
use num::bigint::{BigInt, ToBigInt};
use std::collections::HashMap;
/// The cube, 41063625 (345^3), can be permuted to produce two other cubes: 56623104 (384^3) and
/// 66430125 (405^3). In fact, 41063625 is the smallest cube which has exactly three permutations of
/// its digits which are also cube.
///
/// Find the smallest cube for which exactly five permutations of its digits are cube.
fn main() {
    solutions! {
        sol naive {
            let mut memo = HashMap::new();
            for i in 345.. {
                let cub = num::pow(i.to_bigint().unwrap(), 3);

                let (first, count) = get(&mut memo, &cub);

                if count >= 5 {
                    return format!("{}", first);
                }
            }

            unreachable!();
        }
    }
}

fn get(memo: &mut HashMap<Vec<u8>, (BigInt, i64)>, i: &BigInt) -> (BigInt, i64) {
    let (mut key, _) = i.digits();
    key.sort();

    let (first, count) = memo.remove(&key).unwrap_or((i.clone(), 0));
    memo.insert(key, (first.clone(), count + 1));

    (first, count + 1)
}
