#[macro_use] extern crate libeuler;

use libeuler::GonalNumberHelper;

/// Triangle, pentagonal, and hexagonal numbers are generated by the following formulae:
/// Triangle        Tn=n(n+1)/2         1, 3, 6, 10, 15, ...
/// Pentagonal      Pn=n(3n−1)/2        1, 5, 12, 22, 35, ...
/// Hexagonal       Hn=n(2n−1)      1, 6, 15, 28, 45, ...
///
/// It can be verified that T285 = P165 = H143 = 40755.
///
/// Find the next triangle number that is also pentagonal and hexagonal.
fn main() {
    solutions! {
        sol naive {
            let mut ix = 144;

            loop {
                let h = ix*(ix*2 - 1);

                if h.is_triangular() && h.is_pentagonal() {
                    return h;
                }

                ix += 1;
            }
        }
    }
}
