#[macro_use] extern crate libeuler;

use std::collections::HashMap;

/// In England the currency is made up of pound, £, and pence, p, and there are eight coins in
/// general circulation:
///
///    1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
///
/// It is possible to make £2 in the following way:
///
///    1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
///
/// How many different ways can £2 be made using any number of coins?
fn main() {
    solutions! {
        inputs: (target_pence: u64 = 200)

        sol naive {
            MoneyMaker::new().ways_to_make(target_pence, 200)
        }
    }
}

struct MoneyMaker {
    memo: HashMap<(u64,u64),u64>
}

impl MoneyMaker {
    fn new() -> MoneyMaker {
        MoneyMaker {
            memo: HashMap::new()
        }
    }

    fn ways_to_make(&mut self, pence: u64, max_coin_size: u64) -> u64 {
        if pence == 0 {
            return 1;
        }

        if self.memo.contains_key(&(pence, max_coin_size)) {
            return self.memo[&(pence, max_coin_size)];
        }

        let mut sum = 0;

        for &v in [200,100,50,20,10,5,2,1].iter() {
            if pence >= v && max_coin_size >= v {
                sum += self.ways_to_make(pence - v, v);
            }
        }

        self.memo.insert((pence, max_coin_size), sum);

        sum
    }
}
