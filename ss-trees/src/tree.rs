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
    Replace {
        parent: Option<Parent>,
        current_occupant: Current,
        old: Old,
        new: New,
    },
    Append {
        parent: Option<Parent>,
        old: Old,
        new: New,
    },
    Remove {
        parent: Option<Parent>,
        old: Old,
    }
}

pub trait TreeApi<N1, L1, N2, L2> {
    fn node_unchanged(&self, new: &N1, old: &N2) -> bool;
    fn node_recyclable(&self, new: &N1, old: &N2) -> bool;
    fn node_update(&self, op: ChildUpdate<&N2, Either<&N2, &L2>, Either<&mut N2, &mut L2>, N1>);

    fn leaf_unchanged(&self, new: &L1, old: &L2) -> bool;
    fn leaf_recyclable(&self, new: &L1, old: &L2) -> bool;
    fn leaf_update(&self, op: ChildUpdate<&N2, Either<&N2, &L2>, Either<&mut N2, &mut L2>, L1>);
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
    pub fn unchanged(&self, api: impl TreeApi<SN, SL, IN, IL>, other: ITree<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn recyclable(&self, api: impl TreeApi<SN, SL, IN, IL>, other: ITree<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn sync(&mut self, api: impl TreeApi<SN, SL, IN, IL>, parent: Option<&SN>, new: ITree<IN, IL>) {
        match new {
            ITree::Leaf(new) => {

            }
            ITree::Node(new) => {

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
    pub fn unchanged(&self, api: impl TreeApi<SN, SL, IN, IL>, other: ILeaf<IL>) -> bool {
        unimplemented!()
    }
    pub fn recyclable(&self, api: impl TreeApi<SN, SL, IN, IL>, other: ILeaf<IL>) -> bool {
        unimplemented!()
    }
    pub fn sync(&mut self, api: impl TreeApi<SN, SL, IN, IL>, parent: Option<&SN>, new: ILeaf<IL>) {
        
    }
}

impl<SN, SL, IN, IL> SNode<SN, SL, IN, IL>
where
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: impl TreeApi<SN, SL, IN, IL>, other: INode<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn recyclable(&self, api: impl TreeApi<SN, SL, IN, IL>, other: INode<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn sync(&mut self, api: impl TreeApi<SN, SL, IN, IL>, parent: Option<&SN>, new: INode<IN, IL>) {

    }
}

impl<SN, SL, IN, IL> SChildren<SN, SL, IN, IL>
where
    SN: PartialEq,
    SL: PartialEq,
    IN: PartialEq,
    IL: PartialEq,
{
    pub fn unchanged(&self, api: impl TreeApi<SN, SL, IN, IL>, other: IChildren<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn recyclable(&self, api: impl TreeApi<SN, SL, IN, IL>, other: IChildren<IN, IL>) -> bool {
        unimplemented!()
    }
    pub fn sync(&mut self, api: impl TreeApi<SN, SL, IN, IL>, parent: &SN, new: IChildren<IN, IL>) {

    }
}



