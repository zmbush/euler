#[macro_use] extern crate libeuler;

/// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 +
/// 3 + 5 + 4 + 4 = 19 letters used in total.
///
/// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many
/// letters would be used?
///
/// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains
/// 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing
///    out numbers is in compliance with British usage.
fn main() {
    solutions!{
        inputs: (max_number: u64 = 1_000)

        sol naive {
            fn wordify(v: u64) -> String {
                format!("{}", match v {
                    0 => "".to_string(),
                    1 => "one".to_string(),
                    2 => "two".to_string(),
                    3 => "three".to_string(),
                    4 => "four".to_string(),
                    5 => "five".to_string(),
                    6 => "six".to_string(),
                    7 => "seven".to_string(),
                    8 => "eight".to_string(),
                    9 => "nine".to_string(),
                    10 => "ten".to_string(),
                    11 => "eleven".to_string(),
                    12 => "twelve".to_string(),
                    13 => "thirteen".to_string(),
                    14 => "fourteen".to_string(),
                    15 => "fifteen".to_string(),
                    16 => "sixteen".to_string(),
                    17 => "seventeen".to_string(),
                    18 => "eighteen".to_string(),
                    19 => "nineteen".to_string(),
                    n if n < 100 => {
                        format!("{}{}", match n / 10 {
                            2 => "twenty",
                            3 => "thirty",
                            4 => "forty",
                            5 => "fifty",
                            6 => "sixty",
                            7 => "seventy",
                            8 => "eighty",
                            9 => "ninety",
                            _ => unreachable!()
                        }, wordify(n % 10))
                    },
                    n if n < 1000 => {
                        let hundreds = n / 100;
                        let amount = n % 100;

                        format!("{}hundred{}", wordify(hundreds), match amount {
                            0 => "".to_string(),
                            n if n < 10 => format!("and{}", wordify(n)),
                            n if n % 10 == 0 => format!("and{}", wordify(n)),
                            n if n < 20 => format!("and{}", wordify(n)),
                            _ => format!("and{}", wordify(amount))
                        })
                    },
                    n if n < 100_000 => {
                        let thousands = n / 1000;

                        format!("{}thousand{}", wordify(thousands), wordify(n % 1000))
                    },
                    _ => unreachable!()
                })
            }

            let mut count = 0;
            for i in 0..max_number {
                count += wordify(i + 1).len();
            }

            count
        }
    };
}
