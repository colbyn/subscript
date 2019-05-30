use std::marker::PhantomData;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::collections::*;
use either::Either;
use itertools::Itertools;


#[derive(Debug, PartialEq)]
pub struct SMap<N, K, SV, IV>
where
	N: Debug + PartialEq,
    K: Debug + Eq + Hash,
    SV: Debug + PartialEq,
    IV: Debug + PartialEq,
{
    data: HashMap<K, SV, std::collections::hash_map::RandomState>,
    mark: PhantomData<(N, IV)>,
}

impl<N, K, SV, IV> Default for SMap<N, K, SV, IV>
where
    N: Debug + PartialEq,
    K: Debug + Eq + Hash + Clone,
    SV: Debug + PartialEq,
    IV: Debug + PartialEq,
{
    fn default() -> Self {
        SMap {
            data: HashMap::new(),
            mark: PhantomData,
        }
    }
}


pub trait MapApi<N, K, SV, IV>
where
	N: Debug + PartialEq,
    K: Debug + Eq + Hash + Clone,
    SV: Debug + PartialEq,
    IV: Debug + PartialEq,
{
    fn create(&self, attached: &N, key: &K, new: IV) -> SV;
    fn modified(&self, attached: &N, key: &K, old: &mut SV, new: IV);
    fn remove(&self, attached: &N, key: K, old: SV);
    fn unchanged(&self, old: &SV, new: &IV,) -> bool;
}


impl<N, K, SV, IV> SMap<N, K, SV, IV>
where
	N: Debug + PartialEq,
    K: Debug + Eq + Hash + Clone,
    SV: Debug + PartialEq,
    IV: Debug + PartialEq,
{
    pub fn get_keys(&self) -> HashSet<K> {
        let mut ks = HashSet::new();
        for k in self.data.keys() {
            ks.insert(k.clone());
        }
        ks
    }
    pub fn dangerous_unsync_drain(&mut self) -> HashMap<K, SV, std::collections::hash_map::RandomState> {
        self.data.drain().collect()
    }
    pub fn traverse_values_pair(&self, new: &HashMap<K, IV>, f: &Fn(&SV, &IV)) {
        new .iter()
            .for_each(|(k, v)| {
                if let Some(x) = self.data.get(&k) {
                    f(x, v);
                }
            });
    }
    pub fn traverse_values_mut(&mut self, mut f: impl FnMut(&mut SV)) {
        for value in self.data.values_mut() {
            f(value);
        }
    }
    pub fn traverse(&self, f: impl Fn(&K, &SV)) {
        for (k, v) in self.data.iter() {
            f(k, v);
        }
    }
    pub fn unchanged(&self, api: &MapApi<N, K, SV, IV>, new: &HashMap<K, IV>) -> bool {
        let mut is_unchanged = false;
        if self.data.len() == new.len() {
            let eq_keys = {
                let first = self.data.keys().all(
                    |k| new.contains_key(k)
                );
                let second = new.keys().all(
                    |k| self.data.contains_key(k)
                );
                first && second
            };
            if eq_keys {
                let all_eq = self.data
                    .iter()
                    .all(|(k1, v1)| {
                        let v2 = new.get(k1);
                        assert!(v2.is_some());
                        let v2 = v2.unwrap();
                        api.unchanged(v1, v2)
                    });
                is_unchanged = all_eq;
            }
        }
        is_unchanged
    }
	pub fn sync(&mut self, api: &MapApi<N, K, SV, IV>, attached: &N, new: HashMap<K, IV>) {
        let is_unchanged = self.unchanged(api, &new);
        if !is_unchanged {
            let mut old = HashMap::new();
            std::mem::swap(&mut self.data, &mut old);
            let mut new = new
                .into_iter()
                .map(|(k, v)| {
                    let result = {
                        if let Some(x) = old.remove(&k) {
                            if api.unchanged(&x, &v) {
                                x
                            } else {
                                let mut x = x;
                                api.modified(attached, &k, &mut x, v);
                                x
                            }
                        } else {
                            api.create(attached, &k, v)
                        }
                    };
                    (k, result)
                })
                .collect::<HashMap<K, SV>>();
            // REMOVE UNUSED
            for (k, v) in old {
                api.remove(attached, k, v);
            }
            // SAVE CHANGES
            std::mem::replace(&mut self.data, new);
        }
	}
    pub fn sync_ref(&mut self, api: &MapApi<N, K, SV, IV>, attached: &N, new: &HashMap<K, IV>) where IV: Clone {
        let is_unchanged = self.unchanged(api, &new);
        if !is_unchanged {
            let mut old = HashMap::new();
            std::mem::swap(&mut self.data, &mut old);
            let mut new = new
                .iter()
                .map(|(k, v)| {
                    let result = {
                        if let Some(x) = old.remove(&k) {
                            if api.unchanged(&x, &v) {
                                x
                            } else {
                                let mut x = x;
                                api.modified(attached, &k, &mut x, v.clone());
                                x
                            }
                        } else {
                            api.create(attached, &k, v.clone())
                        }
                    };
                    (k.clone(), result)
                })
                .collect::<HashMap<K, SV>>();
            // REMOVE UNUSED
            for (k, v) in old {
                api.remove(attached, k, v);
            }
            // SAVE CHANGES
            std::mem::replace(&mut self.data, new);
        }
    }
}



