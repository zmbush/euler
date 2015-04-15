use std::collections::HashSet;

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

palindrome_helper_impl!(u8 i8 u16 i16 u32 i32 u64 i64);

pub trait DigitsHelper {
    fn digits(&self) -> (Vec<u8>, HashSet<u8>);

    fn count_digits(&self) -> usize {
        let (v, _) = self.digits();

        v.len()
    }

    fn is_pandigital(&self) -> bool {
        let (v, s) = self.digits();

        (0..(v.len() as u8)).all(|i| s.contains(&(i+1)))
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
        }
    )+)
}

digits_helper_impl!(u8 i8 u16 i16 u32 i32 i64 u64);

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

gonal_number_helper_impl!(u8 i8 u16 i16 u32 i32 i64 u64);