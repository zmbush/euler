#[macro_use] extern crate libeuler;

/// The sum of the squares of the first ten natural numbers is,
/// 1^2 + 2^2 + ... + 10^2 = 385
///
/// The square of the sum of the first ten natural numbers is,
/// (1 + 2 + ... + 10)^2 = 552 = 3025
///
/// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
///
/// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
fn main() {
    solutions!{
        inputs: (numbers: i64 = 100)

        sol naive {
            let mut sumsq = 0;
            let mut sum = 0;

            for i in 0..numbers {
                let num = i + 1;
                sumsq += num.pow(2);
                sum += num;
            }

            let sqsum = sum.pow(2);

            sqsum - sumsq
        }
    };
}
