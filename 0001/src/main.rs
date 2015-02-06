#[macro_use] extern crate libeuler;

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
