#![feature(
    collections,
    step_by
)]

extern crate getopts;
pub use getopts::Options;
use std::collections::HashSet;
use std::collections::HashMap;

#[macro_export]
macro_rules! solutions {
    ($(sol $name:ident $content:block)+) => {{
        solutions! {
            inputs: ()

            $(
               sol $name $content
            )+
        }
    }};

    (inputs: ($($n:ident : $ty:ty = $def:expr),*) $(sol $name:ident $content:block)+) => {{
        let args: Vec<String> = ::std::env::args().collect();
        let mut opts = $crate::Options::new();

        $(
            opts.optopt("", stringify!($n), "", "VALUE");
        )*
        opts.optflag("h", "help", "Display this help text");

        let help = || {
            let brief = format!("Usage: {} [options] [versions] \nVersions: {:?}", &args[0], [$(stringify!($name)),+]);
            print!("{}", opts.usage(&brief));
        };

        let matches = match opts.parse(&args[1..]) {
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
        let solutions: Vec<String> = if matches.free.len() > 0 {
            matches.free.iter()
            .filter(|&m| valid_solutions.contains(&m.as_ref()))
            .map(|ref a| a.clone().to_string())
            .collect()
        } else {
            valid_solutions.iter().map(|&a| a.to_string()).collect()
        };

        $(
            if solutions.contains(&stringify!($name).to_string()) {
                println!("Running: {}", stringify!($name));
                println!("Result: {:?}", $name());
                println!("");
            }
        )+
    }};
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

#[derive(Clone)]
pub struct SieveOfAtkin {
    primes: Vec<u64>,
    prime_set: HashSet<u64>
}

impl SieveOfAtkin {
    pub fn new(limit: u64) -> SieveOfAtkin {
        let sqroot = (limit as f64).sqrt() as u64 + 1;

        let mut is_prime = Vec::new();
        is_prime.resize(limit as usize, false);

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

        let mut primes = vec![2u64, 3];
        let mut prime_set = HashSet::new();
        prime_set.insert(2);
        prime_set.insert(3);

        for x in (5..limit).step_by(2) {
            if is_prime[x as usize] {
                for y in (x*x..limit).step_by(x) {
                    is_prime[y as usize] = false;
                }

                primes.push(x as u64);
                prime_set.insert(x as u64);
            }
        }

        SieveOfAtkin {
            primes: primes,
            prime_set: prime_set
        }
    }

    pub fn iter(&self) -> SieveOfAtkinIterator {
        SieveOfAtkinIterator {
            ix: 0,
            sieve: self.clone()
        }
    }

    pub fn factorize(&self, number: u64) -> Vec<u64> {
        let mut retval = Vec::new();
        let mut factorize = number;

        if number <= 0 { return retval; }

        for &p in self.primes.iter() {
            while factorize % p == 0 {
                retval.push(p);
                factorize /= p;
            }

            if factorize == 1 {
                return retval;
            }
        }

        unreachable!();
    }

    pub fn is_prime(&self, number: u64) -> bool {
        self.prime_set.contains(&number)
    }

    // Algorithm found here http://mathschallenge.net/library/number/sum_of_divisors
    pub fn sum_of_divisors(&self, n: u64) -> u64 {
        let factors = self.factorize(n);
        let groups = factors.iter().fold(HashMap::new(), |mut m, &v| {
            let c = m.remove(&v).unwrap_or(0) + 1;
            m.insert(v, c);

            m
        });

        let mut v = 1;

        for (value, &power) in groups.iter() {
            v *= (value.pow(power as u32 + 1) - 1) / (value - 1);
        }

        v
    }

    pub fn sum_of_proper_divisors(&self, n: u64) -> u64 {
        if n == 1 {
            1
        } else {
            self.sum_of_divisors(n) - n
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

pub trait PalindromeHelper {
    fn is_palindrome(&self) -> bool;
}

impl PalindromeHelper for String {
    fn is_palindrome(&self) -> bool {
        let forward = self.chars().take(self.len() / 2);
        let reverse = self.chars().rev().take(self.len() / 2);

        forward.zip(reverse).all(|(a, b)| { a == b })
    }
}

impl PalindromeHelper for i64 {
    fn is_palindrome(&self) -> bool {
        format!("{}", self).is_palindrome()
    }
}
