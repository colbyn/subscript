use std::marker::PhantomData;
use std::rc::*;
use std::cell::*;
use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either::{self, Left, Right};



///////////////////////////////////////////////////////////////////////////////
// CLIENT API
///////////////////////////////////////////////////////////////////////////////

pub enum ChildInsert<M> {
    Replace {
        parent: M,
        current_occupant: M,
    },
    Swap {
        current_occupant: M,
    },
    Append {
        parent: M,
    },
    InsertBefore {
        parent: M,
    },
}

pub enum InternalChildUpdate<M, S, I> {
    Create(ChildCreate<M, I>),
    Update(ChildUpdate<M, S, I>),
}

pub struct ChildUpdate<M, S, I> {
    insert_op: Option<ChildInsert<M>>,
    old: S,
    new: I,
}

pub struct ChildCreate<M, I> {
    insert_op: Option<ChildInsert<M>>,
    new: I,
}

pub struct ChildRemove<M, S> {
    parent: M,
    value: S,
}

pub trait TreeApi<M, SN, SL, IN, IL> {
    fn node_unchanged(&self, new: &IN, old: &SN) -> bool;
    fn node_recyclable(&self, new: &IN, old: &SN) -> bool;
    fn node_update(&self, op: ChildUpdate<M, &mut SN, IN>);
    fn node_create(&self, op: ChildCreate<M, IN>) -> SN;
    fn node_remove(&self, value: ChildRemove<M, SN>);

    fn leaf_unchanged(&self, new: &IL, old: &SL) -> bool;
    fn leaf_recyclable(&self, new: &IL, old: &SL) -> bool;
    fn leaf_update(&self, op: ChildUpdate<M, &mut SL, IL>);
    fn leaf_create(&self, op: ChildCreate<M, IL>) -> SL;
    fn leaf_remove(&self, value: ChildRemove<M, SL>);

    // fn remount(&self, op: Remount<M, Either<&SN, &SL>>);
    fn get_meta(&self, value: Either<&SN, &SL>) -> M;
    fn unmount(&self, value: M);
}

///////////////////////////////////////////////////////////////////////////////
// INTERNAL UTILS
///////////////////////////////////////////////////////////////////////////////

fn get_meta_option<'a, M, SN, SL, IN, IL>(
    api: &TreeApi<M, SN, SL, IN, IL>,
    value: Option<&SN>,
) -> Option<M>
where
    M: Clone,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq
{
    match value {
        None => None,
        Some(x) => Some(api.get_meta(Left(x)))
    }
}




///////////////////////////////////////////////////////////////////////////////
// INSERTION TREE
///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq)]
pub enum ITree<N, L> {
    Leaf(ILeaf<L>),
    Node(INode<N, L>),
}

#[derive(PartialEq)]
pub struct ILeaf<L> {
    data: L
}

#[derive(PartialEq)]
pub struct INode<N, L> {
    data: N,
    children: IChildren<N, L>,
}

#[derive(PartialEq)]
pub struct IChildren<N, L>(pub Vec<ITree<N, L>>);

///////////////////////////////////////////////////////////////////////////////
// INSERTION TREE HELPERS
///////////////////////////////////////////////////////////////////////////////

// impl<N, L> ITree<N, L> {
//     pub fn unpack(self) -> (Either<N, L>, Option<IChildren<N, L>>) {
//         match self {
//             ITree::Leaf(x) => (Right(x.0), None),
//             ITree::Node(x) => (Left(x.data), Some(x.children)),
//         }
//     }
//     pub fn to_either_inner_own(self) -> Either<(N, IChildren<N, L>), L> {
//         match self {
//             ITree::Leaf(x) => Right(x.0),
//             ITree::Node(x) => Left((x.data, x.children)),
//         }
//     }
//     pub fn to_either_inner(&self) -> Either<&N, &L> {
//         match self {
//             ITree::Leaf(x) => Right(&x.0),
//             ITree::Node(x) => Left(&x.data),
//         }
//     }
// }

///////////////////////////////////////////////////////////////////////////////
// SYNC TREE
///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq)]
pub enum STree<M, SN, SL, IN, IL>{
    Leaf(SLeaf<M, SN, SL, IN, IL>),
    Node(SNode<M, SN, SL, IN, IL>),
}

#[derive(PartialEq)]
pub struct SLeaf<M, SN, SL, IN, IL> {
    mark: PhantomData<(M, SN, SL, IN, IL)>,
    data: SL,
}

#[derive(PartialEq)]
pub struct SNode<M, SN, SL, IN, IL> {
    mark: PhantomData<(M, SN, SL, IN, IL)>,
    data: SN,
    children: SChildren<M, SN, SL, IN, IL>,
}

#[derive(PartialEq)]
pub struct SChildren<M, SN, SL, IN, IL> {
    data: Vec<STree<M, SN, SL, IN, IL>>,
}


///////////////////////////////////////////////////////////////////////////////
// SYNC TREE HELPERS 
///////////////////////////////////////////////////////////////////////////////

impl<M, SN, SL, IN, IL> STree<M, SN, SL, IN, IL> {
    pub fn get_meta(&self, api: &TreeApi<M, SN, SL, IN, IL>) -> M {
        match self {
            STree::Leaf(x) => api.get_meta(Right(&x.data)),
            STree::Node(x) => api.get_meta(Left(&x.data)),
        }
    }
    pub fn to_either(&self) -> Either<&SNode<M, SN, SL, IN, IL>, &SLeaf<M, SN, SL, IN, IL>> {
        match self {
            STree::Leaf(x) => Right(x),
            STree::Node(x) => Left(x),
        }
    }
    pub fn to_either_mut(&mut self) -> Either<&mut SNode<M, SN, SL, IN, IL>, &mut SLeaf<M, SN, SL, IN, IL>> {
        match self {
            STree::Leaf(x) => Right(x),
            STree::Node(x) => Left(x),
        }
    }
    pub fn to_either_inner(&self) -> Either<&SN, &SL> {
        match self {
            STree::Leaf(x) => Right(&x.data),
            STree::Node(x) => Left(&x.data),
        }
    }
    pub fn to_either_inner_mut(&mut self) -> Either<&mut SN, &mut SL> {
        match self {
            STree::Leaf(x) => Right(&mut x.data),
            STree::Node(x) => Left(&mut x.data),
        }
    }
    pub fn unpack_leaf(&self) -> Option<(&SLeaf<M, SN, SL, IN, IL>)> {
        match self {
            STree::Leaf(x) => Some(x),
            STree::Node(_) => None,
        }
    }
    pub fn unpack_node(&self) -> Option<(&SNode<M, SN, SL, IN, IL>)> {
        match self {
            STree::Node(x) => Some(x),
            STree::Leaf(_) => None,
        }
    }
    pub fn unpack_leaf_mut(&mut self) -> Option<(&mut SLeaf<M, SN, SL, IN, IL>)> {
        match self {
            STree::Leaf(x) => Some(x),
            STree::Node(_) => None,
        }
    }
    pub fn unpack_node_mut(&mut self) -> Option<(&mut SNode<M, SN, SL, IN, IL>)> {
        match self {
            STree::Node(x) => Some(x),
            STree::Leaf(_) => None,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// INIT IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

impl<M, SN, SL, IN, IL> STree<M, SN, SL, IN, IL>
where
    M: Clone,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn from(api: &TreeApi<M, SN, SL, IN, IL>, parent: Option<&SN>, new: ITree<IN, IL>) -> Self {
        let insert_op = match parent {
            None => None,
            Some(parent) => Some(ChildInsert::Append {
                parent: api.get_meta(Left(parent)),
            }),
        };
        match new {
            ITree::Node(node) => {
                let INode{data, children: new_children} = node;
                let op = ChildCreate {insert_op, new: data};
                let data = api.node_create(op);
                let children = new_children.0
                    .into_iter()
                    .map(|child| STree::from(api, Some(&data), child))
                    .collect::<Vec<STree<M, SN, SL, IN, IL>>>();
                let children = SChildren {
                    data: children,
                };
                let mark = PhantomData;
                STree::Node(SNode {mark, data, children})
            }
            ITree::Leaf(leaf) => {
                let op = ChildCreate {insert_op, new: leaf.data};
                let data = api.leaf_create(op);
                let mark = PhantomData;
                STree::Leaf(SLeaf {mark, data})
            }
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// SYNC IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

impl<M, SN, SL, IN, IL> STree<M, SN, SL, IN, IL>
where
    M: Clone,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &ITree<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn recyclable(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &ITree<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn sync(&mut self, api: &TreeApi<M, SN, SL, IN, IL>, parent: Option<&SN>, new: ITree<IN, IL>) {
        match new {
            ITree::Leaf(new) => {
                match self {
                    old @ STree::Node(_) => {
                        let insert_op = match parent {
                            None => Some(ChildInsert::Swap{
                                current_occupant: old.get_meta(api),
                            }),
                            Some(parent) => Some(ChildInsert::Replace {
                                parent: api.get_meta(Left(parent)),
                                current_occupant: old.get_meta(api), 
                            })
                        };
                        let op = ChildCreate {
                            insert_op: insert_op,
                            new: new.data,
                        };
                        let data = api.leaf_create(op);
                        let result: Self = STree::Leaf(SLeaf{
                            data,
                            mark: PhantomData,
                        });
                        *old = result;
                    }
                    STree::Leaf(old) => {
                        let op = ChildUpdate {
                            insert_op: None,
                            old: &mut old.data,
                            new: new.data,
                        };
                        api.leaf_update(op);
                    }
                }
            }
            ITree::Node(new) => {
                let INode{data: new_data, children: new_children} = new;
                match self {
                    STree::Node(old) => {
                        let SNode{data: old_data, children: old_children, ..} = old;
                        let mut old_data = old_data;
                        let op = ChildUpdate {
                            insert_op: None,
                            old: old_data,
                            new: new_data,
                        };
                        api.node_update(op);
                        // let ref data = &old.data;
                    }
                    old @ STree::Leaf(_) => {
                        let insert_op = match parent {
                            None => Some(ChildInsert::Swap{
                                current_occupant: old.get_meta(api),
                            }),
                            Some(parent) => Some(ChildInsert::Replace {
                                parent: api.get_meta(Left(parent)),
                                current_occupant: old.get_meta(api), 
                            })
                        };
                        let op = ChildCreate {
                            insert_op: insert_op,
                            new: new_data,
                        };
                        let data = api.node_create(op);
                        let children = new_children.0
                            .into_iter()
                            .map(|child| STree::from(api, Some(&data), child))
                            .collect::<Vec<STree<M, SN, SL, IN, IL>>>();
                        let children = SChildren {
                            data: children,
                        };
                        let result: Self = STree::Node(SNode{
                            data,
                            mark: PhantomData,
                            children: children,
                        });
                        *old = result;
                    }
                }
            }
        }
    }
}


impl<M, SN, SL, IN, IL> SLeaf<M, SN, SL, IN, IL>
where
    M: Clone,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &ILeaf<IL>) -> bool {
        unimplemented!()
    }
    pub fn recyclable(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &ILeaf<IL>) -> bool {
        unimplemented!()
    }
    pub fn sync(&mut self, api: &TreeApi<M, SN, SL, IN, IL>, parent: Option<&SN>, new: ILeaf<IL>) {
        
    }
}

impl<M, SN, SL, IN, IL> SNode<M, SN, SL, IN, IL>
where
    M: Clone,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &INode<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn recyclable(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &INode<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn sync(&mut self, api: &TreeApi<M, SN, SL, IN, IL>, parent: Option<&SN>, new: INode<IN, IL>) {

    }
}


///////////////////////////////////////////////////////////////////////////////
// SYNC IMPLEMENTATION - CHILDREN
///////////////////////////////////////////////////////////////////////////////

enum EntryStatus<New, Old>{
    Unchanged {
        new_ix: usize,
        old_ix: usize,
        new: New,
        old: Old,
    },
    Changed {
        new_ix: usize,
        old_ix: usize,
        new: New,
        old: Old,
    },
    Swap {
        new_ix: usize,
        old_ix: usize,
        new: New,
        old: Old,
    },
    New {
        new_ix: usize,
        new: New,
    },
}


impl<M, SN, SL, IN, IL> SChildren<M, SN, SL, IN, IL>
where
    M: Clone,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &IChildren<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn recyclable(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &IChildren<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn sync(self, api: &TreeApi<M, SN, SL, IN, IL>, parent: &SN, new: IChildren<IN, IL>) -> Self {
        // HELPERS
        fn get_matching_item<M, SN, SL, IN, IL>(
            free_old_ixs: &mut HashSet<usize>,
            old: &Vec<(usize, Rc<STree<M, SN, SL, IN, IL>>)>,
            new: &ITree<IN, IL>,
            api: &TreeApi<M, SN, SL, IN, IL>,
        ) -> Option<(usize, Rc<STree<M, SN, SL, IN, IL>>)>
        where
            M: Clone,
            SN: PartialEq,
            SL: PartialEq,
            IN: PartialEq,
            IL: PartialEq,
        {
            use ITree::*;
            let mut return_ix = None;
            for (entry_ix, entry) in old.into_iter() {
                if return_ix.is_none() && free_old_ixs.contains(&entry_ix) && entry.unchanged(api, new) {
                    return_ix = Some(entry_ix.clone());
                    free_old_ixs.remove(&entry_ix);
                }
            }
            match return_ix {
                None => None,
                Some(return_ix) => {
                    assert!(old.len() > return_ix);
                    match old.get(return_ix) {
                        Some((ix, x)) => Some((ix.clone(), x.clone())),
                        None => panic!()
                    }
                }
            }
        }
        fn get_similar_tree<M, SN, SL, IN, IL>(
            free_old_ixs: &mut HashSet<usize>,
            old: &Vec<(usize, Rc<STree<M, SN, SL, IN, IL>>)>,
            new: &ITree<IN, IL>,
            api: &TreeApi<M, SN, SL, IN, IL>,
        ) -> Option<(usize, Rc<STree<M, SN, SL, IN, IL>>)>
        where
            M: Clone,
            SN: PartialEq,
            SL: PartialEq,
            IN: PartialEq,
            IL: PartialEq,
        {
            use ITree::*;
            let mut return_ix = None;
            for (entry_ix, entry) in old.into_iter() {
                if return_ix.is_none() && free_old_ixs.contains(&entry_ix) && entry.recyclable(api, new) {
                    return_ix = Some(entry_ix.clone());
                    free_old_ixs.remove(&entry_ix);
                }
            }
            match return_ix {
                None => None,
                Some(return_ix) => {
                    assert!(old.len() > return_ix);
                    match old.get(return_ix) {
                        Some((ix, x)) => Some((ix.clone(), x.clone())),
                        None => panic!()
                    }
                }
            }
        }
        fn apply_internal_update<M, SN, SL, IN, IL>(
            entry: InternalChildUpdate<M, Rc<STree<M, SN, SL, IN, IL>>, ITree<IN, IL>>,
            api: &TreeApi<M, SN, SL, IN, IL>,
        ) -> STree<M, SN, SL, IN, IL>
        where
            M: Clone,
            SN: PartialEq,
            SL: PartialEq,
            IN: PartialEq,
            IL: PartialEq,
        {
            match entry {
                InternalChildUpdate::Update(ChildUpdate{insert_op, old, new}) => {
                    let mut old = match Rc::try_unwrap(old) {
                        Ok(x) => x,
                        Err(_) => panic!(),
                    };
                    match new {
                        ITree::Node(node) => {
                            let old = old
                                .unpack_node_mut()
                                .expect("should be a node");
                            let INode{data: new, children} = node;
                            let op = ChildUpdate {insert_op, new, old: &mut old.data};
                            api.node_update(op);
                        }
                        ITree::Leaf(leaf) => {
                            let old = old
                                .unpack_leaf_mut()
                                .expect("should be a leaf");
                            let op = ChildUpdate {insert_op, new: leaf.data, old: &mut old.data};
                            api.leaf_update(op);
                        }
                    }
                    old
                }
                InternalChildUpdate::Create(ChildCreate {insert_op, new}) => {
                    match new {
                        ITree::Node(node) => {
                            let INode{data, children: new_children} = node;
                            let op = ChildCreate {insert_op, new: data};
                            let data = api.node_create(op);
                            let mark = PhantomData;
                            let children = new_children.0
                                .into_iter()
                                .map(|child| STree::from(api, Some(&data), child))
                                .collect::<Vec<STree<M, SN, SL, IN, IL>>>();
                            let children = SChildren {
                                data: children,
                            };
                            STree::Node(SNode {mark, data, children})
                        }
                        ITree::Leaf(leaf) => {
                            let op = ChildCreate {insert_op, new: leaf.data};
                            let data = api.leaf_create(op);
                            let mark = PhantomData;
                            STree::Leaf(SLeaf {mark, data})
                        }
                    }
                }
            }
        }
        // SETUP
        let mut current = self.data
            .iter()
            .enumerate()
            .map(|(ix, x)| (ix, x.get_meta(api)))
            .collect::<Vec<(usize , M)>>();
        let mut old = self.data
            .into_iter()
            .enumerate()
            .map(|(ix, x)| (ix, Rc::new(x)))
            .collect::<Vec<(usize , Rc<STree<M, SN, SL, IN, IL>>)>>();
        let mut free_old_ixs = old
            .iter()
            .map(|(ix, _)| ix.clone())
            .collect::<HashSet<usize>>();
        let mut new = new.0
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, ITree<IN, IL>)>>();
        // SET CHANGED/UNCHANGED/NEW
        let new = new
            .into_iter()
            .map(|(new_ix, new)| -> EntryStatus<ITree<IN, IL>, Rc<STree<M, SN, SL, IN, IL>>> {
                match get_matching_item(&mut free_old_ixs, unimplemented!(), &new, api) {
                    Some((old_ix, old)) => {
                        EntryStatus::Unchanged {
                            new_ix,
                            old_ix,
                            new,
                            old,
                        }
                    },
                    None => {
                        match get_similar_tree(&mut free_old_ixs, unimplemented!(), &new, api) {
                            Some((old_ix, old)) => {
                                match (old.as_ref(), new) {
                                    (STree::Leaf(_), ITree::Leaf(_)) => {
                                        EntryStatus::Changed {
                                            new_ix,
                                            old_ix,
                                            new,
                                            old,
                                        }
                                    }
                                    (STree::Node(_), ITree::Node(_)) => {
                                        EntryStatus::Changed {
                                            new_ix,
                                            old_ix,
                                            new,
                                            old,
                                        }
                                    }
                                    _ => {
                                        EntryStatus::Swap {
                                            new_ix,
                                            old_ix,
                                            new,
                                            old,
                                        }
                                    }
                                }
                            }
                            None => {
                                EntryStatus::New {
                                    new_ix,
                                    new,
                                }
                            }
                        }
                    },
                }
            })
            .collect::<Vec<EntryStatus<ITree<IN, IL>, Rc<STree<M, SN, SL, IN, IL>>>>>();
        // SET INSERT OPERATION
        let new = new
            .into_iter()
            .rev() // NOTE: ...?
            .map(|entry| {
                let mut insert_op: Option<ChildInsert<M>> = None;
                match entry {
                    EntryStatus::Unchanged{new_ix, old_ix, ..} => {
                        let do_inplace = new_ix == old_ix;
                        if !do_inplace {
                            if let Some((_, current_node)) = current.get(new_ix) {
                                insert_op = Some(ChildInsert::Replace {
                                    parent: api.get_meta(Left(parent)),
                                    current_occupant: current_node.clone(),
                                });
                            } else {
                                insert_op = Some(ChildInsert::Append {
                                    parent: api.get_meta(Left(parent)),
                                });
                            }
                        }
                    }
                    EntryStatus::Changed{new_ix, old_ix, ..} => {
                        let do_inplace = new_ix == old_ix;
                        if !do_inplace {
                            if let Some((_, current_node)) = current.get(new_ix) {
                                insert_op = Some(ChildInsert::Replace {
                                    parent: api.get_meta(Left(parent)),
                                    current_occupant: current_node.clone(),
                                });
                            } else {
                                insert_op = Some(ChildInsert::Append {
                                    parent: api.get_meta(Left(parent)),
                                });
                            }
                        }
                    }
                    EntryStatus::Swap{new_ix, old_ix, ..} => {
                        unimplemented!()
                    }
                    EntryStatus::New{new_ix, ..} => {
                        if let Some((_, current_node)) = current.get(new_ix) {
                            insert_op = Some(ChildInsert::Replace {
                                parent: api.get_meta(Left(parent)),
                                current_occupant: current_node.clone(),
                            });
                        } else {
                            insert_op = Some(ChildInsert::Append {
                                parent: api.get_meta(Left(parent)),
                            });
                        }
                    }
                }
                (insert_op, entry)
            })
            .collect::<(Vec<(Option<ChildInsert<M>>, EntryStatus<ITree<IN, IL>, Rc<STree<M, SN, SL, IN, IL>>>)>)>();
        let new = new
            .into_iter()
            .map(|(insert_op, entry)| {
                match entry {
                    EntryStatus::Unchanged{old, new, ..} => {
                        InternalChildUpdate::Update(ChildUpdate{
                            insert_op,
                            new,
                            old,
                        })
                    }
                    EntryStatus::Swap{old, new, ..} => {
                        InternalChildUpdate::Create(ChildCreate {
                            insert_op: Some(ChildInsert::Swap{
                                current_occupant: old.get_meta(api),
                            }),
                            new,
                        })
                    }
                    EntryStatus::Changed{old, new, ..} => {
                        InternalChildUpdate::Update(ChildUpdate{
                            insert_op,
                            new,
                            old,
                        })
                    }
                    EntryStatus::New{new, ..} => {
                        InternalChildUpdate::Create(ChildCreate {
                            insert_op,
                            new,
                        })
                    }
                }
            })
            .collect::<(Vec<(InternalChildUpdate<M, Rc<STree<M, SN, SL, IN, IL>>, ITree<IN, IL>>)>)>();
        // REMOVE UNUSED
        // if current.len() > new.len() {
        //     for (_, unused_entry) in old.into_iter().skip(new.len()) {
        //         let unused_entry = match Rc::try_unwrap(unused_entry) {
        //             Ok(x) => x,
        //             Err(_) => panic!(),
        //         };
        //         match unused_entry {
        //             STree::Leaf(leaf) => {
        //                 api.leaf_remove(ChildRemove {
        //                     parent: api.get_meta(Left(parent)),
        //                     value: leaf.data,
        //                 })
        //             }
        //             STree::Node(node) => {
        //                 api.node_remove(ChildRemove {
        //                     parent: api.get_meta(Left(parent)),
        //                     value: node.data,
        //                 })
        //             }
        //         }
        //     }
        // }
        std::mem::forget(old);
        // PASSES COMPLETE - PROCESS AND RETURN
        let new = new
            .into_iter()
            .map(|(entry)| apply_internal_update(entry, api))
            .collect::<(Vec<(STree<M, SN, SL, IN, IL>)>)>();
        SChildren {
            data: new,
        }
    }
}



