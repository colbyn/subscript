#![allow(unused)]

#[macro_use] extern crate html5ever;
#[macro_use] extern crate markup5ever;
#[macro_use] extern crate lazy_static;

pub mod data;
pub mod parser;
pub mod macors;
pub mod utils;
pub mod cli;

fn main() {
    data::run();
}
