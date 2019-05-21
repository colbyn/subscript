use std::marker::PhantomData;
use std::rc::*;
use std::cell::*;
use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either::{self, Left, Right};
use itertools::Itertools;



///////////////////////////////////////////////////////////////////////////////
// CLIENT API
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub enum InsertOp<M> {
    InsertBefore {
        current: Vec<M>,
        target: M,
    },
    InsertAfter {
        current: Vec<M>,
        target: M,
    },
    Swap {
        parent: M,
        current: M,
        target: M,
    }
}

#[derive(Debug, Clone)]
pub struct Update<S, I> {
    old: S,
    new: I,
}


pub trait TreeApi<M, SN, SL, IN, IL> {
    fn node_unchanged(&self, new: &IN, old: &SN) -> bool;
    fn node_recyclable(&self, new: &IN, old: &SN) -> bool;
    fn node_update(&self, update: Update<&mut SN, IN>);
    fn node_crate(&self, new: IN) -> SN;

    fn leaf_unchanged(&self, new: &IL, old: &SL) -> bool;
    fn leaf_recyclable(&self, new: &IL, old: &SL) -> bool;
    fn leaf_update(&self, update: Update<&mut SL, IL>);
    fn leaf_crate(&self, new: IL) -> SL;

    fn get_meta(&self, value: Either<&SN, &SL>) -> M;
    fn insert(&self, op: InsertOp<M>);
    fn remove(&self, x: M);
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
    M: Clone + PartialEq,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn from(api: &TreeApi<M, SN, SL, IN, IL>, parent: &M, new: ITree<IN, IL>) -> Self {
        create_tree(api, parent, new)
    }
}


///////////////////////////////////////////////////////////////////////////////
// SYNC IMPLEMENTATION
///////////////////////////////////////////////////////////////////////////////

impl<M, SN, SL, IN, IL> STree<M, SN, SL, IN, IL>
where
    M: Clone + PartialEq,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &ITree<IN, IL>) -> bool {
        match (self, other) {
            (STree::Node(old), ITree::Node(new)) => old.unchanged(api, new),
            (STree::Leaf(old), ITree::Leaf(new)) => old.unchanged(api, new),
            _ => false
        }
    }
    pub fn recyclable(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &ITree<IN, IL>) -> bool {
        match (self, other) {
            (STree::Node(old), ITree::Node(new)) => old.recyclable(api, new),
            (STree::Leaf(old), ITree::Leaf(new)) => old.recyclable(api, new),
            _ => false
        }
    }
    pub fn sync(&mut self, api: &TreeApi<M, SN, SL, IN, IL>, parent: &M, new: ITree<IN, IL>) {
        match new {
            ITree::Leaf(new) => {
                match self {
                    STree::Leaf(old) => {old.sync(api, parent, new)}
                    old @ STree::Node(_) => {
                        let new_tree = create_tree(api, parent, ITree::Leaf(new));
                        let insert_op = InsertOp::Swap {
                            parent: parent.clone(),
                            current: old.get_meta(api),
                            target: new_tree.get_meta(api),
                        };
                        api.insert(insert_op);
                        *old = new_tree;
                    }
                }
            }
            ITree::Node(new) => {
                match self {
                    STree::Node(old) => {old.sync(api, parent, new)}
                    old @ STree::Leaf(_) => {
                        let new_tree = create_tree(api, parent, ITree::Node(new));
                        let insert_op = InsertOp::Swap {
                            parent: parent.clone(),
                            current: old.get_meta(api),
                            target: new_tree.get_meta(api),
                        };
                        api.insert(insert_op);
                        *old = new_tree;
                    }
                }
            }
        }
    }
}


impl<M, SN, SL, IN, IL> SLeaf<M, SN, SL, IN, IL>
where
    M: Clone + PartialEq,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &ILeaf<IL>) -> bool {
        api.leaf_unchanged(&other.data, &self.data)
    }
    pub fn recyclable(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &ILeaf<IL>) -> bool {
        api.leaf_recyclable(&other.data, &self.data)
    }
    pub fn sync(&mut self, api: &TreeApi<M, SN, SL, IN, IL>, parent: &M, new: ILeaf<IL>) {
        api.leaf_update(Update {
            old: &mut self.data,
            new: new.data,
        });
    }
}

impl<M, SN, SL, IN, IL> SNode<M, SN, SL, IN, IL>
where
    M: Clone + PartialEq,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &INode<IN, IL>) -> bool {
        api.node_unchanged(&other.data, &self.data) && self.children.unchanged(api, &other.children)
    }
    pub fn recyclable(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &INode<IN, IL>) -> bool {
        api.node_recyclable(&other.data, &self.data)
    }
    pub fn sync(&mut self, api: &TreeApi<M, SN, SL, IN, IL>, parent: &M, new: INode<IN, IL>) {
        api.node_update(Update {
            old: &mut self.data,
            new: new.data,
        });
        self.children.sync(api, &api.get_meta(Left(&self.data)), new.children);
    }
}


///////////////////////////////////////////////////////////////////////////////
// SYNC IMPLEMENTATION - CHILDREN
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Unchanged,
    Changed,
    New,
}

#[derive(Debug, Clone)]
pub enum Item<S> {
    Unchanged(S),
    Changed(S),
    New(S),
}

impl<S> Item<S> {
    pub fn is_unchanged(&self) -> bool {
        match self {
            Item::Unchanged(_) => true,
            _ => false
        }
    }
    pub fn get_type(&self) -> ItemType {
        match self {
            Item::Unchanged(_) => ItemType::Unchanged,
            Item::Changed(_) => ItemType::Changed,
            Item::New(_) => ItemType::New,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ItemGroup<M, S> {
    Unchanged {
        items: Vec<S>,
    },
    Changed {
        items: Vec<S>,
    },
    New {
        insert_op: InsertOp<M>,
        items: Vec<S>,
    }
}

impl<M, S> ItemGroup<M, S> {
    pub fn extract_items(self) -> Vec<S> {
        match self {
            ItemGroup::Unchanged{items} => items,
            ItemGroup::Changed{items} => items,
            ItemGroup::New{items, ..} => items,
        }
    }
}


impl<M, SN, SL, IN, IL> SChildren<M, SN, SL, IN, IL>
where
    M: Clone + PartialEq,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &IChildren<IN, IL>) -> bool {
        if self.data.len() == other.0.len() {
            self.data
                .iter()
                .zip(other.0.iter())
                .all(|(x, y)| x.unchanged(api, y))
        } else {
            false
        }
    }
    pub fn recyclable(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &IChildren<IN, IL>) -> bool {
        if self.data.len() == other.0.len() {
            self.data
                .iter()
                .zip(other.0.iter())
                .all(|(x, y)| x.recyclable(api, y))
        } else {
            false
        }
    }
    pub fn sync(&mut self, api: &TreeApi<M, SN, SL, IN, IL>, parent: &M, new: IChildren<IN, IL>) {
        let mut xs = new.0
            .into_iter()
            .map(|new| -> Item<STree<M, SN, SL, IN, IL>> {
                let mut unchanged = |olds| remove_item_by(olds, |old| {
                    match (&new, &old) {
                        (ITree::Leaf(x), STree::Leaf(y)) => {
                            api.leaf_unchanged(&x.data, &y.data)
                        }
                        (ITree::Node(x), STree::Node(y)) => {
                            api.node_unchanged(&x.data, &y.data)
                        }
                        _ => false
                    }
                });
                let mut changed = |olds| remove_item_by(olds, |old| {
                    match (&new, &old) {
                        (ITree::Leaf(x), STree::Leaf(y)) => {
                            api.leaf_recyclable(&x.data, &y.data)
                        }
                        (ITree::Node(x), STree::Node(y)) => {
                            api.node_recyclable(&x.data, &y.data)
                        }
                        _ => false
                    }
                });
                if let Some(unchanged) = unchanged(&mut self.data) {
                    Item::Unchanged(unchanged)
                } else {
                    if let Some(mut changed) = changed(&mut self.data) {
                        changed.sync(api, parent, new);
                        Item::Changed(changed)
                    } else {
                        Item::New(create_tree(api, parent, new))
                    }
                }
            })
            .group_by(|x| x.get_type())
            .into_iter()
            .map(|(key, group)| -> (ItemType, Vec<Item<STree<M, SN, SL, IN, IL>>>) {
                (key, group.collect_vec())
            })
            .collect_vec();
        let mut ys: Vec<(ItemType, Vec<M>)> = xs
            .iter()
            .map(|(key, group)| -> (ItemType, Vec<M>) {
                let group_ms = group
                    .iter()
                    .map(|x| -> M {
                        match x {
                            Item::Unchanged(x) => api.get_meta(x.to_either_inner()),
                            Item::Changed(x) => api.get_meta(x.to_either_inner()),
                            Item::New(x) => api.get_meta(x.to_either_inner()),
                        }
                    })
                    .collect_vec();
                (key.clone(), group_ms)
            })
            .collect_vec();
        let mut xs = xs
            .into_iter()
            .enumerate()
            .map(|(ix, (key, group))| -> ItemGroup<M, STree<M, SN, SL, IN, IL>> {
                let group_ms: Vec<M> = group
                    .iter()
                    .map(|x| -> M {
                        match x {
                            Item::Unchanged(x) => api.get_meta(x.to_either_inner()),
                            Item::Changed(x) => api.get_meta(x.to_either_inner()),
                            Item::New(x) => api.get_meta(x.to_either_inner()),
                        }
                    })
                    .collect_vec();
                let insert_op: InsertOp<M> = {
                    if ix <= 1 {
                        match ys.get(ix + 1) {
                            Some((_, after)) => {
                                match after.first() {
                                    Some(target) => InsertOp::InsertBefore{
                                        current: group_ms,
                                        target: target.clone(),
                                    },
                                    None => panic!()
                                }
                            }
                            None => panic!()
                        }
                    } else {
                        match ys.get(ix - 1) {
                            Some((_, before)) => {
                                match before.last() {
                                    Some(target) => InsertOp::InsertAfter{
                                        current: group_ms,
                                        target: target.clone(),
                                    },
                                    None => panic!()
                                }
                            }
                            None => match ys.get(ix + 1) {
                                Some((_, after)) => {
                                    match after.first() {
                                        Some(target) => InsertOp::InsertBefore{
                                            current: group_ms,
                                            target: target.clone(),
                                        },
                                        None => panic!()
                                    }
                                }
                                None => panic!()
                            }
                        }
                    }
                };
                let items: Vec<STree<M, SN, SL, IN, IL>> = group
                    .into_iter()
                    .map(|x| {
                        match x {
                            Item::Unchanged(x) => x,
                            Item::Changed(x) => x,
                            Item::New(x) => x,
                        }
                    })
                    .collect_vec();
                match key {
                    ItemType::Unchanged => ItemGroup::Unchanged{
                        items,
                    },
                    ItemType::Changed => ItemGroup::Changed{
                        items,
                    },
                    ItemType::New => ItemGroup::New{
                        insert_op: insert_op,
                        items,
                    },
                }
            })
            .map(|group| -> Vec<STree<M, SN, SL, IN, IL>> {
                match &group {
                    ItemGroup::New{insert_op, ..} => {
                        api.insert(insert_op.clone());
                    }
                    _ => {}
                }
                group.extract_items()
            })
            .collect_vec();
        // REMOVE UNUSED
        let mut removed = Vec::new();
        std::mem::swap(&mut self.data, &mut removed);
        for r in removed {
            api.remove(api.get_meta(r.to_either_inner()));
        }
        // SAVE CHANGES
        for mut group in xs {
            self.data.append(&mut group);
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// SYNC IMPLEMENTATION - CHILDREN HELPERS
///////////////////////////////////////////////////////////////////////////////

pub fn remove_item_by<T: PartialEq>(xs: &mut Vec<T>, f: impl Fn(&T)->bool) -> Option<T> {
    let pos = xs.iter().position(|x| f(x))?;
    Some(xs.remove(pos))
}

pub fn unsafe_unpack_same_type<M, SN, SL, IN, IL>(
    new: ITree<IN, IL>,
    old: STree<M, SN, SL, IN, IL>
) -> Either<(INode<IN, IL>, SNode<M, SN, SL, IN, IL>), (ILeaf<IL>, SLeaf<M, SN, SL, IN, IL>)>
where
    M: Clone + PartialEq,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,

{
    match new {
        ITree::Leaf(new) => {
            match old {
                STree::Leaf(old) => {
                    Either::Right((new, old))
                }
                old @ STree::Node(_) => {
                    panic!()
                }
            }
        }
        ITree::Node(new) => {
            match old {
                STree::Node(old) => {
                    Either::Left((new, old))
                }
                old @ STree::Leaf(_) => {
                    panic!()
                }
            }
        }
    }   
}

pub fn create_tree<M, SN, SL, IN, IL>(api: &TreeApi<M, SN, SL, IN, IL>, parent: &M, new: ITree<IN, IL>) -> STree<M, SN, SL, IN, IL>
where
    M: Clone + PartialEq,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    match new {
        ITree::Leaf(leaf) => {
            let data = api.leaf_crate(leaf.data);
            STree::Leaf(SLeaf {
                mark: PhantomData,
                data,
            })
        }
        ITree::Node(node) => {
            let data = api.node_crate(node.data);
            let children = node.children.0
                .into_iter()
                .map(|c| {
                    STree::from(api, parent, c)
                })
                .collect_vec();
            let children = SChildren{data: children};
            STree::Node(SNode {
                mark: PhantomData,
                data,
                children,
            })
        }
    }
}

pub fn update_tree<M, SN, SL, IN, IL>(
    api: &TreeApi<M, SN, SL, IN, IL>,
    parent: &M,
    new: ITree<IN, IL>,
    old: &mut STree<M, SN, SL, IN, IL>
) -> Result<(), ()>
where
    M: Clone + PartialEq,
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    match new {
        ITree::Leaf(new) => {
            match old {
                STree::Leaf(old) => {
                    api.leaf_update(Update {
                        old: &mut old.data,
                        new: new.data
                    });
                    Ok(())
                }
                old @ STree::Node(_) => {
                    Err(())
                }
            }
        }
        ITree::Node(new) => {
            match old {
                STree::Node(old) => {
                    api.node_update(Update {
                        old: &mut old.data,
                        new: new.data
                    });
                    Ok(())
                }
                old @ STree::Leaf(_) => {
                    Err(())
                }
            }
        }
    }
}


