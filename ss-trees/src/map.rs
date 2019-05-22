use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either;


///////////////////////////////////////////////////////////////////////////////
// MAP - CORE
///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct IMap<K, V>
where
    K: Eq + Hash,
    V: PartialEq,
{
    current: HashMap<K, V, std::collections::hash_map::RandomState>,
}

impl<K: Hash + Eq, V: PartialEq> IMap<K, V>
{
    pub fn new() -> Self {
        IMap {
            current: HashMap::new(),
        }
    }
    pub fn insert(&mut self, k: K, v: V) {
        self.current.insert(k, v);
    }
    pub fn into_inner(self) -> Vec<(K, V)> {
        self.current
            .into_iter()
            .collect()
    }
    pub fn traverse(&self, f: impl Fn(&K, &V)) {
        self.current
            .iter()
            .map(|(k, v)| f(k, v))
            .collect::<Vec<()>>();
    }
}

///////////////////////////////////////////////////////////////////////////////
// MAP SYNC API
///////////////////////////////////////////////////////////////////////////////


pub trait IMapLogic<N, K, V1, V2> {
    fn for_added(&self, attached: &N, key: &K, new: V1) -> V2;
    fn for_modified(&self, attached: &N, key: &K, old: &mut V2, new: V1);
    fn for_removed(&self, attached: &N, key: K, old: V2);
    fn is_unchanged(&self, old: &V2, new: &V1,) -> bool;
}

impl<K: Hash + Eq + Clone, V2: PartialEq> IMap<K, V2> {
    pub fn unchanged<N, V1: PartialEq>(&self, values: &IMap<K, V1>, api: &IMapLogic<N, K, V1, V2>) -> bool {
        if self.current.len() == values.current.len() {
            let eq_keys = {
                let first = self.current.keys().all(
                    |k| values.current.contains_key(k)
                );
                let second = values.current.keys().all(
                    |k| self.current.contains_key(k)
                );
                first && second
            };
            if eq_keys {
                self.current
                    .iter()
                    .all(|(k1, v1)| {
                        let v2 = values.current.get(k1);
                        assert!(v2.is_some());
                        let v2 = v2.unwrap();
                        api.is_unchanged(v1, v2)
                    })
            } else {
                false
            }
        } else {
            false
        }
    }
    pub fn sync<N, V1: PartialEq>(&mut self, attached: &N, values: IMap<K, V1>, api: &IMapLogic<N, K, V1, V2>) {
        if self.unchanged(&values, api) {
            
        } else {
            if values.current.len() == self.current.len() {
                let stream = self.current
                    .iter_mut().
                    zip(values.current.into_iter());
                for ((k1, v1), (k2, v2)) in stream {
                    assert!(k1 == &k2);
                    api.for_modified(attached, k1, v1, v2);
                }
            } else {
                let mut new_keys = Vec::new();
                let mut values = values;
                for (new_key, new_value) in values.current.into_iter() {
                    match self.current.get_mut(&new_key) {
                        None => {
                            self.current.insert(
                                new_key.clone(),
                                api.for_added(attached, &new_key, new_value)
                            );
                            new_keys.push(new_key);
                        }
                        Some(old_value) => {
                            api.for_modified(attached, &new_key, old_value, new_value);
                        }
                    }
                }
                let mut old_keys = Vec::new();
                for key in self.current.keys() {
                    if !new_keys.contains(key) {
                        old_keys.push(key.clone());
                    }
                }
                for old_key in old_keys {
                    let old_value = self.current.remove(&old_key);
                    assert!(old_value.is_some());
                    if let Some(old_value) = old_value {
                        api.for_removed(attached, old_key, old_value);
                    }
                }
            }
        }
    }
}
