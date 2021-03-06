// Copyright 2015 Wesley Wiser. 
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rustc_serialize;
extern crate toml;

use std::env::args;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use rustc_serialize::json::Json;
use toml::{Parser, Value};

fn main() {
    let toml_path = get_cargo_toml_path();
    let mut toml_file = 
        match File::open(toml_path) {
            Ok(f) => f,
            Err(_) => panic!("Couldn't open Cargo.toml")
        };

    let mut contents = String::new();
    toml_file.read_to_string(&mut contents).unwrap();

    let mut parser = Parser::new(&*contents);
    let toml_table = Value::Table(parser.parse().unwrap());

    let toml_value_path = get_requested_path();
    let value = toml_table.lookup(&*toml_value_path).expect("Couldn't find the requested value");

    println!("{}", value);
}

fn get_requested_path() -> String {
    let arguments = args();

    if arguments.len() != 3 {
        panic!("Usage cargo config path.to.value");
    }

    arguments.last().unwrap()
}

fn get_cargo_toml_path() -> String {
    let output = cargo("locate-project");
    let json = 
        match Json::from_str(&*output) {
            Ok(j) => j,
            Err(_) => panic!("Couldn't parse the output of `cargo locate-project`")
        };
    json["root"].as_string().unwrap().to_string()
}

fn cargo(command: &str) -> String {
    let output = Command::new("cargo")
                         .arg(command)
                         .output()
                         .unwrap();
    String::from_utf8_lossy(&output.stdout).into_owned()
}

