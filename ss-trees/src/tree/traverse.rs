use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::*;
use std::cell::*;
use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either::{self, Left, Right};
use itertools::Itertools;
use ss_web_utils::js::console;

use crate::tree::*;

///////////////////////////////////////////////////////////////////////////////
// DATA
///////////////////////////////////////////////////////////////////////////////
pub type Parent<M> = M;
pub struct SyncTraversal<'a,M,SN,SL,IN,IL> {
    pub nodes: &'a Fn(Parent<&M>, &SN, &IN),
    pub leafs: &'a Fn(Parent<&M>, &SL, &IL),
    pub new_node: &'a Fn(Parent<&M>, &IN),
    pub new_leaf: &'a Fn(Parent<&M>, &IL),
}
struct ITreeSyncTraversal<'a,M,N,L> {
    node: &'a Fn(Parent<&M>, &N),
    leaf: &'a Fn(Parent<&M>, &L),
}


pub struct Traversal<'a,SN,SL> {
    pub node: &'a Fn(&SN),
    pub leaf: &'a Fn(&SL),
}



///////////////////////////////////////////////////////////////////////////////
// IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////
impl<N, L> ITree<N, L> {
    fn traverse_sync<'a,M>(&self, parent: &M, f: &ITreeSyncTraversal<'a,M,N,L>) {
        match self {
            ITree::Leaf(l) => {
                (f.leaf)(parent, &l.data);
            }
            ITree::Node(n) => {
                for c in n.children.0.iter() {
                    c.traverse_sync(parent, f);
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
    	parent: &M,
    	new: &ITree<IN, IL>,
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
                n1.children.traverse_sync(api, children_parent, &n2.children, f);
                (f.nodes)(parent, &n1.data.borrow(), &n2.data);
                if !n1.unchanged(api, n2) {
                    api.node_update(Update {
                        new: &n2.data,
                        old: &mut *n1.data.borrow_mut(),
                    });
                }
            }
            (old, new) => {
            	new.traverse_sync(parent, &ITreeSyncTraversal {
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
    pub fn traverse<'a>(&self, f: &Traversal<'a,SN,SL>) {
        match self {
            STree::Leaf(l1) => {
                (f.leaf)(&l1.data.borrow());
            }
            STree::Node(n1) => {
                for child in n1.children.0.borrow().iter() {
                    child.borrow().traverse(f);
                }
                (f.node)(&n1.data.borrow());
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
        parent: &M,
        new: &IChildren<IN, IL>,
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
            if let Some((poped_position, unchanged)) = get_unchanged(&mut this) {
                unchanged.borrow_mut().traverse_sync(api, parent, new, f);
                results.push(Item::Preexisting{
                    poped_position,
                    adjusted_position,
                    data: unchanged.clone(),
                });
                adjusted_position = adjusted_position + 1;
            } else if let Some((poped_position, changed)) = get_changed(&mut this) {
                changed.borrow_mut().traverse_sync(api, parent, new, f);
                results.push(Item::Preexisting{
                    poped_position,
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
        assert!(results.len() == new.0.len());
        for old in this.drain(..) {
            api.remove(old.borrow().get_meta(api));
        }
        let ref metas = results
            .iter()
            .map(|item| -> Item<M> {
                match item {
                    Item::Preexisting{poped_position, adjusted_position, data} => {
                        let poped_position = poped_position.clone();
                        let adjusted_position = adjusted_position.clone();
                        let data = data.borrow().get_meta(api);
                        Item::Preexisting{poped_position, adjusted_position, data}
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
            // NEW - FIRST CHILD - APPEND
            if (ix == 0) && (metas.len() == 1) {
                InsertOp::Append{
                    parent: parent.clone(),
                    new: vec![new],
                }
            }
            // INSERT AFTER?
            else if let Some(old) = metas.get(ix - 1) {
                InsertOp::InsertAfter{
                    new: vec![new],
                    old: old.unpack().clone(),
                }
            }
            // OTHERWISE INSERT BEFORE
            else if let Some(old) = metas.get(ix + 1) {
                InsertOp::InsertBefore{
                    new: vec![new],
                    old: old.unpack().clone(),
                }
            }
            else {panic!()}
        };
        // console::log("-------------------------------------------------------------------------------");
        // console::log("**");
        for (ix, entry) in results.into_iter().enumerate() {
            match entry {
                Item::Preexisting{data, poped_position, adjusted_position} => {
                    // TODO: ...
                    assert!(poped_position == 0);
                    // if poped_position != 0 {
                    //     console::log(format!(
                    //         "{:#?} {:?}",
                    //         (ix, (poped_position, adjusted_position)),
                    //         &data
                    //     ));
                    // }
                    this.push(data);
                }
                Item::New{data, ..} => {
                    let insert_op = get_insert_op(ix, data.borrow().get_meta(api));
                    api.insert(insert_op);
                    this.push(data);
                }
            }
        }
        // console::log("**");
        // console::log("-------------------------------------------------------------------------------");
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
        poped_position: usize,
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


