#![feature(core)]
#[macro_use] extern crate libeuler;

use libeuler::traits::GonalNumberHelper;
/// The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1); so the first ten
/// triangle numbers are:
///
/// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
///
/// By converting each letter in a word to a number corresponding to its alphabetical position and
/// adding these values we form a word value. For example, the word value for SKY is 19 + 11 + 25 =
/// 55 = t10. If the word value is a triangle number then we shall call the word a triangle word.
///
/// Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly
/// two-thousand common English words, how many are triangle words?
fn main() {
    solutions! {
        sol naive {
            let words: Vec<&str> = include_str!("words.txt")
                .split(",")
                .map(|a| a.trim_matches(&['"'] as &[char]))
                .collect();

            words.iter().filter(|w| {
                w.chars()
                    .map(|c| (c as i64) - ('A' as i64) + 1)
                    .sum::<i64>()
                    .is_triangular()
            }).count()
        }
    }
}
