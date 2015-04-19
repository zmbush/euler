#![feature(core)]
#[macro_use] extern crate libeuler;

use libeuler::prime::SieveOfAtkin;
/// The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes and concatenating
/// them in any order the result will always be prime. For example, taking 7 and 109, both 7109 and
/// 1097 are prime. The sum of these four primes, 792, represents the lowest sum for a set of four
/// primes with this property.
///
/// Find the lowest sum for a set of five primes for which any two primes concatenate to
/// produce another prime.
fn main() {
    solutions! {
        sol naive {
            let sieve = SieveOfAtkin::new(100_000_000);

            match find_remarkable_set(&sieve, vec![13], 1000, 5) {
                Some(set) => set.iter().map(|&a| a).sum(),
                _ => 0u64
            }
        }
    }
}

fn find_remarkable_set(sieve: &SieveOfAtkin, mut set: Vec<u64>, start: u64, needed: usize) -> Option<Vec<u64>> {
    for prim in sieve.iter().filter(|&p| p > start).filter(|&p| p < 9000) {
        // print!("Add {} to {:?}? ", prim, set);
        let mut valid = true;
        for i in set.iter() {
            if !sieve.is_prime(format!("{}{}", i, prim).parse().unwrap()) ||
               !sieve.is_prime(format!("{}{}", prim, i).parse().unwrap()) {
                valid = false;
                // println!("No");
                break;
            }
        }

        if valid {
            // println!("Yes");
            set.push(prim);

            if set.len() >= needed {
                return Some(set);
            }

            match find_remarkable_set(sieve, set.clone(), prim, needed) {
                Some(v) => return Some(v),
                _ => {}
            }

            set.pop();
        }
    }

    None
}
