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
    fn node_update(&self, update: Update<&mut SN, &IN>);
    fn node_crate(&self, new: &IN) -> SN;

    fn leaf_unchanged(&self, new: &IL, old: &SL) -> bool;
    fn leaf_recyclable(&self, new: &IL, old: &SL) -> bool;
    fn leaf_update(&self, update: Update<&mut SL, &IL>);
    fn leaf_crate(&self, new: &IL) -> SL;

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
    pub data: L
}

#[derive(Debug, Clone, PartialEq)]
pub struct INode<N, L> {
    pub data: N,
    pub children: IChildren<N, L>,
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
    pub data: Rc<RefCell<SL>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SNode<M, SN, SL, IN, IL> {
    pub mark: PhantomData<(M, SN, SL, IN, IL)>,
    pub data: Rc<RefCell<SN>>,
    pub children: SChildren<M, SN, SL, IN, IL>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SChildren<M, SN, SL, IN, IL>(pub Rc<RefCell<Vec<Rc<RefCell<STree<M, SN, SL, IN, IL>>>>>>);



// ///////////////////////////////////////////////////////////////////////////////
// // MISCELLANEOUS
// ///////////////////////////////////////////////////////////////////////////////

// TODO: DELETE ME!
// Only leads to errors!!!!
pub fn get_item_by<T: PartialEq>(xs: &Vec<T>, f: impl Fn(&T)->bool) -> Option<&T> {
    let pos = xs.iter().position(|x| f(x))?;
    xs.get(pos)
}

pub fn remove_item_by<T: PartialEq>(xs: &mut Vec<T>, f: impl Fn(&T)->bool) -> Option<T> {
    let pos = xs.iter().position(|x| f(x))?;
    Some(xs.remove(pos))
}

impl<M, SN, SL, IN, IL> STree<M, SN, SL, IN, IL> {
    pub fn get_meta(&self, api: &TreeApi<M, SN, SL, IN, IL>) -> M {
        match self {
            STree::Leaf(x) => api.get_meta(Right(&x.data.borrow())),
            STree::Node(x) => api.get_meta(Left(&x.data.borrow())),
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


impl<M, SN, SL, IN, IL> STree<M, SN, SL, IN, IL>
where
    M: PartialEq + Clone + Debug,
    SN: PartialEq + Debug,
    SL: PartialEq + Debug,
    IN: PartialEq + Debug,
    IL: PartialEq + Debug
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
}


impl<M, SN, SL, IN, IL> SLeaf<M, SN, SL, IN, IL>
where
    M: PartialEq + Clone + Debug,
    SN: PartialEq + Debug,
    SL: PartialEq + Debug,
    IN: PartialEq + Debug,
    IL: PartialEq + Debug
{
    pub fn unchanged(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &ILeaf<IL>) -> bool {
        api.leaf_unchanged(&other.data, &self.data.borrow())
    }
    pub fn recyclable(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &ILeaf<IL>) -> bool {
        api.leaf_recyclable(&other.data, &self.data.borrow())
    }
}

impl<M, SN, SL, IN, IL> SNode<M, SN, SL, IN, IL>
where
    M: PartialEq + Clone + Debug,
    SN: PartialEq + Debug,
    SL: PartialEq + Debug,
    IN: PartialEq + Debug,
    IL: PartialEq + Debug
{
    pub fn unchanged(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &INode<IN, IL>) -> bool {
        api.node_unchanged(&other.data, &self.data.borrow()) && self.children.unchanged(api, &other.children)
    }
    pub fn recyclable(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &INode<IN, IL>) -> bool {
        api.node_recyclable(&other.data, &self.data.borrow())
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
    pub fn unchanged(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &IChildren<IN, IL>) -> bool {
        if self.0.borrow().len() == other.0.len() {
            self.0
                .borrow()
                .iter()
                .zip(other.0.iter())
                .all(|(x, y)| x.borrow().unchanged(api, y))
        } else {
            false
        }
    }
    // TODO: run unchanged on all nodes first, then check for recyclable nodes.
    pub fn recyclable(&self, api: &TreeApi<M, SN, SL, IN, IL>, other: &IChildren<IN, IL>) -> bool {
        if self.0.borrow().len() == other.0.len() {
            self.0
                .borrow()
                .iter()
                .zip(other.0.iter())
                .all(|(x, y)| x.borrow().recyclable(api, y))
        } else {
            false
        }
    }
}


impl<IN, IL> ITree<IN, IL> {
    pub fn create_tree<'a,M,SN,SL>(&self, api: &TreeApi<M, SN, SL, IN, IL>, parent: &M) -> STree<M, SN, SL, IN, IL> {
        match self {
            ITree::Leaf(leaf) => {
                let data = api.leaf_crate(&leaf.data);
                let data = Rc::new(RefCell::new(data));
                STree::Leaf(SLeaf {
                    mark: PhantomData,
                    data,
                })
            }
            ITree::Node(node) => {
                let data = api.node_crate(&node.data);
                let mut children_ms = Vec::new();
                let children = node.children.0
                    .iter()
                    .map(|c| {
                        let c = c.create_tree(api, &api.get_meta(Left(&data)));
                        let c = Rc::new(RefCell::new(c));
                        children_ms.push(c.borrow().get_meta(api));
                        c
                    })
                    .collect_vec();
                let children_insert_op = InsertOp::Append {
                    parent: api.get_meta(Left(&data)),
                    new: children_ms,
                };
                api.insert(children_insert_op);
                let children = SChildren(Rc::new(RefCell::new(children)));
                let data = Rc::new(RefCell::new(data));
                STree::Node(SNode {
                    mark: PhantomData,
                    data,
                    children,
                })
            }
        }
    }
}




