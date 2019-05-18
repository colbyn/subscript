pub mod map;

use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either::{self, Left, Right};



///////////////////////////////////////////////////////////////////////////////
// TREE - DATA
///////////////////////////////////////////////////////////////////////////////


pub trait ITreeLogic<N1, L1, N2, L2> {
    fn node_added(&self, parent: Option<&N2>, new: N1) -> N2;
    fn node_modified(&self, parent: Option<&N2>, new: N1, old: &mut N2) -> Result<(), ()>;
    fn node_removed(&self, parent: Option<&N2>, old: N2);
    fn node_unchanged(&self, new: &N1, old: &N2) -> bool;

    fn leaf_added(&self, parent: Option<&N2>, new: L1) -> L2;
    fn leaf_modified(&self, parent: Option<&N2>, new: L1, old: &mut L2) -> Result<(), ()>;
    fn leaf_removed(&self, parent: Option<&N2>, old: L2);
    fn leaf_unchanged(&self, new: &L1, old: &L2) -> bool;
}

pub struct ITreeApi<N1, L1, N2, L2> {
    pub node_added: Box<Fn(N1)->N2>,
    pub node_modified: Box<Fn(N1, N2)->N2>,
    pub node_removed: Box<Fn(N2)>,
    pub node_unchanged: Box<Fn(&N1, &N2)->bool>,
    pub node_adoptable: Box<Fn(&N1, &N2)->bool>,
    pub leaf_added: Box<Fn(L1)->L2>,
    pub leaf_modified: Box<Fn(L1, L2)->L2>,
    pub leaf_removed: Box<Fn(L2)>,
    pub leaf_unchanged: Box<Fn(&L1, &L2)->bool>,
    pub leaf_adoptable: Box<Fn(&L1, &L2)->bool>,
}

#[derive(PartialEq)]
pub enum ITree<N, L> {
    Leaf {
        data: L,
    },
    Node {
        data: N,
        children: Vec<ITree<N, L>>,
    }
}

///////////////////////////////////////////////////////////////////////////////
// TREE - API
///////////////////////////////////////////////////////////////////////////////



impl<N, L> ITree<N, L> {
    pub fn new(value: Either<L, N>) -> Self {
        match value {
            Left(data) => ITree::Leaf{data},
            Right(data) => ITree::Node{data, children: Vec::new()}
        }
    }
    pub fn update_leaf(&mut self, f: &Fn(&mut L)) {
        if let Some(x) = self.unpack_leaf_mut() {
            f(x);
        }
    }
    pub fn update_node(&mut self, f: &Fn(&mut N)) {
        if let Some((x, _)) = self.unpack_node_mut() {
            f(x);
        }
    }
    pub fn add_child(&mut self, child: ITree<N, L>) {
        if let Some((_, xs)) = self.unpack_node_mut() {
            xs.push(child);
        }
    }
    pub fn all(&self, pred: &Fn(Either<&N, &L>)->bool) -> bool {
        match self {
            ITree::Leaf{data} => pred(Either::Right(&data)),
            ITree::Node{data, children} => {
                if pred(Either::Left(&data)) {
                    let mut result = true;
                    for child in children {
                        if result {
                            result = child.all(pred);
                        }
                    }
                    result
                } else {
                    false
                }
            },
        }
    }
    pub fn traverse(&self, f: &Fn(&ITree<N, L>)) {
        match self {
            x @ ITree::Leaf{..} => f(x),
            ITree::Node{data, children} => {
                for child in children {
                    f(child);
                }
                f(self);
            },
        }
    }
    pub fn traverse_mut(&self, f: &Fn(&ITree<N, L>)) {
        match self {
            x @ ITree::Leaf{..} => f(x),
            ITree::Node{data, children} => {
                for child in children {
                    f(child);
                }
                f(self);
            },
        }
    }
}

impl<N2, L2> ITree<N2, L2>
where
    N2: PartialEq,
    L2: PartialEq,
{
    pub fn from<N1, L1>(value: ITree<N1, L1>, api: &ITreeApi<N1, L1, N2, L2>) -> Self
    where
        N1: PartialEq,
        L1: PartialEq,
    {
        value.added(&api.leaf_added, &api.node_added)
    }
    pub fn sync<N1, L1>(mut self, value: ITree<N1, L1>, api: &ITreeApi<N1, L1, N2, L2>)
    where
        N1: PartialEq,
        L1: PartialEq,
    {
        self = value.sync_impl(self, api);
    }
}


///////////////////////////////////////////////////////////////////////////////
// TREE - INTERNAL
///////////////////////////////////////////////////////////////////////////////


/// Helper function
fn remove_matching_item<N1, L1, N2, L2>(
    old: &mut Vec<ITree<N2, L2>>,
    new: &ITree<N1, L1>,
    api: &ITreeApi<N1, L1, N2, L2>,
) -> Option<ITree<N2, L2>>
where
    N1: PartialEq,
    L1: PartialEq,
    N2: PartialEq,
    L2: PartialEq,
{
    use ITree::*;
    let mut return_ix = None;
    for (entry_ix, entry) in old.iter().enumerate() {
        if new.unchanged(&entry, api) {
            if return_ix.is_none() {
                return_ix = Some(entry_ix.clone());
            }
        }
    }
    match return_ix {
        None => None,
        Some(return_ix) => {
            assert!(old.len() > return_ix);
            Some(old.remove(return_ix))
        }
    }
}
fn remove_similar_tree<N1, L1, N2, L2>(
    old: &mut Vec<ITree<N2, L2>>,
    new: &ITree<N1, L1>,
    api: &ITreeApi<N1, L1, N2, L2>,
) -> Option<ITree<N2, L2>>
where
    N1: PartialEq,
    L1: PartialEq,
    N2: PartialEq,
    L2: PartialEq,
{
    use ITree::*;
    let mut return_ix = None;
    for (entry_ix, entry) in old.iter().enumerate() {
        if new.is_similar_tree(entry, api) {
            if return_ix.is_none() {
                return_ix = Some(entry_ix.clone());
            }
        }
    }
    match return_ix {
        None => None,
        Some(return_ix) => {
            assert!(old.len() > return_ix);
            Some(old.remove(return_ix))
        }
    }
}

impl<N1, L1> ITree<N1, L1>
where
    N1: PartialEq,
    L1: PartialEq,
{
    /// Helper for the sync method.
    fn remove(self, for_leaf: &Box<Fn(L1)>, for_node: &Box<Fn(N1)>) {
        match self {
            ITree::Leaf{data} => for_leaf(data),
            ITree::Node{data, children} => {
                for child in children {
                    child.remove(for_leaf, for_node);
                }
                for_node(data);
            },
        }
    }
    /// Helper for the sync method.
    fn added<N2, L2>(self, for_leaf: &Box<Fn(L1)->L2>, for_node: &Box<Fn(N1)->N2>) -> ITree<N2, L2> {
        match self {
            ITree::Leaf{data} => ITree::Leaf{data: for_leaf(data)},
            ITree::Node{data, children} => {
                let children = children
                    .into_iter()
                    .map(|child| child.added(for_leaf, for_node))
                    .collect::<Vec<ITree<N2, L2>>>();
                ITree::Node {
                    data: for_node(data),
                    children: children
                }
            },
        }
    }
    /// Helper for the sync method.
    pub fn is_similar_tree<N2, L2>(&self, other: &ITree<N2, L2>, api: &ITreeApi<N1, L1, N2, L2>) -> bool
    where
        N2: PartialEq,
        L2: PartialEq,
    {
        match (self, other) {
            (ITree::Leaf{data: l1}, ITree::Leaf{data: l2}) if (api.leaf_adoptable)(&l1, &l2)  => true,
            (ITree::Node{children: cs1, data: n1}, ITree::Node{children: cs2, data: n2})
                if (api.node_adoptable)(&n1, &n2) => {
                    if cs1.len() == cs2.len() {
                        cs1 .iter()
                            .zip(cs2.iter())
                            .all(|(c1, c2)| c1.is_similar_tree(c2, api))
                    } else {
                        false
                    }
            }
            _ => false
        }
    }
    fn sync_impl<N2, L2>(mut self, other: ITree<N2, L2>, api: &ITreeApi<N1, L1, N2, L2>) -> ITree<N2, L2>
    where
        N2: PartialEq,
        L2: PartialEq,
    {
        use ITree::*;
        type NewChild<N, L> = ITree<N, L>;
        type ChildIX = usize;
        enum ChildEntry<N, L, N2, L2>{
            Unchanged(ITree<N2, L2>),
            Changed(ChildIX, NewChild<N, L>),
        }
        let sync_leafs = |x: L1, y: L2| -> ITree<N2, L2> {
            Leaf{data: (api.leaf_modified)(x, y)}
        };
        let sync_children = |xs: Vec<ITree<N1, L1>>, ys: Vec<ITree<N2, L2>>| -> Vec<ITree<N2, L2>> {
            // let mut results = Vec::new();
            let mut child_entries = Vec::new();
            let mut current = ys;
            // new_child.added(&api.leaf_added, &api.node_added)
            // UNCHANGED
            for (new_child_ix, new_child) in xs.into_iter().enumerate() {
                match remove_matching_item(&mut current, &new_child, api) {
                    None => child_entries.push(
                        ChildEntry::Changed(new_child_ix, new_child)
                    ),
                    Some(same_current) => child_entries.push(ChildEntry::Unchanged(same_current)),
                }
            }
            // SIMILAR
            let child_entries = child_entries
                .into_iter()
                .map(|x| {
                    match x {
                        ChildEntry::Unchanged(x) => x,
                        ChildEntry::Changed(ix, new) => {
                            match remove_similar_tree(&mut current, &new, api) {
                                None => new.added(&api.leaf_added, &api.node_added),
                                Some(old) => new.sync_impl(old, api)
                            }
                        }
                    }
                })
                .collect::<Vec<ITree<N2, L2>>>();
            child_entries
        };
        let sync_nodes = |(n1, cs1), (n2, cs2)| -> ITree<N2, L2> {
            Node{
                children: sync_children(cs1, cs2),
                data: (api.node_modified)(n1, n2),
            }
        };
        let unchanged = self.unchanged(&other, api);
        if !unchanged {
            match (self, other) {
                (Leaf{data: x}, Leaf{data: y}) => {
                    sync_leafs(x, y)
                },
                (Node{data: x, children: xs}, Node{data: y, children: ys}) => {
                    sync_nodes((x, xs), (y, ys))
                },
                (new, old) => {
                    old.remove(&api.leaf_removed, &api.node_removed);
                    new.added(&api.leaf_added, &api.node_added)
                },
            }
        } else {
            other
        }
    }
    pub fn unchanged<N2, L2>(&self, other: &ITree<N2, L2>, api: &ITreeApi<N1, L1, N2, L2>) -> bool {
        use ITree::*;
        match (self, other) {
            (Leaf{data: x}, Leaf{data: y}) => {
                (api.leaf_unchanged)(x, y)
            },
            (Node{data: x, children: xs}, Node{data: y, children: ys}) => {
                let eq_len = xs.len() == ys.len();
                if (api.node_unchanged)(x, y) && eq_len {
                    xs  .iter()
                        .zip(ys.iter())
                        .all(|(c1, c2)| c1.unchanged(c2, api))
                } else {
                    false
                }
            },
            _ => false,
        }
    }
}


impl<N1, L1> ITree<N1, L1> {
    pub fn unpack_leaf(&self) -> Option<&L1> {
        match self {
            ITree::Leaf{data} => Some(&data),
            _ => None
        }
    }
    pub fn unpack_node(&self) -> Option<(&N1, &Vec<ITree<N1, L1>>)> {
        match self {
            ITree::Node{data, children} => Some((&data, &children)),
            _ => None
        }
    }
    pub fn unpack_leaf_mut(&mut self) -> Option<&mut L1> {
        match self {
            ITree::Leaf{data} => Some(data),
            _ => None
        }
    }
    pub fn unpack_node_mut(&mut self) -> Option<(&mut N1, &mut Vec<ITree<N1, L1>>)> {
        match self {
            ITree::Node{data, children} => Some((data, children)),
            _ => None
        }
    }
}



