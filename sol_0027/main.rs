#[macro_use] extern crate libeuler;

use libeuler::prime::{SieveOfAtkin, SieveOfAtkinIterator};

/// Euler discovered the remarkable quadratic formula:
///
///                                             n² + n + 41
///
/// It turns out that the formula will produce 40 primes for the consecutive values n = 0 to 39.
/// However, when n = 40, 402 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n
/// = 41, 41² + 41 + 41 is clearly divisible by 41.
///
/// The incredible formula  n² − 79n + 1601 was discovered, which produces 80 primes for the
/// consecutive values n = 0 to 79. The product of the coefficients, −79 and 1601, is −126479.
///
/// Considering quadratics of the form:
///
///     n² + an + b, where |a| < 1000 and |b| < 1000
///
///     where |n| is the modulus/absolute value of n
///     e.g. |11| = 11 and |−4| = 4
///
/// Find the product of the coefficients, a and b, for the quadratic expression that produces the
/// maximum number of primes for consecutive values of n, starting with n = 0.
fn main() {
    solutions! {
        sol naive {
            let sieve = SieveOfAtkin::new(1_000_000);
            let mut max_n = 0;
            let mut prod = 0;

            for b in SieveOfAtkinIterator::new(1000) {
                for a in -999..1000i64 {
                    for n in 0..100 {
                        let first = n*n;
                        let second = a*n;
                        let third: i64 = format!("{}", b).parse().unwrap();

                        let v = first+second+third;

                        if v < 0 || !sieve.is_prime(v as u64) {
                            break;
                        }

                        if n > max_n {
                            prod = a*b as i64;
                            max_n = n;
                        }
                    }
                }
            }

            prod
        }
    }
}
