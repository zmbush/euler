#[macro_use] extern crate libeuler;



// You are given the following information, but you may prefer to do some research for yourself.
//
//  1 Jan 1900 was a Monday.
//
//  Thirty days has September,
//  April, June and November.
//  All the rest have thirty-one,
//  Saving February alone,
//  Which has twenty-eight, rain or shine.
//  And on leap years, twenty-nine.
//
//  A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
//
// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

fn main() {
    solutions! {
        inputs: (start_year: u16 = 1900, end_year: u16 = 2000)

        sol naive {
            let start_month = Month::new(0, start_year);

            let mut days = 0u32;
            let mut sundays = 0u32;

            for month in start_month.iter().take_while(|ref m| m.year <= end_year) {
                if days % 7 == 6 {
                    sundays += 1;
                }

                days += month.days();
            }

            sundays
        }
    }
}

#[derive(Clone, Debug)]
struct Month {
    month: u8,
    year: u16
}

impl Month {
    fn new(month: u8, year: u16) -> Month {
        Month {
            month: month,
            year: year
        }
    }

    fn iter(&self) -> MonthIterator {
        MonthIterator {
            current_month: self.clone()
        }
    }

    fn days(&self) -> u32 {
        match self.month {
            3 | 5 | 8 | 10 => 30,
            1 => if self.year % 4 == 0 && self.year % 400 != 0 {
                29
            } else {
                28
            },
            _ => 31
        }
    }
}

struct MonthIterator {
    current_month: Month
}

impl Iterator for MonthIterator {
    type Item = Month;

    fn next(&mut self) -> Option<Month> {
        let retval = self.current_month.clone();

        self.current_month.month += 1;
        self.current_month.month %= 12;
        if self.current_month.month == 0 {
            self.current_month.year += 1;
        }

        return Some(retval);
    }
}
