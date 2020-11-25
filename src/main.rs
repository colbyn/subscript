#![allow(unused)]

#[macro_use] extern crate html5ever;
#[macro_use] extern crate markup5ever;
#[macro_use] extern crate lazy_static;

pub mod data;
pub mod parser;
pub mod macros;
pub mod utils;
pub mod cli;

fn main() {
    cli::run();
    // utils::parse_css();
    // parser::css_root::process_css("div {color: red}");
    // let source = include_str!("../test.css");
}
