#[macro_use] extern crate libeuler;
use std::env;

include!(concat!(env!("OUT_DIR"), "/run.rs"));

fn main() {
    let mut args = env::args();
    args.next();
    run(args);
}
