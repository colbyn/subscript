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
    SN: PartialEq + Clone + Debug,
    SL: PartialEq + Clone + Debug,
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
            (STree::Leaf(old), ITree::Leaf(new)) => {
                (f.leafs)(parent, &old.data.borrow(), &new.data);
                if !old.unchanged(api, new) {
                    api.leaf_update(Update {
                        new: &new.data,
                        old: &mut *old.data.borrow_mut(),
                    });
                }
            }
            (STree::Node(old), ITree::Node(new)) => {
                if !old.unchanged(Intensity::Shallow, api, new) {
                    api.node_update(Update {
                        new: &new.data,
                        old: &mut *old.data.borrow_mut(),
                    });
                }
            	let ref children_parent = api.get_meta(Left(&old.data.borrow()));
                old.children.traverse_sync(api, children_parent, &new.children, f);
                (f.nodes)(parent, &old.data.borrow(), &new.data);
            }
            (old, new) => {
            	new.traverse_sync(parent, &ITreeSyncTraversal {
            		node: f.new_node,
            		leaf: f.new_leaf,
            	});
                let created = new.create_tree(api, parent);
                api.insert(InsertOp::Swap {
                    parent: parent.clone(),
                    old: old.get_meta(api),
                    new: created.get_meta(api),
                });
                *old = created;
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
impl<M, SN, SL, IN, IL> SChildren<M,SN,SL,IN,IL>
where
    M: PartialEq + Clone + Debug,
    SN: PartialEq + Clone + Debug,
    SL: PartialEq + Clone + Debug,
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
        // TMP - SANITY CHECK
        let mut available: HashSet<usize> = (0 .. self.0.borrow().len()).collect();
        let mut this: Vec<(usize, Rc<RefCell<STree<M,SN,SL,IN,IL>>>)> = self.0
            .borrow_mut()
            .drain(..)
            .enumerate()
            .map(|(ix, x)| (ix, x))
            .collect();
        let mut stage1 = new.0
            .iter()
            .map(|new: &ITree<IN, IL>| -> Stage1<M, SN, SL, IN, IL> {
                let mut get_unchanged = |this: &mut Vec<(usize, Rc<RefCell<STree<M,SN,SL,IN,IL>>>)>| {
                    let pos = this.iter().position(|x| x.1.borrow().unchanged(Intensity::Deep, api, &new))?;
                    let (ix, x) = this.remove(pos);
                    assert!(available.remove(&ix));
                    Some((pos, (ix, x)))
                };
                if let Some((poped_pos, (old_ix, old))) = get_unchanged(&mut this) {
                    Stage1::Unchanged(Unchanged {old, new})
                } else {
                    Stage1::Unset(new)
                }
            })
            .collect::<Vec<_>>();
        let mut stage2 = stage1
            .into_iter()
            .map(|stage1: Stage1<M, SN, SL, IN, IL>| -> Stage2<M, SN, SL, IN, IL> {
                let mut get_changed = |new: &ITree<IN, IL>, this: &mut Vec<(usize, Rc<RefCell<STree<M,SN,SL,IN,IL>>>)>| {
                    let pos = this.iter().position(|x| x.1.borrow().recyclable(Intensity::Deep, api, new))?;
                    let (ix, x) = this.remove(pos);
                    assert!(available.remove(&ix));
                    Some((pos, (ix, x)))
                };
                match stage1 {
                    Stage1::Unchanged(x) => {Stage2::Unchanged(x)}
                    Stage1::Unset(new) => {
                        // CHANGED
                        if let Some((poped_pos, (ols_pos, old))) = get_changed(new, &mut this) {
                            Stage2::Changed(Changed{old, new})
                        }
                        // NEW
                        else {
                            let created = new.create_tree(api, parent);
                            let created = Rc::new(RefCell::new(created));
                            Stage2::New(New{created, new})
                        }
                    }
                }
            })
            .collect::<Vec<_>>();
        assert!(stage2.len() == new.0.len());
        // REMOVE UNUSED
        for (old_ix, old) in this.drain(..) {
            api.remove(old.borrow().get_meta(api));
        }
        // UPSERT HELPERS
        let ref metas = stage2
            .iter()
            .map(|entry| -> M {
                match entry {
                    Stage2::Unchanged(Unchanged{old, new, ..}) => {
                        // console::log(format!(
                        //     "Children::Unchanged {:?} <---> {:?}",
                        //     old.borrow().to_either_inner_clone(),
                        //     new.to_either_inner(),
                        // ));
                        old.borrow().get_meta(api)
                    },
                    Stage2::Changed(Changed{old, new, ..}) => {
                        // console::log(format!(
                        //     "Children::Changed {:?} ---> {:?}",
                        //     old.borrow().to_either_inner_clone(),
                        //     new.to_either_inner(),
                        // ));
                        old.borrow().get_meta(api)
                    },
                    Stage2::New(New{created, new}) => {
                        // console::log(format!("Children::New {:?}", created.borrow().to_either_inner_clone()));
                        created.borrow().get_meta(api)
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
            // CHECK INSERT AFTER
            else if let Some(old) = metas.get(ix - 1) {
                InsertOp::InsertAfter{
                    new: vec![new],
                    old: old.clone(),
                }
            }
            // OTHERWISE INSERT BEFORE
            else if let Some(old) = metas.get(ix + 1) {
                InsertOp::InsertBefore{
                    new: vec![new],
                    old: old.clone(),
                }
            }
            else {panic!()}
        };
        // APPLY API TRAIT UPDATES & USER-CALLBACKS
        let mut stage3 = stage2
            .into_iter()
            .enumerate()
            .map(|(ix, entry)| -> Stage3<M, SN, SL, IN, IL> {
                match entry {
                    Stage2::Unchanged(Unchanged{old, new, ..}) => {
                        old.borrow_mut().traverse_sync(api, parent, new, f);
                        Stage3::PositionUnchanged {data: old}
                    },
                    Stage2::Changed(Changed{old, new, ..}) => {
                        old.borrow_mut().traverse_sync(api, parent, new, f);
                        Stage3::PositionUnchanged {data: old}
                    },
                    Stage2::New(New{created, new}) => {
                        let insert_op = get_insert_op(ix, created.borrow().get_meta(api));
                        new.traverse_sync(parent, &ITreeSyncTraversal {
                            node: f.new_node,
                            leaf: f.new_leaf,
                        });
                        Stage3::Upsert {insert_op, data: created}
                    },
                }
            })
            .collect::<Vec<_>>();
        // APPLY UPSERTS
        let mut results = stage3
            .into_iter()
            .map(|entry| -> Rc<RefCell<STree<M, SN, SL, IN, IL>>> {
                match entry {
                    Stage3::PositionUnchanged{data, ..} => {data}
                    Stage3::Upsert{data, insert_op} => {
                        api.insert(insert_op);
                        data
                    },
                }
            })
            .collect::<Vec<_>>();
        // SAVE & DONE
        self.0.borrow_mut().append(&mut results);
        // console::log("***");
        // console::log("-------------------------------------------------------------------------------");
    }
}

pub enum Stage1<'a, M, SN, SL, IN, IL> {
    Unchanged(Unchanged<'a, M, SN, SL, IN, IL>),
    Unset(&'a ITree<IN, IL>),
}
pub enum Stage2<'a, M, SN, SL, IN, IL> {
    Unchanged(Unchanged<'a, M, SN, SL, IN, IL>),
    Changed(Changed<'a, M, SN, SL, IN, IL>),
    New(New<'a, M, SN, SL, IN, IL>),
}
pub enum Stage3<M, SN, SL, IN, IL> {
    PositionUnchanged {
        data: Rc<RefCell<STree<M, SN, SL, IN, IL>>>,
    },
    Upsert {
        insert_op: InsertOp<M>,
        data: Rc<RefCell<STree<M, SN, SL, IN, IL>>>,
    },
}

#[derive(Debug)]
pub struct Unchanged<'a, M, SN, SL, IN, IL> {
    old: Rc<RefCell<STree<M, SN, SL, IN, IL>>>,
    new: &'a ITree<IN, IL>,
}
#[derive(Debug)]
pub struct Changed<'a, M, SN, SL, IN, IL> {
    old: Rc<RefCell<STree<M, SN, SL, IN, IL>>>,
    new: &'a ITree<IN, IL>,
}
#[derive(Debug)]
pub struct New<'a, M, SN, SL, IN, IL> {
    created: Rc<RefCell<STree<M, SN, SL, IN, IL>>>,
    new: &'a ITree<IN, IL>,
}

// #[derive(Debug)]
// pub enum ItemType {
//     Unchanged,
//     Changed,
//     New,
// }

// #[derive(Debug)]
// pub enum Item<X> {
//     Preexisting {
//         poped_position: usize,
//         adjusted_position: usize,
//         data: X,
//     },
//     New{
//         adjusted_position: usize,
//         data: X,
//     },
// }

// impl<X> Item<X> {
//     fn unpack(&self) -> &X {
//         match self {
//             Item::New{data, ..} => data,
//             Item::Preexisting{data, ..} => data,
//         }
//     }
// }


