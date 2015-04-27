#![feature(plugin, path_ext)]
#![plugin(string_cache_plugin)]

extern crate html5ever;
extern crate hyper;
#[macro_use] extern crate string_cache;

use std::io::{self, Read, Write};
use std::default::Default;
use std::env;
use std::fs::{self, PathExt, File};
use std::path::Path;

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

fn get_problem_num() -> i64 {
    let args = env::args();

    let mut prob = match args.last() {
        Some(v) => v.parse().ok(),
        None => None
    };

    while prob.is_none() {
        print!("Which problem? ");
        io::stdout().flush().unwrap();

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();

        prob = answer.trim().parse().ok();
    }

    prob.unwrap()
}

const BASE: &'static str = "https://projecteuler.net/problem=";
fn main() {
    let mut client = Client::new();
    let mut result = String::new();

    let problem = get_problem_num();

    let sol_dir = format!("sol_{:04}", problem);
    let dir = Path::new(&sol_dir);
    let file = dir.join("main.rs");

    if !dir.exists() {
        fs::create_dir(dir).ok().expect("Unable to create directory!");
    }

    if file.exists() {
        print!("File already exists. Overwrite? (y/N) ");
        io::stdout().flush().unwrap();

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();

        match answer.trim().as_ref() {
            "y" | "Y" | "yes" | "Yes" => {},
            _ => return
        }
    }

    client.get(format!("{}{}", BASE, problem).as_ref())
        .send().unwrap()
        .read_to_string(&mut result).unwrap();

    let dom = getdom(result);

    let div = find(dom.document, "problem_content".to_string());

    let mut problem_text = String::new();

    match div {
        Some(d) => to_string(d, &mut problem_text),
        None => {
            println!("No problem found!");
            return;
        }
    }

    let max_line_length = 100;
    let mut first_line = true;

    let errmsg = format!("Couldn't open `{}`", file.display());
    let mut out = File::create(file).ok().expect(&errmsg);

    writeln!(out, "#[macro_use] extern crate libeuler;").unwrap();
    writeln!(out, "").unwrap();
    for line in  problem_text.split("\n") {
        if line.trim().len() == 0 { continue }
        if !first_line { writeln!(out, "///").unwrap(); }

        write!(out, "///").unwrap();
        let mut len = 3;
        for word in line.split_whitespace() {
            if word.len() + len + 1 > max_line_length {
                writeln!(out, "").unwrap();
                write!(out, "///").unwrap();
                len = 3;
            }
            write!(out, " {}", word).unwrap();
            len += word.len() + 1;
        }

        writeln!(out, "").unwrap();

        first_line = false;
    }

    writeln!(out, "fn main() {{").unwrap();
    writeln!(out, "    solutions! {{").unwrap();
    writeln!(out, "        sol naive {{").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}").unwrap();
}
