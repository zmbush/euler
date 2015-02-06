extern crate getopts;
use getopts::Options;
use std::os;

macro_rules! solutions {
    (inputs: ($($n:ident : $ty:ty = $def:expr),*) $(sol $name:ident $content:block)+) => (
        {
            let args: Vec<String> = os::args();
            let mut opts = Options::new();

            $(
                opts.optopt("", stringify!($n), "algorithm argument", "VALUE");
            )*

            let matches = match opts.parse(args.tail()) {
                Ok(m) => m,
                Err(f) => { panic!(f.to_string()) }
            };

            $(
                let $n: $ty = match matches.opt_str(stringify!($n)) {
                    Some(s) => { s.parse().unwrap_or($def) },
                    None => $def
                };
            )*

            $(
                let $name = || {
                    $content
                };
            )+
            $(
                println!("Running: {}", stringify!($name));
                println!("Result: {:?}", $name());
                println!("");
            )+
        }
    )
}


fn main() {
    solutions!{
        inputs: (max: i64 = 1000, fac1: i64 = 3, fac2: i64 = 5)

        sol naive {
            let mut sum = 0i64;

            for i in 0..max {
                sum += match i {
                    i if (i % fac1 == 0 || i % fac2 == 0) => i,
                    _ => 0
                }
            }

            sum
        }

        sol broken {
            -5
        }
    };
}
