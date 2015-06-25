#[macro_use] extern crate libeuler;

/// Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is
/// called a reduced proper fraction.
///
/// If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:
///
/// 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6,
/// 6/7, 7/8
///
/// It can be seen that there are 3 fractions between 1/3 and 1/2.
///
/// How many fractions lie between 1/3 and 1/2 in the sorted set of reduced proper fractions for d
/// ≤ 12,000?
fn main() {
    solutions! {
        inputs: (max_denom: i64 = 12_000)

        sol naive {
            let mut i = I { a: 1,       b: 3,
                            c: 4000,    d: 11999 };

            let mut count = 0;

            while i.c != 1 || i.d != 2 {
                i = i.next(max_denom);
                count += 1;
            }

            count
        }
    }
}

struct I {
    a: i64,
    b: i64,
    c: i64,
    d: i64
}

impl I {
    fn next(self, n: i64) -> I {
        let k = (n + self.b) / self.d;
        I {
            a: self.c,
            b: self.d,
            c: k*self.c - self.a,
            d: k*self.d - self.b
        }
    }
}
