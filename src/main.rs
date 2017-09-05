#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

use std::io::prelude::*;
use std::env;
use std::collections::BTreeSet;
use std::io::Error as IOError;
use std::io::BufReader;
use std::fs::File;

fn load_passwords() -> BTreeSet<String> {
    let mut passwords = BTreeSet::new();
    for filename in env::args().skip(1) {
        match injest_password_file(&mut passwords, &filename) {
            Ok(_) => println!("Completed file {}... passwords so far: {}", filename, passwords.len()),
            Err(error) => println!("Skipping file {}: read canceled because {}", filename, error),
        }
    }
    passwords
}

fn injest_password_file(passwords: &mut BTreeSet<String>, filename: &str) -> Result<(), IOError> {
    let file = File::open(filename)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if line.len() >= 8 {
            passwords.insert(line);
        }
    }
    Ok(())
}

#[get("/")]
fn health() -> &'static str {
    "OK"
}

#[post("/", data = "<input>")]
fn find(input: String, passwords: rocket::State<BTreeSet<String>>) -> &'static str {
    if passwords.contains(&input) {
        "Bad\n"
    } else {
        "Good\n"
    }
}

fn main() {
    let passwords = load_passwords();
    rocket::ignite()
        .manage(passwords)
        .mount("/checkpass", routes![health, find])
        .launch();
}
