#![feature(collections)]
#[macro_use] extern crate libeuler;

use libeuler::prime::SieveOfAtkin;
use libeuler::traits::DigitsHelper;
/// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is
/// unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit
/// numbers are permutations of one another.
///
/// There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this
/// property, but there is one other 4-digit increasing sequence.
///
/// What 12-digit number do you form by concatenating the three terms in this sequence?
fn main() {
    solutions! {
        sol naive {
            let sieve = SieveOfAtkin::new(10_000);
            let prime_iterators = sieve.iter()
                .filter(|&p| p > 999).map(|p| PrimePermutationIterator::new(p, &sieve))
                .filter(|iter_option| iter_option.is_some())
                .map(|iter_option| iter_option.unwrap());

            for iter in prime_iterators {
                let len = iter.clone().count();

                if len >= 3 {
                    let askip = len - 2;
                    for i in 0..askip {
                        let mut itra = iter.clone();
                        let a = itra.nth(i).unwrap();

                        if a == 1487 { // Skip known solution
                            continue;
                        }

                        let bskip = len - (i+1) - 1;
                        for j in 0..bskip {
                            let mut itrb = itra.clone();
                            let b = itrb.nth(j).unwrap();
                            let cskip = len - (i+1) - (j+1);
                            for k in 0..cskip {
                                let mut itrc = itrb.clone();
                                let c = itrc.nth(k).unwrap();

                                if b - a == c - b {
                                    return format!("{}{}{}", a, b, c);
                                }
                            }

                        }
                    }
                }
            }

            "".to_string()
        }
    }
}

#[derive(Clone)]
struct PrimePermutationIterator<'a> {
    d: Vec<u8>,
    sieve: &'a SieveOfAtkin,
    first: bool

}

impl<'a> PrimePermutationIterator<'a> {
    fn new(num: u64, sieve: &SieveOfAtkin) -> Option<PrimePermutationIterator> {
        let (d, _) = num.digits();

        let mut d2 = d.clone();
        let mut previous_prime = false;
        while d2.prev_permutation() {
            if sieve.is_prime(d2.iter().fold(0, |v, &i| v*10 + i as u64)) {
                previous_prime = true;
            }
        }

        if sieve.is_prime(num) && !previous_prime {
            Some(PrimePermutationIterator {
                d: d,
                sieve: sieve,
                first: true
            })
        } else {
            None
        }
    }
}

impl<'a> Iterator for PrimePermutationIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.first {
            self.first = false;

            Some(self.d.iter().fold(0, |v, &i| v*10 + i as u64))
        } else {
            while self.d.next_permutation() {
                let v = self.d.iter().fold(0, |v, &i| v*10 + i as u64);

                if self.sieve.is_prime(v) {
                    return Some(v);
                }
            }

            None
        }
    }
}
