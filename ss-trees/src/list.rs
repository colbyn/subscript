use std::fmt::Debug;
use std::rc::*;
use std::cell::*;
use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either::{self, Left, Right};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Item<T> {
	Unchanged(T),
	New(T),
}

impl<T> Item<T> {
	pub fn into_inner(self) -> T {
		match self {
			Item::Unchanged(x) => x,
			Item::New(x) => x,
		}
	}
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ItemType {
	Unchanged,
	New,
}

impl<T> Item<T> {
	pub fn is_unchanged(&self) -> bool {
		match self {
			Item::Unchanged(_) => true,
			_ => false
		}
	}
	pub fn get_type(&self) -> ItemType {
		match self {
			Item::Unchanged(_) => ItemType::Unchanged,
			Item::New(_) => ItemType::New,
		}
	}
}

#[derive(Debug, Clone)]
pub enum InsertOp<T> {
	InsertBefore(T),
	InsertAfter(T),
}

#[derive(Debug, Clone)]
pub enum Process<T> {
	Unchanged {
		items: Vec<Item<T>>,
	},
	Update {
		insert_op: InsertOp<T>,
		items: Vec<Item<T>>,
	}
}

#[derive(Debug, Clone)]
pub struct List<T> {
	data: Vec<T>,
}

pub fn remove_item<T: PartialEq>(xs: &mut Vec<T>, item: &T) -> Option<T> {
    let pos = xs.iter().position(|x| *x == *item)?;
    Some(xs.remove(pos))
}

pub fn remove_item_by<T: PartialEq>(xs: &mut Vec<T>, f: impl Fn(&T)->bool) -> Option<T> {
    let pos = xs.iter().position(|x| f(x))?;
    Some(xs.remove(pos))
}

impl<T: PartialEq + Clone + Debug> List<T> {
	pub fn from(xs: Vec<T>) -> Self {
		List {data: xs}
	}
	pub fn sync(&mut self, xs: Vec<T>) {
		let mut xs = xs
			.into_iter()
			.map(|x| match remove_item_by(&mut self.data, |y| y == &x) {
				None => Item::New(x),
				Some(a) => Item::Unchanged(a),
			})
			.collect_vec();
		let xs = xs
			.into_iter()
			.group_by(|x| x.get_type())
			.into_iter()
			.map(|(key, group)| (key, group.collect_vec()))
			.collect_vec();
		let ys = xs.clone();
		let xs = xs
			.into_iter()
			.enumerate()
			.map(|(ix, (key, items))| {
				match key {
					ItemType::Unchanged => {
						Process::Unchanged {
							items: items,
						}
					}
					ItemType::New => {
						Process::Update {
							insert_op: {
								if ix <= 1 {
									match ys.get(ix + 1) {
										Some((_, after)) => {
											match after.first() {
												Some(x) => InsertOp::InsertBefore(x.clone().into_inner()),
												None => panic!()
											}
										}
										None => panic!()
									}
								} else {
									match ys.get(ix - 1) {
										Some((_, before)) => {
											match before.last() {
												Some(x) => InsertOp::InsertAfter(x.clone().into_inner()),
												None => panic!()
											}
										}
										None => match ys.get(ix + 1) {
											Some((_, after)) => {
												match after.first() {
													Some(x) => InsertOp::InsertBefore(x.clone().into_inner()),
													None => panic!()
												}
											}
											None => panic!()
										}
									}
								}
							},
							items: items,
						}
					}
				}
			})
			.collect_vec();
		// REMOVE UNUSED
		self.data.clear();
		// SAVE CHANGES
		let xs = xs
			.into_iter()
			.map(|(process)| {
				let items = match process {
					Process::Unchanged{items, ..} => items,
					Process::Update{items, ..} => items,
				};
				let mut items = items
					.into_iter()
					.map(|x| x.into_inner())
					.collect_vec();
				self.data.append(&mut items);
			})
			.collect_vec();
		println!("{:#?}", self.data);
	}
}





