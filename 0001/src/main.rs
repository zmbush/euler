#[macro_use] extern crate libeuler;

/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The
/// sum of these multiples is 23.
///
/// Find the sum of all the multiples of 3 or 5 below 1000.
fn main() {
    solutions!{
        inputs: (max: i64 = 1000, fac1: i64 = 3, fac2: i64 = 5)

        sol naive {
            let mut sum = 0i64;

            for i in 0..max {
                sum += match i {
                    i if (i % fac1 == 0 || i % fac2 == 0) => i,
                    _ => 0
                }
            }

            sum
        }

        sol broken {
            -5
        }
    };
}
