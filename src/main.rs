#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

use std::env;

mod checker;
use checker::*;

fn init_checker_from_args() -> Checker {
    let files = env::args().skip(1).collect();
    let mut checker = Checker::new();
    checker.load_passwords(files);
    checker
}

#[get("/")]
fn health() -> &'static str {
    "OK"
}

#[post("/", data = "<input>")]
fn find(input: String, passwords: rocket::State<Checker>) -> &'static str {
    if passwords.contains(&input) {
        "Bad\n"
    } else {
        "Good\n"
    }
}

fn main() {
    rocket::ignite()
        .manage(init_checker_from_args())
        .mount("/checkpass", routes![health, find])
        .launch();
}
