#![feature(core)]
#[macro_use] extern crate libeuler;

mod arrays;

use std::str::FromStr;

/// In the card game poker, a hand consists of five cards and are ranked, from lowest to highest,
/// in the following way:
///
///     High Card: Highest value card.
///     One Pair: Two cards of the same value.
///     Two Pairs: Two different pairs.
///     Three of a Kind: Three cards of the same value.
///     Straight: All cards are consecutive values.
///     Flush: All cards of the same suit.
///     Full House: Three of a kind and a pair.
///     Four of a Kind: Four cards of the same value.
///     Straight Flush: All cards are consecutive values of same suit.
///     Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
///
/// The cards are valued in the order:
/// 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.
///
/// If two players have the same ranked hands then the rank made up of the highest value wins; for
/// example, a pair of eights beats a pair of fives (see example 1 below). But if two ranks tie,
/// for example, both players have a pair of queens, then highest cards in each hand are compared
/// (see example 4 below); if the highest cards tie then the next highest cards are compared, and
/// so on.
///
/// The file, poker.txt, contains one-thousand random hands dealt to two players. Each line of the
/// file contains ten cards (separated by a single space): the first five are Player 1's cards and
/// the last five are Player 2's cards. You can assume that all hands are valid (no invalid
/// characters or repeated cards), each player's hand is in no specific order, and in each hand
/// there is a clear winner.
///
/// How many hands does Player 1 win?
fn main() {
    solutions! {
        sol naive {

            let hands: Vec<String> = include_str!("poker.txt")
                .split("\n")
                .map(|s| s.to_string())
                .collect();

            hands.iter()
                .map(|h| get_hands(h))
                .filter(|h| h.is_ok())
                .map(|h| h.unwrap())
                .filter(|&(ref p1, ref p2)| {
                    p1.val() < p2.val()
                }).count()
        }
    }
}

fn get_hands(row: &String) -> Result<(Hand, Hand), String> {
    let cards: Vec<Card> = row.split(" ")
        .map(|s| s.parse())
        .filter(|c| c.is_ok())
        .map(|c| c.unwrap())
        .collect();

    if cards.len() < 10 {
        return Err(format!("Not enough cards. Found {}", cards.len()))
    }

    let mut hands = cards.chunks(5);

    let h1 = match hands.next() {
        Some(h) => h,
        None => return Err(format!("Unable to get 5 cards"))
    };

    let h2 = match hands.next() {
        Some(h) => h,
        None => return Err(format!("Unable to get 5 cards"))
    };

    Ok(
        (try!(Hand::new(h1)), try!(Hand::new(h2)))
    )
}

#[derive(Clone, Copy)]
struct Card {
    bits: i64
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Card, String> {
        let err = Err(format!("{} is not a valid card", s));

        if s.len() < 2 {
            return err;
        }

        let rank = match &s[0..1] {
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "T" => 10,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => return err
        };

        let suit = match &s[1..2] {
            "S" => 1,
            "H" => 2,
            "D" => 3,
            "C" => 4,
            _ => return err
        };

        let b_mask = 1 << (14 + rank);
        let cdhs_mask = 1 << (11 + suit);
        let r_mask = (rank - 2) << 8;
        let p_mask = arrays::PRIMES[rank as usize - 2] as i64;

        Ok(Card {
            bits: b_mask | cdhs_mask | r_mask | p_mask
        })
    }
}

struct Hand {
    cards: [Card; 5],
}


impl Hand {
    fn new(vec: &[Card]) -> Result<Hand, String> {
        if vec.len() >= 5 {
            Ok(Hand {
                cards: [vec[0], vec[1], vec[2], vec[3], vec[4]]
            })
        } else {
            Err(format!("Card array has only {} elements", vec.len()))
        }
    }

    // Using Cactus Kev's Poker Hand Evaluator
    // http://suffe.cool/poker/evaluator.html
    fn val(&self) -> i64 {
        let has_flush = self.cards.iter().fold(0xF000, |m, &c| m & c.bits) > 0;
        let q = (self.cards.iter().fold(0, |m, &c| m | c.bits) >> 16) as usize;

        if has_flush {
            arrays::FLUSHES[q] as i64
        } else {
            match arrays::UNIQUE5[q] {
                a if a != 0 => a as i64,
                _ => {
                    let primeprod: i32 = self.cards.iter().map(|c| c.bits & 0xFF).product::<i64>() as i32;

                    match arrays::PRODUCTS.binary_search(&primeprod) {
                        Ok(ix) => arrays::VALUES[ix] as i64,
                        _ => unreachable!()
                    }
                }
            }
        }
    }
}
