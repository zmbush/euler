use std::collections::{HashSet, HashMap};

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

    pub fn factors(&self, number: u64) -> HashSet<u64> {
        let mut retval = HashSet::new();
        for factor in self.factorize(number) {
            retval.insert(factor);
        }
        retval
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

use num::traits::ToPrimitive;
pub fn is_prime<T: ToPrimitive>(i: T) -> bool {
    let num = i.to_i64().unwrap();

    for i in 2..(num.to_f64().unwrap().sqrt().floor() as i64 + 1) {
        if num % i == 0 {
            return false;
        }
    }

    true
}
