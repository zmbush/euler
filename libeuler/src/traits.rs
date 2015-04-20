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
    fn from_digits(Vec<u8>) -> Self;

    fn reverse(&self) -> Self {
        let (mut v, _) = self.digits();
        v.reverse();
        Self::from_digits(v)
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

        Self::from_digits(va)
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

            fn from_digits(digits: Vec<u8>) -> $T {
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

    fn from_digits(digits: Vec<u8>) -> BigInt {
        let ten = 10.to_bigint().unwrap();
        digits.iter().fold(BigInt::zero(), |r, &d| r*(&ten) + d.to_bigint().unwrap())
    }
}

pub trait PolygonalNumber {
    /// n = (1/a)(sqrt(bx + c) + d)
    fn calc_poly(&self, a: f64, b: f64, c: f64, d: f64) -> f64;

    fn is_poly(&self, a: f64, b: f64, c: f64, d: f64) -> bool {
        let n = self.calc_poly(a, b, c, d);
        n.floor() == n
    }

    /// Triangle        T_n=n(n+1)/2         1, 3, 6, 10, 15, ...
    /// n = (1/2)(sqrt(8x + 1) - 1)
    fn is_triangular(&self) -> bool {
        self.is_poly(2.0, 8.0, 1.0, -1.0)
    }

    /// Square          S_n=n^2              1, 4, 9, 15, 25, ...
    /// n = sqrt(x)
    fn is_square(&self) -> bool {
        self.is_poly(1.0, 1.0, 0.0, 0.0)
    }

    /// Pentagonal      P_n=n(3n−1)/2        1, 5, 12, 22, 35, ...
    /// n = (1/6)(sqrt(24x + 1) + 1)
    fn is_pentagonal(&self) -> bool {
        self.is_poly(6.0, 24.0, 1.0, 1.0)
    }

    /// Hexagonal       H_n=n(2n−1)          1, 6, 15, 28, 45, ...
    /// n = (1/4)(sqrt(8x + 1) + 1)
    fn is_hexagonal(&self) -> bool {
        self.is_poly(4.0, 8.0, 1.0, 1.0)
    }

    /// Heptagonal      H_n=n(5n-3)/2        1, 7, 18, 34, 55, ...
    /// n = (1/10)(sqrt(40x + 9) + 3)
    fn is_heptagonal(&self) -> bool {
        self.is_poly(10.0, 40.0, 9.0, 3.0)
    }

    /// Octagonal       O_n=n(3n-2)          1, 8, 21, 40, 65, ...
    /// n = (1/3)(sqrt(3x + 1) + 1)
    fn is_octagonal(&self) -> bool {
        self.is_poly(3.0, 3.0, 1.0, 1.0)
    }
}

macro_rules! polygonal_number_impl {
    ($($ty:ty)+) => ($(
        impl PolygonalNumber for $ty {
            fn calc_poly(&self, a: f64, b: f64, c: f64, d: f64) -> f64 {
                (1.0/a) * ((b*(*self as f64) + c).sqrt() + d)
            }
        }
    )+)
}

polygonal_number_impl!(u8 i8 u16 i16 u32 i32 i64 u64 isize usize);

macro_rules! test_polygonal {
    ($($ident:ident [$($t:expr),*] [$($f:expr),*]),*) => {{
        $(
            $(
                let v = $t;
                println!("{} {}", v, stringify!($ident));
                assert!(v.$ident());
            )*

            $(
                let v = $f;
                println!("{} !{}", v, stringify!($ident));
                assert!(!v.$ident());
            )*
        )*
    }}
}

#[test]
fn test_polygonal_number() {
    test_polygonal! {
        is_triangular [1, 3, 6, 10, 15] [2, 5, 13, 5],
        is_square     [1, 4, 9, 16, 25] [2, 3, 14, 24],
        is_pentagonal [1, 5, 12, 22, 35] [2, 6, 13, 21],
        is_hexagonal  [1, 6, 15, 28, 45] [2, 7, 16, 29, 40],
        is_heptagonal [1, 7, 18, 34, 55, 540, 286] [2, 8, 19, 35, 300, 45],
        is_octagonal  [1, 8, 21, 40, 65] [2, 9, 39, 60, 45]
    }
}

#[test]
fn test_heptagonal() {
    let heptagonal = (1..10000)
        .map(|n| n * (5*n - 3) / 2);

    for i in heptagonal {
        println!("Checking {}", i);
        assert!(i.is_heptagonal());
    }
}
