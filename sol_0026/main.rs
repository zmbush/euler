#[macro_use] extern crate libeuler;

use std::collections::HashMap;

/// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions
/// with denominators 2 to 10 are given:
///
///     1/2     =   0.5
///     1/3     =   0.(3)
///     1/4     =   0.25
///     1/5     =   0.2
///     1/6     =   0.1(6)
///     1/7     =   0.(142857)
///     1/8     =   0.125
///     1/9     =   0.(1)
///     1/10    =   0.1
///
/// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has
/// a 6-digit recurring cycle.
///
/// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal
/// fraction part.
fn main() {
    solutions! {
        inputs: (upper_range: i32 = 1000)

        sol naive {
            match (2..upper_range).map(|i| (count_repeats(i), i)).max() {
                Some((_, index)) => index,
                None => 0
            }
        }
    }
}

fn count_repeats(d: i32) -> i32 {
    let mut previous_states = HashMap::new();

    let mut val = 1;
    let mut i = 0;

    while val > 0 {
        i += 1;

        if previous_states.contains_key(&val) {
            return i - previous_states[&val];
        }

        previous_states.insert(val, i);

        val *= 10;
        if d < val {
            val %= d;
        }
    }

    0
}
