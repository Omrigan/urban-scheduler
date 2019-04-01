#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use crate::logic::Problem;
use crate::logic::MyPoint;
use geo::Point;
use geo::Coordinate;
use crate::logic::Event;

mod logic;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
//    let p = logic::get_sample_problem();
//    println!("{:?}", p);
//
//    let s = logic::solve_stupid(&p);

//    rocket::ignite().mount("/", routes![index]).launch();
}