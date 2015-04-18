use std::collections::HashSet;
use num::bigint::{BigInt, ToBigInt};
use num::traits::{Zero, ToPrimitive};

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

macro_rules! palindrome_helper_impl {
    ($($T:ty)+) => ($(
        impl PalindromeHelper for $T {
            fn is_palindrome(&self) -> bool {
                format!("{}", self).is_palindrome()
            }
        }
    )+)
}

palindrome_helper_impl!(u8 i8 u16 i16 u32 i32 u64 i64 isize usize);

impl PalindromeHelper for BigInt {
    fn is_palindrome(&self) -> bool {
        format!("{}", self).is_palindrome()
    }
}

pub trait DigitsHelper: Clone + Sized {
    fn digits(&self) -> (Vec<u8>, HashSet<u8>);
    fn from_digits(&self, Vec<u8>) -> Self;

    fn reverse(&self) -> Self {
        let (mut v, _) = self.digits();
        v.reverse();
        self.from_digits(v)
    }

    fn count_digits(&self) -> usize {
        let (v, _) = self.digits();

        v.len()
    }

    fn is_pandigital(&self) -> bool {
        let (v, s) = self.digits();

        (0..(v.len() as u8)).all(|i| s.contains(&(i+1)))
    }

    fn is_permutation_of(&self, other: &Self) -> bool {
        let (mut va, _) = self.digits();
        let (mut vb, _) = other.digits();
        va.sort();
        vb.sort();

        va == vb
    }

    fn replace_all(&self, value: u8, with: u8) -> Self {
        let (mut va, _) = self.digits();
        
        for i in 0..va.len() {
            if va[i] == value {
                va[i] = with;
            }
        }

        self.from_digits(va)
    }
}

macro_rules! digits_helper_impl {
    ($($T:ty)+) => ($(
        impl DigitsHelper for $T {
            fn digits(&self) -> (Vec<u8>, HashSet<u8>) {
                let mut num = self.clone();
                let mut rv = Vec::new();
                let mut rs = HashSet::new();

                while num > 0 {
                    let n = (num % 10) as u8;
                    rv.push(n);
                    rs.insert(n);
                    num /= 10;
                }

                rv.reverse();

                (rv, rs)
            }

            fn from_digits(&self, digits: Vec<u8>) -> $T {
                digits.iter().fold(0, |r, &d| r*10 + d as $T)
            }
        }
    )+)
}

digits_helper_impl!(u8 i8 u16 i16 u32 i32 i64 u64 isize usize);

impl DigitsHelper for BigInt {
    fn digits(&self) -> (Vec<u8>, HashSet<u8>) {
        let ten = 10.to_bigint().unwrap();
        let mut num = self.clone();
        let mut rv = Vec::new();
        let mut rs = HashSet::new();

        while num > BigInt::zero() {
            let n = (&num % &ten).to_u8().unwrap();
            rv.push(n);
            rs.insert(n);
            num = &num / &ten;
        }

        rv.reverse();

        (rv, rs)
    }

    fn from_digits(&self, digits: Vec<u8>) -> BigInt {
        let ten = 10.to_bigint().unwrap();
        digits.iter().fold(BigInt::zero(), |r, &d| r*(&ten) + d.to_bigint().unwrap())
    }
}

pub trait GonalNumberHelper {
    /// Triangle        T_n=n(n+1)/2         1, 3, 6, 10, 15, ...
    fn is_triangular(&self) -> bool;

    /// Pentagonal      P_n=n(3n−1)/2        1, 5, 12, 22, 35, ...
    fn is_pentagonal(&self) -> bool;

    /// Hexagonal       H_n=n(2n−1)      1, 6, 15, 28, 45, ...
    fn is_hexagonal(&self) -> bool;
}

macro_rules! gonal_number_helper_impl {
    ($($ty:ty)+) => ($(
        impl GonalNumberHelper for $ty {
            fn is_triangular(&self) -> bool {
                let n = 0.5*(((8.0*(*self) as f64) + 1.0).sqrt() - 1.0);
                n.floor() == n
            }

            fn is_pentagonal(&self) -> bool {
                let n = (1.0/6.0) * ((24.0 * ((*self) as f64) + 1.0).sqrt() + 1.0);

                n.floor() == n
            }

            fn is_hexagonal(&self) -> bool {
                let n = 0.25*(((8.0*(*self) as f64) + 1.0).sqrt() + 1.0);
                n.floor() == n
            }
        }
    )+)
}

gonal_number_helper_impl!(u8 i8 u16 i16 u32 i32 i64 u64 isize usize);
