#[macro_use] extern crate libeuler;

include!(concat!(env!("OUT_DIR"), "/run.rs"));

fn main() {
    run();
}
