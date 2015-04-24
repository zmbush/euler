#![feature(collections)]
#[macro_use] extern crate libeuler;

use std::usize;
/// Consider the following "magic" 3-gon ring, filled with the numbers 1 to 6, and each line adding
/// to nine.
///
/// Working clockwise, and starting from the group of three with the numerically lowest external
/// node (4,3,2 in this example), each solution can be described uniquely. For example, the above
/// solution can be described by the set: 4,3,2; 6,2,1; 5,1,3.
///
/// It is possible to complete the ring with four different totals: 9, 10, 11, and 12. There are
/// eight solutions in total.
/// Total   Solution Set
/// 9   4,2,3; 5,3,1; 6,1,2
/// 9   4,3,2; 6,2,1; 5,1,3
/// 10  2,3,5; 4,5,1; 6,1,3
/// 10  2,5,3; 6,3,1; 4,1,5
/// 11  1,4,6; 3,6,2; 5,2,4
/// 11  1,6,4; 5,4,2; 3,2,6
/// 12  1,5,6; 2,6,4; 3,4,5
/// 12  1,6,5; 3,5,4; 2,4,6
///
/// By concatenating each group it is possible to form 9-digit strings; the maximum string for a
/// 3-gon ring is 432621513.
///
/// Using the numbers 1 to 10, and depending on arrangements, it is possible to form 16- and
/// 17-digit strings. What is the maximum 16-digit string for a "magic" 5-gon ring?
fn main() {
    solutions! {
        inputs: (size: usize = 5)

        sol naive {
            let mut n: Vec<_> = (0..(size*2)).map(|d| d + 1).collect();
            let mut minlen = usize::MAX;
            let mut max = 0;

            // [r1, r2, r3, r4, r5, e1, e2, e3, e4, e5]
            while n.next_permutation() {
                let ring = &n[0..5];

                if ring.iter().any(|&i| i == 10) {
                    continue;
                }

                let sides: Vec<_> = (0..size)
                    .map(|ix| (n[ix + size], n[ix], n[(ix + 1) % size]))
                    .collect();

                let minside = sides.iter().map(|&(a, _, _)| a).min().unwrap();

                if sides[0].0 != minside {
                    continue;
                }

                let (sameness, total) = sides.iter()
                    .map(|&(a, b, c)| a+b+c)
                    .fold((true, None), |(c, prev), v| {
                        let t = c && (prev.is_none() || prev.unwrap() == v);

                        (t, Some(v))
                    });

                if sameness {
                    println!("{:?} {:?} {}", total, sides, sameness);

                    let d = sides.iter()
                        .map(|&(a, b, c)| format!("{}{}{}", a, b, c))
                        .collect::<Vec<_>>().concat();

                    println!("{} {} / {} {}", d, d.len(), max, minlen);

                    let v: i64 = d.parse().unwrap();
                    if d.len() < minlen || (d.len() == minlen && v > max) {
                        max = v;
                        minlen = d.len();
                    }
                }
            }

            max
        }
    }
}
