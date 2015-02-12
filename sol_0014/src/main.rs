#[macro_use] extern crate libeuler;
use std::collections::HashMap;

/// The following iterative sequence is defined for the set of positive integers:
///
///   n -> n/2 (n is even)
///   n -> 3n + 1 (n is odd)
///
/// Using the rule above and starting with 13, we generate the following sequence:
/// 13 →  40 →  20 →  10 →  5 →  16 →  8 →  4 →  2 →  1
///
/// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
/// Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers
/// finish at 1.
///
/// Which starting number, under one million, produces the longest chain?
///
/// NOTE: Once the chain starts the terms are allowed to go above one million.
fn main() {
    solutions!{
        inputs: (max_value: u64 = 1_000_000)

        sol naive {
            struct CollatzCounter {
                cache: HashMap<u64, u64>
            }

            impl CollatzCounter {
                fn new() -> CollatzCounter {
                    CollatzCounter {
                        cache: HashMap::new()
                    }
                }

                fn count(&mut self, n: u64) -> u64 {
                    if n < 1 { return 0; }
                    if n == 1 { return 1; }

                    if !self.cache.contains_key(&n) {
                        let value = match n % 2 == 0 {
                            true => self.count(n / 2) + 1,
                            false => self.count(3*n + 1) + 1
                        };

                        self.cache.insert(n, value);
                    }

                    self.cache[n]
                }
            }

            let mut collatz = CollatzCounter::new();

            let mut max = 0;
            let mut max_int = 0;

            for i in 0..max_value {
                let count = collatz.count(i);
                if count > max {
                    max = count;
                    max_int = i;
                }
            }

            println!("Cache size: {}", collatz.cache.len());

            max_int
        }
    };
}
