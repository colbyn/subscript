use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either;


pub enum ITree<N, L> {
    Leaf {
        data: L,
    },
    Node {
        data: N,
        children: Vec<ITree<N, L>>,
    }
}

impl<N, L> ITree<N, L>
where
    N: PartialEq,
    L: PartialEq,
{
    pub fn similar<N2, L2>(&self, other: ITree<N2, L2>)
    where
        N2: PartialEq,
        L2: PartialEq,
    {
        
    }
}





