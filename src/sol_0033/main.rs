#![feature(zero_one)]
#[macro_use] extern crate libeuler;

use std::collections::HashSet;
use std::num::One;
use std::ops::Mul;
/// The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to
/// simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by
/// cancelling the 9s.
///
/// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
///
/// There are exactly four non-trivial examples of this type of fraction, less than one in value,
/// and containing two digits in the numerator and denominator.
///
/// If the product of these four fractions is given in its lowest common terms, find the value of
/// the denominator.
fn main() {
    solutions! {
        sol naive {
            let mut prod = Fraction::one();

            for i in 10..99 {
                for j in (i+1)..99 {
                    let fr = Fraction::new(i, j);
                    if fr.is_curious() {
                        prod = prod * fr;
                    }
                }
            }

            prod
        }
    }
}

#[derive(Debug, Clone)]
struct Fraction {
    numer: i64,
    denom: i64
}

impl Fraction {
    fn new(n: i64, d: i64) -> Fraction {
        Fraction {
            numer: n,
            denom: d
        }
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if a == 0 || b == 0 {
            0
        } else if a == b {
            a
        } else if a > b {
            Fraction::gcd(a - b, b)
        } else {
            Fraction::gcd(a, b - a)
        }
    }

    fn reduced(&self) -> Fraction {
        let gcd = Fraction::gcd(self.numer, self.denom);

        if gcd == 0 {
            Fraction::new(0, 0)
        } else {
            Fraction::new(
                self.numer / gcd,
                self.denom / gcd)
        }
    }

    fn digits(mut a: i64) -> HashSet<u8> {
        let mut r = HashSet::new();
        while a > 0 {
            r.insert((a % 10) as u8);
            a /= 10;
        }

        r
    }

    fn common_digits(&self) -> Vec<u8> {
        Fraction::digits(self.numer).intersection(&Fraction::digits(self.denom)).map(|&a| a).collect()
    }

    fn is_curious(&self) -> bool {
        for &digit in self.common_digits().iter() {
            if digit != 0 {
                if &Fraction::new(self.numer.remove_digit(digit), self.denom.remove_digit(digit)) == self {
                    return true;
                }
            }
        }

        false
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        let a = self.reduced();
        let b = other.reduced();

        a.numer == b.numer && a.denom == b.denom
    }
}

impl One for Fraction {
    fn one() -> Fraction {
        Fraction::new(1, 1)
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Fraction {
        Fraction::new(self.numer * rhs.numer, self.denom * rhs.denom).reduced()
    }
}

trait RemoveDigit {
    fn remove_digit(&self, digit: u8) -> Self;
}

impl RemoveDigit for i64 {
    fn remove_digit(&self, digit: u8) -> i64 {
        format!("{}", self).chars()
            .filter(|c| format!("{}", c) != format!("{}", digit))
            .collect::<String>()
            .parse().ok().unwrap_or(0)
    }
}
