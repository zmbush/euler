#[macro_use] extern crate libeuler;

/// Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is
/// called a reduced proper fraction.
///
/// If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:
///
/// 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6,
/// 6/7, 7/8
///
/// It can be seen that 2/5 is the fraction immediately to the left of 3/7.
///
/// By listing the set of reduced proper fractions for d ≤ 1,000,000 in ascending order of size,
/// find the numerator of the fraction immediately to the left of 3/7.
fn main() {
    solutions! {
        inputs: (max_denom: i64 = 1_000_000, target_top: i64 = 3, target_bot: i64 = 7)

        sol naive {
            let mut best_top = 0;
            let mut best_bot = 1;

            for nth in 0..max_denom {
                let denom = max_denom - nth;
                let numer = (denom * target_top - 1) / target_bot;

                // n/d > bt/bb => n*bb > bt*d
                if numer * best_bot > best_top * denom {
                    best_top = numer;
                    best_bot = denom;
                }
            }

            best_top
        }
    }
}
