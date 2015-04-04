#[macro_use] extern crate libeuler;

fn main() {
    solutions!{
        inputs: (max_value: i64 = 4_000_000)

        sol naive {
            let mut prev = 0;
            let mut value = 1;
            let mut result = 0;

            while value < max_value {
                let tmp = prev+value;
                prev = value;
                value = tmp;

                if value % 2 == 0 {
                    result += value;
                }
            }

            result
        }
    };
}
