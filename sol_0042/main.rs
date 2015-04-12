#![feature(core)]
#[macro_use] extern crate libeuler;

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
                is_triangle(w.chars()
                            .map(|c| (c as i64) - ('A' as i64) + 1)
                            .sum())
            }).count()
        }
    }
}

//       x = 1/2 * n * (n + 1)
//       2x = n*(n+1)
//       2x = n^2 + n
//       2x = n^2 + n
//       8x = 4n^2 + 4
//       8x + 1 = 4n^2 + 4 + 1
//       8x + 1 = (2n + 1)^2
//       sqrt(8x + 1) = 2n + 1
//       2n = sqrt(8x + 1) - 1
//       n = .5(sqrt(8x + 1) - 1)
fn is_triangle(x: i64) -> bool {
    let n = 0.5*(((8.0*x as f64) + 1.0).sqrt() - 1.0);

    n.floor() == n
}

