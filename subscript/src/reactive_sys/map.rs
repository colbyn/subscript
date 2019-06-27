pub mod view_observer;

use std::hash::Hash;
use std::fmt::Debug;
use std::marker::*;
use std::any::*;
use std::cell::*;
use std::rc::*;
use std::collections::*;

use crate::reactive_sys::signal::*;
use crate::reactive_sys::value::*;
use crate::reactive_sys::value;

///////////////////////////////////////////////////////////////////////////////
// OBSERVERS
///////////////////////////////////////////////////////////////////////////////

pub trait MapOpObserver<K, V> {
    fn insert_op(&mut self, key: &K, value: &V);
    fn remove_op(&mut self, key: &K);
    fn clear_op(&mut self);
}

///////////////////////////////////////////////////////////////////////////////
// MAP-SIGNAL
///////////////////////////////////////////////////////////////////////////////

pub struct MapSignal<K, V> {
    pub(crate) value: Rc<RefCell<HashMap<K, V>>>,
    pub(crate) op_subscribers: Rc<RefCell<Vec<Box<MapOpObserver<K, V>>>>>,
    pub(crate) change_subscribers: Rc<RefCell<Vec<Box<FnMut(&HashMap<K, V>)>>>>,
}

impl<K, V> MapSignal<K, V>
where
    K: Clone + 'static + Eq + Hash,
    V: Clone + 'static
{
    pub fn new() -> Self {
        let value = Default::default();
        let op_subscribers = Default::default();
        let change_subscribers = Default::default();
        MapSignal{value, op_subscribers, change_subscribers}
    }
    pub fn insert(&mut self, key: K, value: V) {
        for sub in self.op_subscribers.borrow_mut().iter_mut() {
            sub.insert_op(&key, &value);
        }
        self.value.borrow_mut().insert(key, value);
        for sub in self.change_subscribers.borrow_mut().iter_mut() {
            sub(&self.value.borrow());
        }
    }
    pub fn remove(&mut self, key: &K) {
        for sub in self.op_subscribers.borrow_mut().iter_mut() {
            sub.remove_op(&key);
        }
        self.value.borrow_mut().remove(key);
        for sub in self.change_subscribers.borrow_mut().iter_mut() {
            sub(&self.value.borrow());
        }
    }
    pub fn clear(&mut self) {
        for sub in self.op_subscribers.borrow_mut().iter_mut() {
            sub.clear_op();
        }
        self.value.borrow_mut().clear();
        for sub in self.change_subscribers.borrow_mut().iter_mut() {
            sub(&self.value.borrow());
        }
    }
    pub fn reduce_formula<T: 'static>(&self, f: impl Fn(&HashMap<K, V>)->T + 'static) -> Formula<T> {
        let subscribers: value::SubscribersRef<T> =
            value::SubscribersRef::Own(Rc::new(RefCell::new(Vec::new())));
        let apply = move |x: &HashMap<K, V>| -> Rc<T> {
            Rc::new(f(x))
        };
        let current_value: Rc<RefCell<Rc<T>>> = Rc::new(RefCell::new(apply(&self.value.borrow())));
        let getter: Rc<Fn()->Rc<T>> = Rc::new({
            let current_value = current_value.clone();
            move || {
                current_value.borrow().clone()
            }
        });
        let result = value::Value{subscribers, getter, setter: None};
        self.change_subscribers.borrow_mut().push(Box::new({
            let current_value = current_value.clone();
            let result = result.clone();
            move |new_value: &HashMap<K, V>| {
                let result = result.clone();
                current_value.replace(apply(new_value));
                result.notify_subscribers(&current_value.borrow());
            }
        }));
        Formula(result)
    }
    pub fn get_formula(&self, key: &K) -> Formula<Option<V>> {
        let key = key.clone();
        self.reduce_formula(move |x| {
            let x = x.get(&key.clone())?;
            Some(x.clone())
        })
    }
    pub fn contains_key_formula(&self, key: &K) -> Formula<bool> {
        let key = key.clone();
        self.reduce_formula(move |x| x.contains_key(&key.clone()))
    }
    pub fn is_empty_formula(&self) -> Formula<bool> {
        self.reduce_formula(move |x| x.is_empty())
    }
    pub fn len_formula(&self) -> Formula<usize> {
        self.reduce_formula(move |x| x.len())
    }
}

impl<K, V> Default for MapSignal<K, V>
where
    K: Default + Eq + Hash,
    V: Default,
{
    fn default() -> Self {
        MapSignal {
            value: Default::default(),
            op_subscribers: Default::default(),
            change_subscribers: Default::default(),
        }
    }
}
impl<K, V> Clone for MapSignal<K, V> {
    fn clone(&self) -> Self {
        let value = self.value.clone();
        let op_subscribers = self.op_subscribers.clone();
        let change_subscribers = self.change_subscribers.clone();
        MapSignal{value, op_subscribers, change_subscribers}
    }
}

///////////////////////////////////////////////////////////////////////////////
// READ-ONLY MAP-SIGNAL
///////////////////////////////////////////////////////////////////////////////

pub struct MapFormula<K, V>(pub(crate) MapSignal<K, V>);

impl<K, V> MapFormula<K, V>
where
    K: Clone + 'static + Eq + Hash,
    V: Clone + 'static
{
    pub(crate) fn reduce_formula<T: 'static>(&self, f: impl Fn(&HashMap<K, V>) -> T + 'static) -> Formula<T> {
        self.0.reduce_formula(f)
    }
}

impl<K,V> Default for MapFormula<K, V>
where
    K: Default + Eq + Hash,
    V: Default,
{
    fn default() -> Self {
        MapFormula(MapSignal::default())
    }
}
impl<K,V> Clone for MapFormula<K, V> {
    fn clone(&self) -> Self {
        MapFormula(self.0.clone())
    }
}

