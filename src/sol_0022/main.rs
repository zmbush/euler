#![feature(iter_arith)]
#[macro_use] extern crate libeuler;

// Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over
// five-thousand first names, begin by sorting it into alphabetical order. Then working out the
// alphabetical value for each name, multiply this value by its alphabetical position in the list
// to obtain a name score.
//
// For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12
// + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 =
// 49714.
//
// What is the total of all the name scores in the file?

fn main() {
    let mut names: Vec<&str> = include_str!("names.txt")
        .split(",")
        .map(|a| a.trim_matches(&['"'] as &[char]))
        .collect();
    names.sort();

    solutions! {
        sol naive {
            names.iter().zip(0..names.len()).map(|(&name, index)| {
                name.chars()
                    .map(|c| c as u64 - 'A' as u64 + 1)
                    .sum::<u64>() * (index as u64 + 1)
            }).sum::<u64>()
        }
    }
}
