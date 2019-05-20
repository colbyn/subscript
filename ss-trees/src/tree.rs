use std::marker::PhantomData;
use std::rc::*;
use std::cell::*;
use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either::{self, Left, Right};



///////////////////////////////////////////////////////////////////////////////
// CLIENT API
///////////////////////////////////////////////////////////////////////////////

pub enum ChildUpdate<Parent, Current, Old, New> {
    RecycleReplace {
        parent: Option<Parent>,
        current_occupant: Current,
        old: Old,
        new: New,
    },
    RecycleAppend {
        parent: Option<Parent>,
        old: Old,
        new: New,
    },
    NewReplace {
        parent: Option<Parent>,
        current_occupant: Current,
        new: New,
    },
    NewAppend {
        parent: Option<Parent>,
        new: New,
    },
    Remove {
        parent: Option<Parent>,
        old: Old,
    },
    Inplace {
        parent: Option<Parent>,
        old: Old,
        new: New,
    }
}

pub enum ChildCreate<Parent, Current, New> {
    NewReplace {
        parent: Option<Parent>,
        current_occupant: Current,
        new: New,
    },
    NewAppend {
        parent: Option<Parent>,
        new: New,
    },
}

pub enum Remount<Parent, T> {
    RemountReplace {
        parent: Option<Parent>,
        current_occupant: T,
        child: T,
    },
    RemountAppend {
        parent: Option<Parent>,
        child: T,
    },
}

pub trait TreeApi<SN, SL, IN, IL> {
    fn node_unchanged(&self, new: &IN, old: &SN) -> bool;
    fn node_recyclable(&self, new: &IN, old: &SN) -> bool;
    fn node_update(&self, op: ChildUpdate<&SN, Either<&SN, &SL>, Either<&mut SN, &mut SL>, IN>);
    fn node_create(&self, op: ChildCreate<&SN, Either<&SN, &SL>, IN>) -> SN;

    fn leaf_unchanged(&self, new: &IL, old: &SL) -> bool;
    fn leaf_recyclable(&self, new: &IL, old: &SL) -> bool;
    fn leaf_update(&self, op: ChildUpdate<&SN, Either<&SN, &SL>, Either<&mut SN, &mut SL>, IL>);
    fn leaf_create(&self, op: ChildCreate<&SN, Either<&SN, &SL>, IL>) -> SL;

    fn remount(&self, op: Remount<&SN, Either<&SN, &SL>>);
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
pub struct ILeaf<L>(pub L);

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
pub enum STree<SN, SL, IN, IL>{
    Leaf(SLeaf<SN, SL, IN, IL>),
    Node(SNode<SN, SL, IN, IL>),
}

#[derive(PartialEq)]
pub struct SLeaf<SN, SL, IN, IL> {
    mark: PhantomData<(SN, SL, IN, IL)>,
    data: SL,
}

#[derive(PartialEq)]
pub struct SNode<SN, SL, IN, IL> {
    mark: PhantomData<(SN, SL, IN, IL)>,
    data: SN,
    children: SChildren<SN, SL, IN, IL>,
}

#[derive(PartialEq)]
pub struct SChildren<SN, SL, IN, IL> {
    data: Vec<STree<SN, SL, IN, IL>>,
}


///////////////////////////////////////////////////////////////////////////////
// SYNC TREE HELPERS 
///////////////////////////////////////////////////////////////////////////////

impl<SN, SL, IN, IL> STree<SN, SL, IN, IL> {
    pub fn to_either(&self) -> Either<&SNode<SN, SL, IN, IL>, &SLeaf<SN, SL, IN, IL>> {
        match self {
            STree::Leaf(x) => Right(x),
            STree::Node(x) => Left(x),
        }
    }
    pub fn to_either_mut(&mut self) -> Either<&mut SNode<SN, SL, IN, IL>, &mut SLeaf<SN, SL, IN, IL>> {
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
    pub fn unpack_leaf(&self) -> Option<(&SLeaf<SN, SL, IN, IL>)> {
        match self {
            STree::Leaf(x) => Some(x),
            STree::Node(_) => None,
        }
    }
    pub fn unpack_node(&self) -> Option<(&SNode<SN, SL, IN, IL>)> {
        match self {
            STree::Node(x) => Some(x),
            STree::Leaf(_) => None,
        }
    }
    pub fn unpack_leaf_mut(&mut self) -> Option<(&mut SLeaf<SN, SL, IN, IL>)> {
        match self {
            STree::Leaf(x) => Some(x),
            STree::Node(_) => None,
        }
    }
    pub fn unpack_node_mut(&mut self) -> Option<(&mut SNode<SN, SL, IN, IL>)> {
        match self {
            STree::Node(x) => Some(x),
            STree::Leaf(_) => None,
        }
    }
}



///////////////////////////////////////////////////////////////////////////////
// SYNC IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

impl<SN, SL, IN, IL> STree<SN, SL, IN, IL>
where
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: &TreeApi<SN, SL, IN, IL>, other: &ITree<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn recyclable(&self, api: &TreeApi<SN, SL, IN, IL>, other: &ITree<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn sync(&mut self, api: &TreeApi<SN, SL, IN, IL>, parent: Option<&SN>, new: ITree<IN, IL>) {
        match new {
            ITree::Leaf(new) => {
                let update: ChildUpdate<&SN, Either<&SN, &SL>, Either<&mut SN, &mut SL>, IL> = ChildUpdate::Inplace {
                    parent: parent,
                    old: self.to_either_inner_mut(),
                    new: new.0,
                };
            }
            ITree::Node(new) => {
                let INode{data: new, children: new_children} = new;
                let update: ChildUpdate<&SN, Either<&SN, &SL>, Either<&mut SN, &mut SL>, IN> = ChildUpdate::Inplace {
                    parent: parent,
                    old: self.to_either_inner_mut(),
                    new: new,
                };
            }
        }
    }
}


impl<SN, SL, IN, IL> SLeaf<SN, SL, IN, IL>
where
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: &TreeApi<SN, SL, IN, IL>, other: &ILeaf<IL>) -> bool {
        unimplemented!()
    }
    pub fn recyclable(&self, api: &TreeApi<SN, SL, IN, IL>, other: &ILeaf<IL>) -> bool {
        unimplemented!()
    }
    pub fn sync(&mut self, api: &TreeApi<SN, SL, IN, IL>, parent: Option<&SN>, new: ILeaf<IL>) {
        
    }
}

impl<SN, SL, IN, IL> SNode<SN, SL, IN, IL>
where
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: &TreeApi<SN, SL, IN, IL>, other: &INode<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn recyclable(&self, api: &TreeApi<SN, SL, IN, IL>, other: &INode<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn sync(&mut self, api: &TreeApi<SN, SL, IN, IL>, parent: Option<&SN>, new: INode<IN, IL>) {

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
    New {
        new_ix: usize,
        new: New,
    },
}


impl<SN, SL, IN, IL> SChildren<SN, SL, IN, IL>
where
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: &TreeApi<SN, SL, IN, IL>, other: &IChildren<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn recyclable(&self, api: &TreeApi<SN, SL, IN, IL>, other: &IChildren<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn sync(&mut self, api: &TreeApi<SN, SL, IN, IL>, parent: &SN, new: IChildren<IN, IL>) {
        // HELPERS
        fn get_matching_item<'a, SN, SL, IN, IL>(
            free_old_ixs: &mut HashSet<usize>,
            old: &Vec<(usize, &'a mut STree<SN, SL, IN, IL>)>,
            new: &ITree<IN, IL>,
            api: &TreeApi<SN, SL, IN, IL>,
        ) -> Option<(usize, &'a mut STree<SN, SL, IN, IL>)>
        where
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
                        Some((ix, x)) => Some((ix.clone(), unimplemented!())),
                        None => panic!()
                    }
                }
            }
        }
        fn get_similar_tree<'a, SN, SL, IN, IL>(
            free_old_ixs: &mut HashSet<usize>,
            old: &Vec<(usize, &'a mut STree<SN, SL, IN, IL>)>,
            new: &ITree<IN, IL>,
            api: &TreeApi<SN, SL, IN, IL>,
        ) -> Option<(usize, &'a mut STree<SN, SL, IN, IL>)>
        where
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
                        Some((ix, x)) => Some((ix.clone(), unimplemented!())),
                        None => panic!()
                    }
                }
            }
        }
        // SETUP
        let mut old = self.data
            .iter_mut()
            .enumerate()
            .map(|(ix, x)| -> (usize ,&mut STree<SN, SL, IN, IL>) {(ix, x)})
            .collect::<Vec<(usize ,&mut STree<SN, SL, IN, IL>)>>();
        let mut free_old_ixs = old
            .iter()
            .map(|(ix, _)| ix.clone())
            .collect::<HashSet<usize>>();
        let mut new = new.0
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, ITree<IN, IL>)>>();
        // SET CHANGED/UNCHANGED/NEW
        let mut new = new
            .into_iter()
            .map(|(new_ix, new)| {
                match get_matching_item(&mut free_old_ixs, &old, &new, api) {
                    Some((old_ix, old)) => {
                        EntryStatus::Unchanged {
                            new_ix,
                            old_ix,
                            new,
                            old,
                        }
                    },
                    None => {
                        match get_similar_tree(&mut free_old_ixs, &old, &new, api) {
                            Some((old_ix, old)) => {
                                EntryStatus::Changed {
                                    new_ix,
                                    old_ix,
                                    new,
                                    old,
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
            .collect::<Vec<EntryStatus<ITree<IN, IL>, &mut STree<SN, SL, IN, IL>>>>();
        // PROCESS RESULTS
        let current = old;
        let new = new .into_iter()
            .map(|entry| {
                match entry {
                    EntryStatus::Unchanged{new_ix, old_ix, new, old} => {
                        // MAYBE REMOUNT
                        if !old_ix == new_ix {
                            if let Some((_, current_node)) = current.get(new_ix) {
                                let op = Remount::RemountReplace {
                                    parent: Some(parent),
                                    current_occupant: current_node.to_either_inner(),
                                    child: old.to_either_inner(),
                                };
                                api.remount(op);
                            } else {
                                let op = Remount::RemountAppend {
                                    parent: Some(parent),
                                    child: old.to_either_inner(),
                                };
                                api.remount(op);
                            }
                        }
                        // DONE
                        old
                    }
                    EntryStatus::Changed{new_ix, old_ix, new, old} => {
                        let do_inplace = new_ix == old_ix;
                        if let Some((_, current_node)) = current.get(new_ix) {
                            match new {
                                ITree::Leaf(leaf) => {
                                    let op = if do_inplace {
                                        ChildUpdate::Inplace {
                                            parent: Some(parent),
                                            old: old.to_either_inner_mut(),
                                            new: leaf.0,
                                        }
                                    } else {
                                        ChildUpdate::RecycleReplace {
                                            parent: Some(parent),
                                            current_occupant: current_node.to_either_inner(),
                                            old: old.to_either_inner_mut(),
                                            new: leaf.0,
                                        }
                                    };
                                    api.leaf_update(op);
                                    old
                                }
                                ITree::Node(node) => {
                                    let INode{data, children} = node;
                                    let op = if do_inplace {
                                        ChildUpdate::Inplace {
                                            parent: Some(parent),
                                            old: old.to_either_inner_mut(),
                                            new: data,
                                        }
                                    } else {
                                        ChildUpdate::RecycleReplace {
                                            parent: Some(parent),
                                            current_occupant: current_node.to_either_inner(),
                                            old: old.to_either_inner_mut(),
                                            new: data,
                                        }
                                    };
                                    api.node_update(op);
                                    old
                                }
                            }
                        } else {
                            match new {
                                ITree::Leaf(leaf) => {
                                    let op = if do_inplace {
                                        ChildUpdate::Inplace {
                                            parent: Some(parent),
                                            old: old.to_either_inner_mut(),
                                            new: leaf.0,
                                        }
                                    } else {
                                        ChildUpdate::RecycleAppend {
                                            parent: Some(parent),
                                            old: old.to_either_inner_mut(),
                                            new: leaf.0,
                                        }
                                    };
                                    api.leaf_update(op);
                                    old
                                }
                                ITree::Node(node) => {
                                    let INode{data, children} = node;
                                    let op = if do_inplace {
                                        ChildUpdate::Inplace {
                                            parent: Some(parent),
                                            old: old.to_either_inner_mut(),
                                            new: data,
                                        }
                                    } else {
                                        ChildUpdate::RecycleAppend {
                                            parent: Some(parent),
                                            old: old.to_either_inner_mut(),
                                            new: data,
                                        }
                                    };
                                    api.node_update(op);
                                    old
                                }
                            }
                        }
                    }
                    EntryStatus::New{new_ix, new} => {
                        if let Some((current_ix, current_node)) = current.get(new_ix) {
                            match new {
                                ITree::Leaf(leaf) => {
                                    let op = ChildCreate::NewReplace {
                                        parent: Some(parent),
                                        current_occupant: current_node.to_either_inner(),
                                        new: leaf.0,
                                    };
                                    let mut r: STree<SN, SL, IN, IL> = STree::Leaf(SLeaf {
                                        mark: PhantomData,
                                        data: api.leaf_create(op)
                                    });
                                    unimplemented!()
                                }
                                ITree::Node(node) => {
                                    let INode{data, children} = node;
                                    let op = ChildCreate::NewReplace {
                                        parent: Some(parent),
                                        current_occupant: current_node.to_either_inner(),
                                        new: data,
                                    };
                                    &mut STree::Node(SNode {
                                        mark: PhantomData,
                                        data: api.node_create(op),
                                        children: unimplemented!()
                                    })
                                }
                            }
                        } else {
                            match new {
                                ITree::Leaf(leaf) => {
                                    let op = ChildCreate::NewAppend {
                                        parent: Some(parent),
                                        new: leaf.0,
                                    };
                                    let r: STree<SN, SL, IN, IL> = STree::Leaf(SLeaf {
                                        mark: PhantomData,
                                        data: api.leaf_create(op)
                                    });
                                    unimplemented!()
                                }
                                ITree::Node(node) => {
                                    let INode{data, children} = node;
                                    let op = ChildCreate::NewAppend {
                                        parent: Some(parent),
                                        new: data,
                                    };
                                    &mut STree::Node(SNode {
                                        mark: PhantomData,
                                        data: api.node_create(op),
                                        children: unimplemented!()
                                    })
                                }
                            }
                        }
                    }
                }
            })
            .collect::<Vec<&mut STree<SN, SL, IN, IL>>>();
    }
}



