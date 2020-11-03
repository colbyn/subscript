#![allow(unused)]

#[macro_use] extern crate html5ever;
#[macro_use] extern crate markup5ever;

pub mod data;
pub mod parser;
pub mod macors;
pub mod utils;


fn main() {
    data::run();
}
