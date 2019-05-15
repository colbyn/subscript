use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either;


///////////////////////////////////////////////////////////////////////////////
// UTILS
///////////////////////////////////////////////////////////////////////////////

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

///////////////////////////////////////////////////////////////////////////////
// MAP - CORE
///////////////////////////////////////////////////////////////////////////////

pub struct IMap<K, V> {
    current: HashMap<K, V>,
}

impl<K: Hash + Eq, V> IMap<K, V> {
    pub fn new() -> Self {
        IMap {
            current: HashMap::new(),
        }
    }
    pub fn insert(&mut self, k: K, v: V) {
        self.current.insert(k, v);
    }
}

///////////////////////////////////////////////////////////////////////////////
// MAP SYNC API
///////////////////////////////////////////////////////////////////////////////

pub struct IMapApi<K, V1, V2> {
    for_added: fn(&K, V1)->V2,
    for_modified: fn(&K, &mut V2, V1),
    for_removed: fn(K, V2),
    is_unchanged: fn(&V2, &V1,)->bool,
}

impl<K: Hash + Eq + Clone, V2: Hash> IMap<K, V2> {
    pub fn sync<V1: Hash>(mut self, values: HashMap<K, V1>, api: IMapApi<K, V1, V2>) -> Self {
        let unchanged = {
            if self.current.len() == values.len() {
                let eq_keys = {
                    let first = self.current.keys().all(
                        |k| values.contains_key(k)
                    );
                    let second = values.keys().all(
                        |k| self.current.contains_key(k)
                    );
                    first && second
                };
                if eq_keys {
                    self.current
                        .iter()
                        .all(|(k1, v1)| {
                            let v2 = values.get(k1);
                            assert!(v2.is_some());
                            let v2 = v2.unwrap();
                            (api.is_unchanged)(v1, v2)
                        })
                } else {
                    false
                }
            } else {
                false
            }
        };
        if unchanged {
            self
        } else {
            if values.len() == self.current.len() {
                let stream = self.current
                    .iter_mut().
                    zip(values.into_iter());
                for ((k1, v1), (k2, v2)) in stream {
                    assert!(k1 == &k2);
                    (api.for_modified)(k1, v1, v2);
                }
                self
            } else {
                let mut new_keys = Vec::new();
                let mut values = values;
                for (new_key, new_value) in values.into_iter() {
                    match self.current.get_mut(&new_key) {
                        None => {
                            self.current.insert(
                                new_key.clone(),
                                (api.for_added)(&new_key, new_value)
                            );
                            new_keys.push(new_key);
                        }
                        Some(old_value) => {
                            (api.for_modified)(
                                &new_key, old_value, new_value
                            );
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
                        (api.for_removed)(old_key, old_value);
                    }
                }
                self
            }
        }
    }
}


