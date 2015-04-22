#[macro_use] extern crate libeuler;

use std::collections::HashSet;
/// All square roots are periodic when written as continued fractions and can be written in the form:
/// √N = a0 +          1
///           -----------------
///           a1 +       1
///               -------------
///               a2 +     1
///                    --------
///                    a3 + ...
///
/// For example, let us consider √23:
/// √23 = 4 + √23 — 4 = 4 +   1   = 4 +       1
///                         -----       -----------
///                           1         1 + √23 – 3
///                         -----           -------
///                         √23—4              7
///
/// If we continue we would get the following expansion:
/// √23 = 4 +           1
///           ------------------
///           1 +         1
///               --------------
///               3 +      1
///                   ----------
///                   1 +   1
///                      -------
///                      8 + ...
///
/// It can be seen that the sequence is repeating. For conciseness, we use the notation √23 =
/// [4;(1,3,1,8)], to indicate that the block (1,3,1,8) repeats indefinitely.
///
/// The first ten continued fraction representations of (irrational) square roots are:
///
/// √2=[1;(2)], period=1
/// √3=[1;(1,2)], period=2
/// √5=[2;(4)], period=1
/// √6=[2;(2,4)], period=2
/// √7=[2;(1,1,1,4)], period=4
/// √8=[2;(1,4)], period=2
/// √10=[3;(6)], period=1
/// √11=[3;(3,6)], period=2
/// √12= [3;(2,6)], period=2
/// √13=[3;(1,1,1,1,6)], period=5
///
/// Exactly four continued fractions, for N ≤ 13, have an odd period.
///
/// How many continued fractions for N ≤ 10000 have an odd period?
fn main() {
    solutions! {
        inputs: (max_n: i64 = 10_000)

        sol naive {
            let squares: Vec<(i64, i64)> = (0..).zip(
                    (0..)
                        .map(|n| n*n)
                        .take_while(|&n| n <= max_n)
                ).collect();

            let mut count = 0;

            for subject in 1..(max_n + 1) {
                let &(base, value) = squares.iter()
                    .filter(|&&(_, pow)| pow <= subject)
                    .last()
                    .unwrap();

                if value == subject {
                    continue;
                }

                let mut top = 1;
                let mut bot_number = -base;

                let mut seen_states = HashSet::new();
                let mut period = 0;

                for _ in 0.. {
                    let key = (bot_number, top);

                    if seen_states.contains(&key) {
                        break;
                    }
                    seen_states.insert(key);

                    top = (subject - bot_number*bot_number) / top;

                    let mut to_subtract = 0;

                    while -bot_number - to_subtract - top >= -base {
                        to_subtract += top;
                    }

                    bot_number = -bot_number - to_subtract;

                    period += 1;
                }

                if period % 2 != 0 {
                    count += 1;
                }
            }

            count
        }
    }
}
