#[macro_use] extern crate libeuler;
extern crate num;
use std::env;
use std::ffi::OsString;

include!(concat!(env!("OUT_DIR"), "/run.rs"));

fn main() {
    let mut args = env::args();
    args.next();

    match args.next().unwrap_or(OsString::from_str("run")).into_string() {
        Ok(v) => {
            match v.as_slice() {
                "run" => {
                    println!("running");
                    run(args);
                },
                "start" => {
                    println!("Starting a prompt :3");
                },
                _ => {
                    println!("Unknown argument");
                }
            }
        },
        Err(_) => {
            println!("Unable to parse command line arguments");
        }
    }
}
