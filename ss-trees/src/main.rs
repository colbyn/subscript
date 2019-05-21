#![allow(dead_code, unused, unused_variables)]
pub mod tree;
pub mod map;
pub mod list;

pub fn main() {
	use list::*;
	let xs = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];
	let ys = vec!['0', '1', 'c', 'd', 'e', 'f', '1', '2', 'i', 'j', 'x', 'y', 'z'];
	let mut current = List::from(xs);
	current.sync(ys);
}