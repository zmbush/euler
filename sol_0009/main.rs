#[macro_use] extern crate libeuler;
use std::num::Int;
use std::iter::MultiplicativeIterator;



/// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
/// a^2 + b^2 = c^2
///
/// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
///
/// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
/// Find the product abc.
fn main() {
    solutions!{
        inputs: (sum: i64 = 1000)

        sol naive {
            for a in 1..(sum + 1) {
                let apow = a.pow(2);

                for b in (a + 1)..(sum - a + 1) {
                    let left = apow + b.pow(2);

                    for c in (b + 1)..(sum - b - a + 1) {
                        if a + b + c == sum {
                            if left == c.pow(2) {
                                return Some(a * b * c);
                            }
                        }
                    }
                }
            }

            return None;
        }
    };
}
