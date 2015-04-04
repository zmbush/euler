#[macro_use] extern crate libeuler;
extern crate num;
use libeuler::SieveOfAtkin;
use std::collections::HashMap;

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
            // Algorithm found here http://mathschallenge.net/library/number/sum_of_divisors
            let d = |n: u64| {
                let factors = sieve.factorize(n);
                let groups = factors.iter().fold(HashMap::new(), |mut m, &v| {
                    let c = m.remove(&v).unwrap_or(0) + 1;
                    m.insert(v, c);

                    m
                });


                let mut v = 1;
                for (value, &power) in groups.iter() {
                    v *= (value.pow(power as u32 + 1) - 1) / (value - 1);
                }

                if n != 1 {
                    v - n
                } else {
                    v
                }
            };

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
