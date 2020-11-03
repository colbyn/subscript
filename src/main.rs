#![allow(unused)]

#[macro_use] extern crate html5ever;
#[macro_use] extern crate markup5ever;

pub mod data;
pub mod parser;


fn main() {
    data::run();
}
