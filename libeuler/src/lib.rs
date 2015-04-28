#![feature(
    collections,
    step_by
)]

extern crate getopts;
extern crate time;
extern crate num;
use std::fmt::Debug;

pub use getopts::Options;
pub mod prime;
pub mod traits;
pub mod irrational;

pub fn time_execution<F: Fn() -> R, R: Debug>(f: F) -> R {
    let start = time::precise_time_s();
    let retval = f();
    let diff = time::precise_time_s() - start;
    println!("Time: {} seconds ({} milliseconds)", diff, diff * 1000.0);

    retval
}

#[macro_export]
macro_rules! solutions {
    ($(sol $name:ident $content:block)+) => {{
        solutions! {
            inputs: ()

            $(
               sol $name $content
            )+
        }
    }};

    (inputs: ($($n:ident : $ty:ty = $def:expr),*) $(sol $name:ident $content:block)+) => {{
        let args: Vec<String> = ::std::env::args().collect();
        let mut opts = $crate::Options::new();

        $(
            opts.optopt("", stringify!($n), "", "VALUE");
        )*
        opts.optflag("h", "help", "Display this help text");

        let help = || {
            let brief = format!("Usage: {} [options] [versions] \nVersions: {:?}", &args[0], [$(stringify!($name)),+]);
            print!("{}", opts.usage(&brief));
        };

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(_) => { help(); return; }
        };

        if matches.opt_present("h") {
            help();
            return;
        }

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

        let valid_solutions = vec![$(stringify!($name)),*];
        let solutions: Vec<String> = if matches.free.len() > 0 {
            matches.free.iter()
            .filter(|&m| valid_solutions.contains(&m.as_ref()))
            .map(|ref a| a.clone().to_string())
            .collect()
        } else {
            valid_solutions.iter().map(|&a| a.to_string()).collect()
        };

        $(
            if solutions.contains(&stringify!($name).to_string()) {
                println!("Running: {}", stringify!($name));
                println!("Result: {:?}", $crate::time_execution($name));
                println!("");
            }
        )+
    }};
}

#[macro_export]
macro_rules! hash_map {
    ($($key:expr => $val:expr),*) => {{
        let mut ret = ::std::collections::HashMap::new();
        $(ret.insert($key, $val);)*
        ret
    }};

    ($old:ident, { $($key:expr => $val:expr),* } ) => {{
        $($old.insert($key, $value);)*
    }};
}

#[macro_export]
macro_rules! sparse_array {
    ($size:expr, $($key:expr => $val:expr),+) => {{
        let mut array = Vec::with_capacity($size);
        for _ in 0..$size {
            array.push(0);
        }
        $(array[$key] = $val;)+
        array
    }};
}
