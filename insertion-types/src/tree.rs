use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either;



///////////////////////////////////////////////////////////////////////////////
// TREE
///////////////////////////////////////////////////////////////////////////////

pub struct Interface<N1, L1, N2, L2> {
    node_added: fn(N1)->N2,
    node_modified: fn(N1, N2)->N2,
    node_removed: fn(N2),
    node_unchanged: fn(&N1, &N2)->bool,
    leaf_added: fn(L1)->L2,
    leaf_modified: fn(L1, L2)->L2,
    leaf_removed: fn(L2),
    leaf_unchanged: fn(&L1, &L2)->bool,
}

pub struct ITreeApi<N1, L1, N2, L2> {
    current: ITree<N2, L2>,
    api: Interface<N1, L1, N2, L2>,
}

pub enum ITree<N, L> {
    Leaf {
        data: L,
    },
    Node {
        data: N,
        children: Vec<ITree<N, L>>,
    }
}

impl<N1, L1, N2, L2> ITreeApi<N1, L1, N2, L2>
where
    N1: PartialEq,
    L1: PartialEq,
    N2: PartialEq,
    L2: PartialEq,
{
    pub fn sync(mut self, value: ITree<N1, L1>) {
        let unchanged = value.unchanged(&self.current, &self.api);
        if !unchanged {
            
        }
    }
}

/// Helper function
fn remove_matching_item<N1, L1, N2, L2>(
    old: &mut Vec<ITree<N2, L2>>,
    new: &ITree<N1, L1>,
    api: &Interface<N1, L1, N2, L2>,
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
        if new.is_similar_tree(entry) {
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
    fn remove(self, for_leaf: &Fn(L1), for_node: &Fn(N1)) {
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
    fn added<N2, L2>(self, for_leaf: &Fn(L1)->L2, for_node: &Fn(N1)->N2) -> ITree<N2, L2> {
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
    pub fn is_similar_tree<N2, L2>(&self, other: &ITree<N2, L2>) -> bool
    where
        N2: PartialEq,
        L2: PartialEq,
    {
        match (self, other) {
            (ITree::Leaf{..}, ITree::Leaf{..}) => true,
            (ITree::Node{children: cs1, ..}, ITree::Node{children: cs2, ..}) => {
                if cs1.len() == cs2.len() {
                    cs1 .iter()
                        .zip(cs2.iter())
                        .all(|(c1, c2)| c1.is_similar_tree(c2))
                } else {
                    false
                }
            }
            _ => false
        }
    }
    pub fn sync<N2, L2>(mut self, other: ITree<N2, L2>, api: &Interface<N1, L1, N2, L2>) -> ITree<N2, L2>
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
                            match remove_similar_tree(&mut current, &new) {
                                None => new.added(&api.leaf_added, &api.node_added),
                                Some(old) => new.sync(old, api)
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
    pub fn unchanged<N2, L2>(&self, other: &ITree<N2, L2>, api: &Interface<N1, L1, N2, L2>) -> bool {
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
    pub fn all(&self, pred: &Fn(Either<&N1, &L1>)->bool) -> bool {
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
    pub fn traverse(&self, f: &Fn(&ITree<N1, L1>)) {
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
    pub fn traverse_mut(&self, f: &Fn(&ITree<N1, L1>)) {
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




