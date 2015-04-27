#![feature(plugin)]
#![plugin(string_cache_plugin)]

extern crate html5ever;
extern crate hyper;
#[macro_use] extern crate string_cache;

use std::io::{self, Read, Write};
use std::default::Default;
use std::env;

use hyper::Client;

use html5ever::sink::common::{Text, Element};
use html5ever::sink::rcdom::{RcDom, Handle};
use html5ever::{parse, one_input};

fn getdom(input: String) -> RcDom {
    parse(one_input(input), Default::default())
}

fn to_string(handle: Handle, text: &mut String) {
    let node = handle.borrow();

    match node.node {
        Text(ref value) => {
            text.push_str(value);
        },
        _ => {},
    }

    for child in node.children.iter() {
        to_string(child.clone(), text);
    }
}

fn find(handle: Handle, class: String) -> Option<Handle> {
    let node = handle.borrow();

    match node.node {
        Element(_, ref attrs) => {
            for attr in attrs.iter() {
                match (attr.name.local.as_slice(), attr.value.to_string()) {
                ("class", ref c) if c == &class => return Some(handle.clone()),
                    _ => continue
                }
            }
        },
        _ => {}
    }

    for child in node.children.iter() {
        match find(child.clone(), class.clone()) {
            Some(node) => return Some(node),
            None => {}
        }
    }

    None
}

const BASE: &'static str = "https://projecteuler.net/problem=";
fn main() {
    let mut client = Client::new();
    let mut result = String::new();

    let problem_num = {
        let args = env::args();

        let mut prob = match args.last() {
            Some(v) => v.parse::<i64>().ok(),
            None => None
        };

        while prob.is_none() {
            print!("Which problem? ");
            io::stdout().flush().unwrap();

            let mut answer = String::new();
            io::stdin().read_line(&mut answer).unwrap();

            prob = answer.trim().parse::<i64>().ok();
        }

        prob.unwrap()
    };


    client.get(format!("{}{}", BASE, problem_num).as_ref())
        .send().unwrap()
        .read_to_string(&mut result).unwrap();

    let dom = getdom(result);

    let div = find(dom.document, "problem_content".to_string());

    let mut problem = String::new();

    match div {
        Some(d) => to_string(d, &mut problem),
        None => println!("class not found")
    }

    let max_line_length = 100;
    let mut first_line = true;
    for line in  problem.split("\n") {
        if line.trim().len() == 0 { continue }
        if !first_line { println!("///"); }

        print!("///");
        let mut len = 3;
        for word in line.split_whitespace() {
            if word.len() + len + 1 > max_line_length {
                println!("");
                print!("///");
                len = 3;
            }
            print!(" {}", word);
            len += word.len() + 1;
        }

        println!("");

        first_line = false;
    }
}
