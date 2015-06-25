#![feature(iter_arith, range_inclusive)]
#[macro_use] extern crate libeuler;

use std::collections::HashMap;
use std::iter::range_inclusive;
/// There are exactly ten ways of selecting three from five, 12345:
///
/// 123, 124, 125, 134, 135, 145, 234, 235, 245, and 345
///
/// In combinatorics, we use the notation, 5C3 = 10.
///
/// In general,
/// nCr = n! / (r!(n−r)!)
///
/// where r ≤ n, n! = n×(n−1)×...×3×2×1, and 0! = 1.
///
/// It is not until n = 23, that a value exceeds one-million: 23C10 = 1144066.
///
/// How many, not necessarily distinct, values of  nCr, for 1 ≤ n ≤ 100, are greater than
/// one-million?
fn main() {
    solutions! {
        inputs: (min_val: i64 = 1_000_000)

        sol naive {
            let mut mem = HashMap::new();

            range_inclusive(1, 100).map(|n| {
                range_inclusive(0, n).map(|r| {
                    match ncr(&mut mem, n, r) {
                        Num::TooBig => 1,
                        Num::N(a) if a > min_val => 1,
                        _ => 0
                    }
                }).sum::<i64>()
            }).sum::<i64>()
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Num {
    TooBig,
    N(i64)
}

fn ncr(memo: &mut HashMap<(i64, i64), Num>, n: i64, r: i64) -> Num {
    use Num::*;

    if memo.contains_key(&(n, r)) {
        return memo[&(n, r)];
    }

    let proto_ret = match (n, r) {
        (_, 0) => N(1),
        (n, r) if n == r => N(1),
        (n, r) => match (ncr(memo, n-1, r-1), ncr(memo, n-1, r)) {
            (TooBig, _) => TooBig,
            (_, TooBig) => TooBig,
            (N(a), N(b)) => N(a + b)
        }
    };

    let ret = match proto_ret {
        N(a) if a > 1_000_000_000 => TooBig,
        a => a
    };

    memo.insert((n,r), ret);

    ret
}
