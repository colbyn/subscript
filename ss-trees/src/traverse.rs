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

///////////////////////////////////////////////////////////////////////////////
// DATA
///////////////////////////////////////////////////////////////////////////////
pub struct SyncTraversal<'a,M,SN,SL,IN,IL> {
    pub nodes: &'a Fn(Parent<&M>, &SN, &IN),
    pub leafs: &'a Fn(Parent<&M>, &SL, &IL),
    pub new_node: &'a Fn(Parent<&M>, &IN),
    pub new_leaf: &'a Fn(Parent<&M>, &IL),
}

pub struct ItreeTraversal<'a,M,N,L> {
    pub node: &'a Fn(Parent<&M>, &N),
    pub leaf: &'a Fn(Parent<&M>, &L),
}

pub type Parent<M> = M;



///////////////////////////////////////////////////////////////////////////////
// IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////
impl<N, L> ITree<N, L> {
    pub fn traverse<'a,M>(&self, parent: &M, f: &ItreeTraversal<'a,M,N,L>) {
        match self {
            ITree::Leaf(l) => {
                (f.leaf)(parent, &l.data);
            }
            ITree::Node(n) => {
                for c in n.children.0.iter() {
                    c.traverse(parent, f);
                }
                (f.node)(parent, &n.data);
            }
        }
    }
}

impl<M, SN, SL, IN, IL> STree<M,SN,SL,IN,IL>
where
    M: PartialEq + Clone + Debug,
    SN: PartialEq + Debug,
    SL: PartialEq + Debug,
    IN: PartialEq + Debug,
    IL: PartialEq + Debug
{
    pub fn traverse_sync<'a>(
    	&mut self,
    	api: &TreeApi<M, SN, SL, IN, IL>,
    	new: &ITree<IN, IL>,
    	parent: &M,
    	f: &SyncTraversal<'a,M,SN,SL,IN,IL>
    ) {
        match (self, new) {
            (STree::Leaf(l1), ITree::Leaf(l2)) => {
                (f.leafs)(parent, &l1.data.borrow(), &l2.data);
                if !l1.unchanged(api, l2) {
                    api.leaf_update(Update {
                        new: &l2.data,
                        old: &mut *l1.data.borrow_mut(),
                    });
                }
            }
            (STree::Node(n1), ITree::Node(n2)) => {
            	let ref children_parent = api.get_meta(Left(&n1.data.borrow()));
                n1.children.traverse_sync(api, &n2.children, children_parent, f);
                (f.nodes)(parent, &n1.data.borrow(), &n2.data);
                if !n1.unchanged(api, n2) {
                    api.node_update(Update {
                        new: &n2.data,
                        old: &mut *n1.data.borrow_mut(),
                    });
                }
            }
            (old, new) => {
            	new.traverse(parent, &ItreeTraversal {
            		node: f.new_node,
            		leaf: f.new_leaf,
            	});
                let result = new.create_tree(api, parent);
                api.insert(InsertOp::Swap {
                    parent: parent.clone(),
                    current: result.get_meta(api),
                    target: old.get_meta(api),
                });
                *old = result;
            }
        }
    }
}
impl<M, SN, SL, IN, IL> SChildren<M, SN, SL, IN, IL>
where
    M: PartialEq + Clone + Debug,
    SN: PartialEq + Debug,
    SL: PartialEq + Debug,
    IN: PartialEq + Debug,
    IL: PartialEq + Debug
{
    pub fn traverse_sync<'a>(
        &self,
        api: &TreeApi<M, SN, SL, IN, IL>,
        new: &IChildren<IN, IL>,
        parent: &M,
        f: &SyncTraversal<'a,M,SN,SL,IN,IL>
    ) {
        let mut results: Vec<Item<Rc<RefCell<STree<M, SN, SL, IN, IL>>>>> = Vec::new();
        let mut this: RefMut<Vec<Rc<RefCell<STree<M, SN, SL, IN, IL>>>>> = self.0.borrow_mut();
        let mut adjusted_position: usize = 0;
        for new in new.0.iter() {
            let mut get_unchanged = |this: &mut RefMut<Vec<Rc<RefCell<STree<M, SN, SL, IN, IL>>>>>| {
                let pos = this.iter().position(|x| x.borrow().unchanged(api, &new))?;
                Some((pos, this.remove(pos)))
            };
            let mut get_changed = |this: &mut RefMut<Vec<Rc<RefCell<STree<M, SN, SL, IN, IL>>>>>| {
                let pos = this.iter().position(|x| x.borrow().recyclable(api, &new))?;
                Some((pos, this.remove(pos)))
            };
            if let Some((old_position, unchanged)) = get_unchanged(&mut this) {
                unchanged.borrow_mut().traverse_sync(api, new, parent, f);
                results.push(Item::Preexisting{
                    old_position,
                    adjusted_position,
                    data: unchanged.clone(),
                });
                adjusted_position = adjusted_position + 1;
            } else if let Some((old_position, changed)) = get_changed(&mut this) {
                changed.borrow_mut().traverse_sync(api, new, parent, f);
                results.push(Item::Preexisting{
                    old_position,
                    adjusted_position,
                    data: changed.clone(),
                });
                adjusted_position = adjusted_position + 1;
            } else {
                let created = new.create_tree(api, parent);
                results.push(Item::New{
                    adjusted_position,
                    data: Rc::new(RefCell::new(created)),
                });
            }
        }
        for old in this.drain(..) {
            api.remove(old.borrow().get_meta(api));
        }
        let ref metas = results
            .iter()
            .map(|item| -> Item<M> {
                match item {
                    Item::Preexisting{old_position, adjusted_position, data} => {
                        let old_position = old_position.clone();
                        let adjusted_position = adjusted_position.clone();
                        let data = data.borrow().get_meta(api);
                        Item::Preexisting{old_position, adjusted_position, data}
                    }
                    Item::New{adjusted_position, data} => {
                        let adjusted_position = adjusted_position.clone();
                        let data = data.borrow().get_meta(api);
                        Item::New{adjusted_position, data}
                    }
                }
            })
            .collect::<Vec<_>>();
        let get_insert_op = |ix: usize, new: M| -> InsertOp<M> {
            if ix == 0 {
                match metas.get(ix + 1) {
                    Some(old) => {
                        InsertOp::InsertBefore{
                            new: vec![new],
                            old: old.unpack().clone(),
                        }
                    }
                    None => {panic!()}
                }
            } else {
                match metas.get(ix - 1) {
                    Some(old) => {
                        InsertOp::InsertAfter{
                            new: vec![new],
                            old: old.unpack().clone(),
                        }
                    }
                    None => match metas.get(ix + 1) {
                        Some(old) => {
                            InsertOp::InsertBefore{
                                new: vec![new],
                                old: old.unpack().clone(),
                            }
                        }
                        None => panic!()
                    }
                }
            }
        };
        for (ix, entry) in results.into_iter().enumerate() {
            match entry {
                Item::Preexisting{data, old_position, adjusted_position} => {
                    assert!(old_position == adjusted_position);
                    this.push(data);
                }
                Item::New{data, ..} => {
                    let insert_op = get_insert_op(ix, data.borrow().get_meta(api));
                    api.insert(insert_op);
                    this.push(data);
                }
            }
        }
    }
}


#[derive(Debug)]
pub enum ItemType {
    Unchanged,
    Changed,
    New,
}

#[derive(Debug)]
pub enum Item<X> {
    Preexisting {
        old_position: usize,
        adjusted_position: usize,
        data: X,
    },
    New{
        adjusted_position: usize,
        data: X,
    },
}

impl<X> Item<X> {
    fn unpack(&self) -> &X {
        match self {
            Item::New{data, ..} => data,
            Item::Preexisting{data, ..} => data,
        }
    }
}


