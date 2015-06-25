#[macro_use] extern crate libeuler;

use std::collections::HashMap;
/// The number 145 is well known for the property that the sum of the factorial of its digits is
/// equal to 145:
///
/// 1! + 4! + 5! = 1 + 24 + 120 = 145
///
/// Perhaps less well known is 169, in that it produces the longest chain of numbers that link back
/// to 169; it turns out that there are only three such loops that exist:
///
/// 169 → 363601 → 1454 → 169
///
/// 871 → 45361 → 871
///
/// 872 → 45362 → 872
///
/// It is not difficult to prove that EVERY starting number will eventually get stuck in a loop. For
/// example,
///
/// 69 → 363600 → 1454 → 169 → 363601 (→ 1454)
///
/// 78 → 45360 → 871 → 45361 (→ 871)
///
/// 540 → 145 (→ 145)
///
/// Starting with 69 produces a chain of five non-repeating terms, but the longest non-repeating
/// chain with a starting number below one million is sixty terms.
///
/// How many chains, with a starting number below one million, contain exactly sixty non-repeating
/// terms?
fn main() {
    solutions! {
        sol naive {
            let mut facts = HashMap::new();
            facts.insert(0, 1);
            let mut fact = 1;
            for f in (1..10) {
                fact *= f;
                facts.insert(f, fact);
            }

            let loops = sparse_array! {
                1_000_000,

                169     => 3,
                363601  => 3,
                1454    => 3,
                871     => 2,
                45361   => 2,
                872     => 2,
                45362   => 2
            };

            let mut total = 0;

            for i in 0..1_000_000 {
                let mut v = i;
                let mut count = 0;
                while v >= 1_000_000 || loops[v] == 0 {
                    v = match next(&facts, v) {
                        a if a == v => { break },
                        a => a
                    };

                    count += 1;
                }

                count += loops[v];

                if count == 60 {
                    total += 1;
                }
            }

            total
        }
    }
}

fn next(facts: &HashMap<usize, usize>, i: usize) -> usize {
    let mut v = i;
    let mut ret = 0;
    while v > 0 {
        ret += facts[&(v % 10)];
        v /= 10;
    }

    if ret > 0 { ret } else { 1 }
}
