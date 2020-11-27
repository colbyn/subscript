#![allow(unused)]

#[macro_use] extern crate html5ever;
#[macro_use] extern crate markup5ever;
#[macro_use] extern crate lazy_static;

pub mod data;
pub mod parser;
pub mod macros;
pub mod utils;
pub mod cli;
pub mod server;
pub mod browser;


fn main() {
    cli::run();
}
