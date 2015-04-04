#[macro_use] extern crate libeuler;
use std::collections::HashMap;

/// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and
/// down, there are exactly 6 routes to the bottom right corner.
///
/// How many such routes are there through a 20×20 grid?
fn main() {
    solutions!{
        inputs: (width: u64 = 20, height: u64 = 20)

        sol naive {
            struct Lattice {
                memo: HashMap<(u64, u64), u64>
            }

            impl Lattice {
                fn new() -> Lattice {
                    Lattice {
                        memo: HashMap::new()
                    }
                }

                fn lattice(&mut self, wid: u64, hei: u64) -> u64 {
                    if wid > hei {
                        return self.lattice(hei, wid);
                    }

                    if self.memo.contains_key(&(wid, hei)) {
                        return self.memo.get(&(wid, hei)).unwrap().clone();
                    }

                    let retval = if wid <= 0 || hei <= 0 {
                        1
                    } else {
                        self.lattice(wid - 1, hei) + self.lattice(wid, hei - 1)
                    };

                    self.memo.insert((wid, hei), retval);

                    retval
                }
            }

            let mut l = Lattice::new();

            l.lattice(width, height)
        }
    };
}

