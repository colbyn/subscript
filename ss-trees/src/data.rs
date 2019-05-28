use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::*;
use std::cell::*;
use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either::{self, Left, Right};
use itertools::Itertools;
use ss_web_utils::js::console;


///////////////////////////////////////////////////////////////////////////////
// CLIENT API
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub enum InsertOp<M> {
    InsertBefore {
        new: Vec<M>,
        old: M,
    },
    InsertAfter {
        new: Vec<M>,
        old: M,
    },
    Swap {
        parent: M,
        current: M,
        target: M,
    },
    Append {
        parent: M,
        new: Vec<M>,
    }
}

#[derive(Debug, Clone)]
pub struct Update<S, I> {
    pub old: S,
    pub new: I,
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
    SN: PartialEq + Debug,
    SL: PartialEq + Debug,
    IN: PartialEq + Debug,
    IL: PartialEq + Debug
{
    match value {
        None => None,
        Some(x) => Some(api.get_meta(Left(x)))
    }
}




///////////////////////////////////////////////////////////////////////////////
// INSERTION TREE
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub enum ITree<N, L> {
    Leaf(ILeaf<L>),
    Node(INode<N, L>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ILeaf<L> {
    data: L
}

#[derive(Debug, Clone, PartialEq)]
pub struct INode<N, L> {
    data: N,
    children: IChildren<N, L>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IChildren<N, L>(pub Vec<ITree<N, L>>);

///////////////////////////////////////////////////////////////////////////////
// INSERTION TREE HELPERS
///////////////////////////////////////////////////////////////////////////////

impl<N, L> ITree<N, L> {
    fn unpack_node_mut(&mut self) -> Option<&mut INode<N, L>> {
        match self {
            ITree::Node(node) => Some(node),
            _ => None
        }
    }
    pub fn new(value: Either<N, L>) -> Self {
        match value {
            Left(data) => ITree::Node(INode {
                data,
                children: IChildren(Vec::new()),
            }),
            Right(data) => ITree::Leaf(ILeaf{
                data,
            }),
        }
    }
    pub fn add_child(&mut self, value: Self) {
        if let Some(node) = self.unpack_node_mut() {
            node.children.0.push(value);
        }
    }
    pub fn get_node_mut(&mut self) -> Option<&mut N> {
        match self {
            ITree::Node(node) => Some(&mut node.data),
            _ => None
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// SYNC TREE
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub enum STree<M, SN, SL, IN, IL>{
    Leaf(SLeaf<M, SN, SL, IN, IL>),
    Node(SNode<M, SN, SL, IN, IL>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SLeaf<M, SN, SL, IN, IL> {
    pub mark: PhantomData<(M, SN, SL, IN, IL)>,
    pub data: SL,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SNode<M, SN, SL, IN, IL> {
    pub mark: PhantomData<(M, SN, SL, IN, IL)>,
    pub data: SN,
    pub children: SChildren<M, SN, SL, IN, IL>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SChildren<M, SN, SL, IN, IL> {
    pub data: Vec<STree<M, SN, SL, IN, IL>>,
}



