#![feature(step_by)]
#[macro_use] extern crate libeuler;

/// Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is
/// called a reduced proper fraction.
///
/// If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:
///
/// 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6,
/// 6/7, 7/8
///
/// It can be seen that there are 21 elements in this set.
///
/// How many elements would be contained in the set of reduced proper fractions for d ≤ 1,000,000?
fn main() {
    solutions! {
        inputs: (max_denom: u64 = 1_000_000)

        sol naive {
            let mut nums = (0..(max_denom+1)).collect::<Vec<u64>>();

            let mut result = 0;
            for denom in 2..(max_denom + 1) {
                if nums[denom as usize] == denom {
                    for next in (denom..(max_denom + 1)).step_by(denom) {
                        nums[next as usize] = nums[next as usize] / denom * (denom - 1);
                    }
                }

                result += nums[denom as usize];
            }

            result
        }
    }
}
