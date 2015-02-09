extern crate getopts;
pub use getopts::Options;
use std::collections::HashMap;
use std::num::Float;
use std::num::Int;
use std::iter::range_step;

#[macro_export]
macro_rules! solutions {
    (inputs: ($($n:ident : $ty:ty = $def:expr),*) $(sol $name:ident $content:block)+) => (
        {
            let args: Vec<String> = ::std::os::args();
            let mut opts = $crate::Options::new();

            $(
                opts.optopt("", stringify!($n), "", "VALUE");
            )*
            opts.optflag("h", "help", "Display this help text");

            let help = || {
                let brief = format!("Usage: {} [options] [versions] \nVersions: {:?}", &args[0], [$(stringify!($name)),+]);
                print!("{}", opts.usage(brief.as_slice()));
            };

            let matches = match opts.parse(args.tail()) {
                Ok(m) => m,
                Err(_) => { help(); return; }
            };

            if matches.opt_present("h") {
                help();
                return;
            }

            $(
                let $n: $ty = match matches.opt_str(stringify!($n)) {
                    Some(s) => { s.parse().unwrap_or($def) },
                    None => $def
                };
            )*

            $(
                let $name = || {
                    $content
                };
            )+

            let valid_solutions = vec![$(stringify!($name)),*];
            let solutions: Vec<&String> = matches.free.iter()
                .filter(|&m| valid_solutions.contains(&m.as_slice()))
                .collect();

            $(
                if solutions.len() <= 0 || solutions.contains(&&stringify!($name).to_string()) {
                    println!("Running: {}", stringify!($name));
                    println!("Result: {:?}", $name());
                    println!("");
                }
            )+
        }
    )
}

pub struct PrimeIterator {
    current: i64,
    previous_primes: Vec<i64>
}

impl PrimeIterator {
    pub fn new() -> PrimeIterator {
        PrimeIterator {
            current: 1,
            previous_primes: Vec::new()
        }
    }

    fn is_prime(&self, v: i64) -> bool {
        for &pp in self.previous_primes.iter() {
            if v % pp == 0 {
                return false;
            }
        }

        true
    }
}

impl Iterator for PrimeIterator {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        loop {
            self.current += 1;

            if self.is_prime(self.current) {
                self.previous_primes.push(self.current);
                return Some(self.current);
            }
        }
    }
}

pub struct SieveOfAtkin {
    primes: Vec<u64>
}

impl SieveOfAtkin {
    pub fn new(limit: u64) -> SieveOfAtkin {
        let sqroot = (limit as f64).sqrt() as u64 + 1;

        let mut is_prime = Vec::new();
        is_prime.resize(limit as usize, false);

        let mut primes = vec![2u64, 3];

        {
            let mut invert = |n: u64| {
                is_prime[n as usize] = !is_prime[n as usize];
            };

            for x in 0..sqroot {
                let xp2 = x*x;
                for y in 0..sqroot {
                    let yp2 = y*y;

                    let n = 3*xp2 + yp2;
                    if n <= limit && n % 12 == 7 {
                        invert(n);
                    }

                    let n = n + xp2;
                    if n <= limit && (n % 12 == 1 || n % 12 == 5) {
                        invert(n);
                    }

                    if x > y {
                        let n = n - (xp2 + 2*yp2);
                        if n <= limit && n % 12 == 11 {
                            invert(n);
                        }
                    }
                }
            }
        }

        for x in range_step(5, limit, 2) {
            if (is_prime[x as usize]) {
                for y in range_step(x*x, limit, x) {
                    is_prime[y as usize] = false;
                }

                primes.push(x as u64);
            }
        }

        SieveOfAtkin {
            primes: primes
        }
    }
}

pub struct SieveOfAtkinIterator {
    ix: usize,
    sieve: SieveOfAtkin
}

impl SieveOfAtkinIterator {
    pub fn new(limit: u64) -> SieveOfAtkinIterator {
        SieveOfAtkinIterator {
            ix: 0,
            sieve: SieveOfAtkin::new(limit)
        }
    }
}

impl Iterator for SieveOfAtkinIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.ix += 1;
        if self.ix <= self.sieve.primes.len() {
            let val = self.sieve.primes[self.ix - 1];
            Some(val)
        } else {
            None
        }
    }
}
