pub mod map;


use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either::{self, Left, Right};



///////////////////////////////////////////////////////////////////////////////
// TREE - DATA
///////////////////////////////////////////////////////////////////////////////

pub enum ChildInsert<Parent, Current, Old, New> {
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
}

pub trait ITreeLogic<N1, L1, N2, L2> {
    fn node_added(&self, parent: Option<&N2>, new: N1) -> N2;
    fn node_modified(&self, parent: Option<&N2>, new: N1, old: &mut N2) -> Result<(), ()>;
    fn node_removed(&self, parent: Option<&N2>, old: N2);
    fn node_unchanged(&self, new: &N1, old: &N2) -> bool;
    fn node_adoptable(&self, new: &N1, old: &N2) -> bool;
    fn node_insert(&self, op: ChildInsert<&N2, Either<&N2, &L2>, Either<N2, L2>, N1>) -> N2;

    fn leaf_added(&self, parent: Option<&N2>, new: L1) -> L2;
    fn leaf_modified(&self, parent: Option<&N2>, new: L1, old: &mut L2) -> Result<(), ()>;
    fn leaf_removed(&self, parent: Option<&N2>, old: L2);
    fn leaf_unchanged(&self, new: &L1, old: &L2) -> bool;
    fn leaf_adoptable(&self, new: &L1, old: &L2) -> bool;
    fn leaf_insert(&self, op: ChildInsert<&N2, Either<&N2, &L2>, Either<N2, L2>, L1>) -> L2;
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

#[derive(PartialEq)]
pub struct Children<N, L> {
    data: Vec<ITree<N, L>>,
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
    pub fn from<N1, L1>(value: ITree<N1, L1>, api: &ITreeLogic<N1, L1, N2, L2>) -> Self
    where
        N1: PartialEq,
        L1: PartialEq,
    {
        value.insert(None, api)
    }
    pub fn sync<N1, L1>(mut self, parent: Option<&N2>, value: ITree<N1, L1>, api: &ITreeLogic<N1, L1, N2, L2>)
    where
        N1: PartialEq,
        L1: PartialEq,
    {
        self = value.sync_impl(self, parent, api);
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



///////////////////////////////////////////////////////////////////////////////
// TREE - INTERNAL
///////////////////////////////////////////////////////////////////////////////


/// Helper function
fn remove_matching_item<N1, L1, N2, L2>(
    old: &mut Vec<ITree<N2, L2>>,
    new: &ITree<N1, L1>,
    api: &ITreeLogic<N1, L1, N2, L2>,
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
    api: &ITreeLogic<N1, L1, N2, L2>,
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
    fn sync_impl<N2, L2>(mut self, other: ITree<N2, L2>, parent: Option<&N2>, api: &ITreeLogic<N1, L1, N2, L2>) -> ITree<N2, L2>
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
        let sync_leafs = |x: L1, mut y: L2| -> ITree<N2, L2> {
            match api.leaf_modified(parent, x, &mut y) {
                Ok(_) => (),
                Err(_) => (),
            }
            ITree::Leaf{data: y}
        };
        let sync_children = |parent: &N2, xs: Vec<ITree<N1, L1>>, ys: Vec<ITree<N2, L2>>| -> Vec<ITree<N2, L2>> {
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
                                None => new.insert(Some(parent), api),
                                Some(old) => new.sync_impl(old, Some(parent), api),
                            }
                        }
                    }
                })
                .collect::<Vec<ITree<N2, L2>>>();
            child_entries
        };
        let sync_nodes = |(n1, cs1), (mut n2, cs2)| -> ITree<N2, L2> {
            match api.node_modified(parent, n1, &mut n2) {
                Err(_) => (),
                Ok(_) => (),
            }
            ITree::Node {
                children: sync_children(&n2, cs1, cs2),
                data: n2,
            }
        };
        let unchanged = self.unchanged(&other, api);
        if !unchanged {
            match (self, other) {
                (Leaf{data: x}, Leaf{data: y}) => {
                    sync_leafs(x, y)
                },
                (Node{data: x, children: xs}, Node{data: mut y, children: ys}) => {
                    sync_nodes((x, xs), (y, ys))
                },
                (new, old) => {
                    old.remove(parent, api);
                    new.insert(parent, api)
                },
            }
        } else {
            other
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// MISC HELPERS
///////////////////////////////////////////////////////////////////////////////

impl<N1, L1> ITree<N1, L1> {
    pub fn to_either(&self) -> Either<&N1, &L1> {
        match self {
            ITree::Node{data, ..} => Either::Left(data),
            ITree::Leaf{data} => Either::Right(data),
        }
    }
    pub fn to_either_own(self) -> Either<(N1, Vec<ITree<N1, L1>>), L1> {
        match self {
            ITree::Node{data, children} => Either::Left((data, children)),
            ITree::Leaf{data} => Either::Right(data),
        }
    }
    pub fn unchanged<N2, L2>(&self, other: &ITree<N2, L2>, api: &ITreeLogic<N1, L1, N2, L2>) -> bool {
        unchanged(self, other, api)
    }
    pub fn is_similar_tree<N2, L2>(&self, other: &ITree<N2, L2>, api: &ITreeLogic<N1, L1, N2, L2>) -> bool {
        is_similar_tree(self, other, api)
    }
}

#[inline]
pub fn unchanged<N1, N2, L1, L2>(x: &ITree<N1, L1>, y: &ITree<N2, L2>, api: &ITreeLogic<N1, L1, N2, L2>) -> bool {
    use ITree::*;
    match (x, y) {
        (Leaf{data: x}, Leaf{data: y}) => {
            api.leaf_unchanged(x, y)
        },
        (Node{data: x, children: xs}, Node{data: y, children: ys}) => {
            let eq_len = xs.len() == ys.len();
            if api.node_unchanged(x, y) && eq_len {
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

#[inline]
pub fn is_similar_tree<N1, N2, L1, L2>(x: &ITree<N1, L1>, y: &ITree<N2, L2>, api: &ITreeLogic<N1, L1, N2, L2>) -> bool {
    match (x, y) {
        (ITree::Leaf{data: l1}, ITree::Leaf{data: l2}) if api.leaf_adoptable(&l1, &l2)  => true,
        (ITree::Node{children: cs1, data: n1}, ITree::Node{children: cs2, data: n2})
            if api.node_adoptable(&n1, &n2) => {
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



///////////////////////////////////////////////////////////////////////////////
// SYNC VARIANT HELPERS
///////////////////////////////////////////////////////////////////////////////


pub fn sync_leaves<N1, N2, L1, L2>(new: L1, mut old: L2, parent: Option<&N2>, api: &ITreeLogic<N1, L1, N2, L2>) {
    unimplemented!()
}
pub fn sync_nodes<N1, N2, L1, L2>(new: N1, mut old: N2, parent: Option<&N2>, api: &ITreeLogic<N1, L1, N2, L2>) {
    unimplemented!()
}


///////////////////////////////////////////////////////////////////////////////
// SYNC CHILDREN HELPERS
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

pub fn sync_children<N1, N2, L1, L2>(
    new: Vec<ITree<N1, L1>>,
    old: Vec<ITree<N2, L2>>,
    parent: &N2,
    api: &ITreeLogic<N1, L1, N2, L2>,
)
where
    N1: PartialEq,
    L1: PartialEq,
    N2: PartialEq,
    L2: PartialEq,
{
    // HELPERS
    fn remove_matching_item<N1, L1, N2, L2>(
        old: &Vec<(usize, Rc<ITree<N2, L2>>)>,
        new: &ITree<N1, L1>,
        api: &ITreeLogic<N1, L1, N2, L2>,
    ) -> Option<(usize, Rc<ITree<N2, L2>>)>
    where
        N1: PartialEq,
        L1: PartialEq,
        N2: PartialEq,
        L2: PartialEq,
    {
        use ITree::*;
        let mut return_ix = None;
        for (entry_ix, entry) in old.iter() {
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
                match old.get(return_ix) {
                    Some((ix, x)) => Some((ix.clone(), x.clone())),
                    None => panic!()
                }
            }
        }
    }
    fn remove_similar_tree<N1, L1, N2, L2>(
        old: &Vec<(usize, Rc<ITree<N2, L2>>)>,
        new: &ITree<N1, L1>,
        api: &ITreeLogic<N1, L1, N2, L2>,
    ) -> Option<(usize, Rc<ITree<N2, L2>>)>
    where
        N1: PartialEq,
        L1: PartialEq,
        N2: PartialEq,
        L2: PartialEq,
    {
        use ITree::*;
        let mut return_ix = None;
        for (entry_ix, entry) in old.iter() {
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
                match old.get(return_ix) {
                    Some((ix, x)) => Some((ix.clone(), x.clone())),
                    None => panic!()
                }
            }
        }
    }
    // SETUP
    let old = old
        .into_iter()
        .enumerate()
        .map(|(ix, x)| (ix, Rc::new(x)))
        .collect::<Vec<(usize, Rc<ITree<N2, L2>>)>>();
    let mut new = new
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, ITree<N1, L1>)>>();
    // SET CHANGED/UNCHANGED/NEW
    let mut new = new
        .into_iter()
        .map(|(new_ix, new)| {
            match remove_matching_item(&old, &new, api) {
                Some((old_ix, old)) => {
                    EntryStatus::Unchanged {
                        new_ix,
                        old_ix,
                        new,
                        old,
                    }
                },
                None => {
                    match remove_similar_tree(&old, &new, api) {
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
        .collect::<Vec<EntryStatus<ITree<N1, L1>, Rc<ITree<N2, L2>>>>>();
    // PROCESS RESULTS
    let current = old;
    new .into_iter()
        .map(|entry| {
            match entry {
                EntryStatus::Unchanged{new_ix, old_ix, new, old} => {
                    assert!(new_ix == old_ix);
                    let r = match Rc::try_unwrap(old) {
                        Ok(x) => x,
                        Err(_) => panic!(),
                    };
                }
                EntryStatus::Changed{new_ix, old_ix, new, old} => {
                    let old = match Rc::try_unwrap(old) {
                        Ok(x) => x,
                        Err(_) => panic!(),
                    };
                    let (old, old_children) = match old.to_either_own() {
                        Left((old, children)) => (Either::Left(old), children),
                        Right(old) => (Either::Right(old), vec![]),
                    };
                    if let Some((current_ix, current_node)) = current.get(new_ix) {
                        let r: ITree<N2, L2> = match new {
                            ITree::Leaf{data} => {
                                let op = ChildInsert::Replace {
                                    parent: Some(parent),
                                    current_occupant: current_node.to_either(),
                                    old: unimplemented!(),
                                    new: unimplemented!(),
                                };
                                ITree::Leaf{
                                    data: api.leaf_insert(op)
                                }
                            }
                            ITree::Node{data, children} => {
                                let op = ChildInsert::Replace {
                                    parent: Some(parent),
                                    current_occupant: current_node.to_either(),
                                    old: unimplemented!(),
                                    new: unimplemented!(),
                                };
                                ITree::Node{
                                    data: api.node_insert(op),
                                    children: unimplemented!()
                                }
                            }
                        };
                    } else {
                        let r: ITree<N2, L2> = match new {
                            ITree::Leaf{data} => {
                                let op = ChildInsert::Append {
                                    parent: Some(parent),
                                    old: unimplemented!(),
                                    new: unimplemented!(),
                                };
                                ITree::Leaf{
                                    data: api.leaf_insert(op)
                                }
                            }
                            ITree::Node{data, children} => {
                                let op = ChildInsert::Append {
                                    parent: Some(parent),
                                    old: unimplemented!(),
                                    new: unimplemented!(),
                                };
                                ITree::Node{
                                    data: api.node_insert(op),
                                    children: unimplemented!()
                                }
                            }
                        };
                    }
                }
                EntryStatus::New{new_ix, new} => {
                    if let Some(current_node) = current.get(new_ix) {

                    } else {

                    }
                }
            }
            unimplemented!()
        })
        .collect::<Vec<ITree<N2, L2>>>();
}


///////////////////////////////////////////////////////////////////////////////
// SYNC CRUD HELPERS
///////////////////////////////////////////////////////////////////////////////

impl<N1, L1> ITree<N1, L1> {
    pub fn insert<N2, L2>(self, parent: Option<&N2>, api: &ITreeLogic<N1, L1, N2, L2>) -> ITree<N2, L2> {
        insert(self, parent, api)
    }
}

impl<N2, L2> ITree<N2, L2> {
    pub fn remove<N1, L1>(self, parent: Option<&N2>, api: &ITreeLogic<N1, L1, N2, L2>) {
        remove(self, parent, api);
    }
}


pub fn insert<N1, L1, N2, L2>(new: ITree<N1, L1>, parent: Option<&N2>, api: &ITreeLogic<N1, L1, N2, L2>) -> ITree<N2, L2> {
    match new {
        ITree::Leaf{data} => ITree::Leaf{data: api.leaf_added(parent, data)},
        ITree::Node{data, children} => {
            let children = children
                .into_iter()
                .map(|child| child.insert(parent, api))
                .collect::<Vec<ITree<N2, L2>>>();
            ITree::Node {
                data: api.node_added(parent, data),
                children: children
            }
        },
    }
}
pub fn remove<N1, L1, N2, L2>(old: ITree<N2, L2>, parent: Option<&N2>, api: &ITreeLogic<N1, L1, N2, L2>) {
    match old {
        ITree::Leaf{data} => {
            api.leaf_removed(parent, data);
        }
        ITree::Node{data, children} => {
            api.node_removed(parent, data);
        }
    }
}


