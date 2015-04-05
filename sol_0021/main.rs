#[macro_use] extern crate libeuler;
extern crate num;
use libeuler::SieveOfAtkin;

/// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly
/// into n).
/// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b
/// are called amicable numbers.
///
/// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
/// therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
///
/// Evaluate the sum of all the amicable numbers under 10000.
fn main() {
    solutions!{
        inputs: (max_number: u64 = 10_000)

        sol naive {
            let sieve = SieveOfAtkin::new(2_000_000);

            let d = |n: u64| sieve.sum_of_proper_divisors(n);

            let mut sum = 0;
            for a in 1..max_number {
                let b = d(a);
                if b > a && d(b) == a {
                    println!("d({a}) = {b}; d({b}) = {a}", a = a, b = b);
                    sum += a + b;
                }
            }

            sum
        }
    };
}
