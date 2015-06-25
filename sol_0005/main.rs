#![feature(slice_position_elem, iter_arith)]
#[macro_use] extern crate libeuler;

/// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
///
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
fn main() {
    solutions!{
        inputs: (max_factor: i64 = 20)

        sol naive {
            let primes = prime_factors_less_than(&max_factor);
            let mut needed_factors = Vec::new();

            for factors in primes.iter() {
                let mut f = needed_factors.clone();
                let still_needed: Vec<&i64> = factors.iter()
                    .filter(|&fac| {
                        if f.contains(fac) {
                            let pos = f.position_elem(fac).unwrap();
                            f.swap_remove(pos);
                            false
                        } else {
                            true
                        }
                    }).collect();

                for v in still_needed {
                    needed_factors.push(v.clone());
                }
            }

            needed_factors.iter().map(|&i| i).product::<i64>()
        }
    };
}

fn factors(value: &i64) -> Vec<i64> {
    let mut factor = 2;
    let mut v = value.clone();
    let mut retval = Vec::new();

    while v > 1 {
        if v % factor == 0 {
            retval.push(factor);
            v /= factor;
        } else {
            factor += 1;
        }
    }

    retval
}

fn prime_factors_less_than(max: &i64) -> Vec<Vec<i64>> {
    let mut retval = Vec::new();
    for i in 1..*max {
        retval.push(factors(&i));
    }

    retval
}
