use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::*;
use std::cell::*;
use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either::{self, Left, Right};
use itertools::Itertools;
use ss_web_utils::js::console;

use crate::data::*;


// impl<M, SN, SL, IN, IL> STree<M, SN, SL, IN, IL>
// where
//     M: PartialEq + Clone + Debug,
//     SN: PartialEq + Debug,
//     SL: PartialEq + Debug,
//     IN: PartialEq + Debug,
//     IL: PartialEq + Debug
// {
//     pub fn traverse(&self, nf: &mut FnMut(&SN), lf: &mut FnMut(&SL)) {
//         match self {
//             STree::Leaf(leaf) => {
//                 lf(&leaf.data);
//             }
//             STree::Node(node) => {
//                 for mut child in node.children.data.iter() {
//                     child.traverse(nf, lf);
//                 }
//                 nf(&node.data);
//             }
//         }
//     }
//     pub fn traverse_pair(&self, api: &TreeApi<M, SN, SL, IN, IL>, new: &ITree<IN, IL>, f: &PairTraversal<&Fn(&SN, &IN), &Fn(&SL, &IL)>) {
//         match (self, new) {
//             (STree::Leaf(l1), ITree::Leaf(l2)) => {
//                 (f.leafs)(&l1.data, &l2.data);
//             }
//             (STree::Node(n1), ITree::Node(n2)) => {
//                 for new in n2.children.0.iter() {
//                     let unchanged = || get_item_by(&n1.children.data, |old| {
//                         old.unchanged(api, &new)
//                     });
//                     let changed = || get_item_by(&n1.children.data, |old| {
//                         old.recyclable(api, &new)
//                     });
//                     if let Some(unchanged) = unchanged() {
//                         unchanged.traverse_pair(api, new, f);
//                     } if let Some(changed) = changed() {
//                         changed.traverse_pair(api, new, f);
//                     }
//                 }
//                 (f.nodes)(&n1.data, &n2.data);
//             }
//             _ => {}
//         }
//     }
// }

// pub struct PairTraversal<NS, LS> {
//     pub nodes: NS,
//     pub leafs: LS,
// }


