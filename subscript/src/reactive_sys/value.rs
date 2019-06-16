use std::cell::*;
use std::rc::*;
use std::sync::mpsc;
use either::{Either, Either::*};

///////////////////////////////////////////////////////////////////////////////
// SUBSCRIPTIONS
///////////////////////////////////////////////////////////////////////////////

type Subscribers<T> = RefCell<Vec<Box<FnMut(&Rc<T>)>>>;

pub(crate) enum SubscribersRef<T> {
    Own(Rc<Subscribers<T>>),
    Weak(Weak<Subscribers<T>>),
}

impl<T> SubscribersRef<T> {
    pub(crate) fn notify_subscribers(&self, sig: &Rc<T>) {
        match self {
            SubscribersRef::Own(ss) => {
                for sub in ss.borrow_mut().iter_mut() {
                    sub(sig);
                }
            }
            SubscribersRef::Weak(ss) => {
                if let Some(ss) = ss.upgrade() {
                    for sub in ss.borrow_mut().iter_mut() {
                        sub(sig);
                    }
                }
            }
        }
    }
}

impl<T> Clone for SubscribersRef<T> {
    fn clone(&self) -> Self {
        match self {
            SubscribersRef::Own(x) => {
                SubscribersRef::Own(x.clone())
            }
            SubscribersRef::Weak(x) => {
                SubscribersRef::Weak(x.clone())
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// VALUE
///////////////////////////////////////////////////////////////////////////////

pub(crate) struct Value<T> {
    pub(crate) subscribers: SubscribersRef<T>,
    pub(crate) getter: Rc<Fn()->Rc<T>>,
    pub(crate) setter: Option<Rc<Fn(Rc<T>)>>,
}

impl<T> std::fmt::Debug for Value<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Value")
    }
}

impl<T> Clone for Value<T> {
    fn clone(&self) -> Self {
        let subscribers =  self.subscribers.clone();
        let getter = self.getter.clone();
        let setter = self.setter.clone();
        Value{subscribers, getter, setter}
    }
}

impl<T: 'static> Value<T> {
    pub(crate) fn new_static(x: T) -> Self {
        let subscribers: SubscribersRef<T> =
            SubscribersRef::Own(Rc::new(RefCell::new(Vec::new())));
        let x = Rc::new(x);
        let getter: Rc<Fn()->Rc<T>> = Rc::new({
            let x = x.clone();
            move || {
                x.clone()
            }
        });
        let setter = None;
        Value{subscribers, getter, setter}
    }
    pub(crate) fn new_mutable(x: T) -> Self {
        let subscribers: SubscribersRef<T> =
            SubscribersRef::Own(Rc::new(RefCell::new(Vec::new())));
        let value: Rc<RefCell<Rc<T>>> = Rc::new(RefCell::new(Rc::new(x)));
        let getter: Rc<Fn()->Rc<T>> = Rc::new({
            let value = value.clone();
            move || {
                value.borrow().clone()
            }
        });
        let setter: Option<Rc<Fn(Rc<T>)>> = Some(Rc::new({
            let value = value.clone();
            move |new_value: Rc<T>| {
                let value = value.clone();
                value.replace(new_value);
            }
        }));
        Value{subscribers, getter, setter}
    }
    pub(crate) fn new_weak(x: Value<T>) -> Self {
        let getter = x.getter.clone();
        let setter = x.setter.clone();
        let subscribers = match &x.subscribers {
            SubscribersRef::Own(x) => {
                SubscribersRef::Weak(Rc::downgrade(&x))
            }
            SubscribersRef::Weak(x) => {
                SubscribersRef::Weak(x.clone())
            }
        };
        Value{subscribers, getter, setter}
    }
    pub(crate) fn get(&self) -> Rc<T> {
        (self.getter)()
    }
    pub(crate) fn set(&self, value: T) {
        if let Some(setter) = &self.setter {
            let value = Rc::new(value);
            self.notify_subscribers(&value);
            (setter)(value);
        }
        else {panic!()}
    }
    pub(crate) fn notify_subscribers(&self, sig: &Rc<T>) {
        self.subscribers.notify_subscribers(sig);
    }
    pub(crate) fn subscribe(&self, f: impl FnMut(&Rc<T>) + 'static) {
        match &self.subscribers {
            SubscribersRef::Own(ss) => {
                ss.borrow_mut().push(Box::new(f));
            }
            SubscribersRef::Weak(ss) => {
                if let Some(ss) = ss.upgrade() {
                    ss.borrow_mut().push(Box::new(f));
                }
            }
        }
    }
    pub(crate) fn map<U: 'static>(&self, f: impl Fn(&T) -> U + 'static) -> Value<U> {
        let subscribers: SubscribersRef<U> =
            SubscribersRef::Own(Rc::new(RefCell::new(Vec::new())));
        let apply = move |x: &T| -> Rc<U> {
            Rc::new(f(x))
        };
        let current_value: Rc<RefCell<Rc<U>>> = Rc::new(RefCell::new(apply(&self.get())));
        let getter: Rc<Fn()->Rc<U>> = Rc::new({
            let current_value = current_value.clone();
            move || {
                current_value.borrow().clone()
            }
        });
        self.subscribe({
            let subscribers = subscribers.clone();
            let current_value = current_value.clone();
            move |value: &Rc<T>| {
                let current_value = current_value.clone();
                current_value.replace(apply(value));
                subscribers.notify_subscribers(&current_value.borrow());
            }
        });
        Value{subscribers, getter, setter: None}
    }
    pub(crate) fn zip<U>(&self, other: &Value<U>) -> Value<(T, U)>
    where
        T: 'static + Clone,
        U: 'static + Clone,
    {
        let subscribers: SubscribersRef<(T, U)> =
            SubscribersRef::Own(Rc::new(RefCell::new(Vec::new())));
        let current_value: Rc<RefCell<Rc<(T, U)>>> = Rc::new(RefCell::new({
            let left: T = (self.getter)().as_ref().clone();
            let right: U = (other.getter)().as_ref().clone();
            Rc::new((left, right))
        }));
        let getter: Rc<Fn()->Rc<(T, U)>> = Rc::new({
            let current_value = current_value.clone();
            move || {
                current_value.borrow().clone()
            }
        });
        self.subscribe({
            let subscribers = subscribers.clone();
            let current_value = current_value.clone();
            move |value: &Rc<T>| {
                let current_value = current_value.clone();
                let t: T = value.as_ref().clone();
                let u: U = {
                    let inner: &Rc<(T, U)> = &current_value.borrow();
                    inner.as_ref().1.clone()
                };
                current_value.replace(Rc::new((t, u)));
                subscribers.notify_subscribers(&current_value.borrow());
            }
        });
        other.subscribe({
            let current_value = current_value.clone();
            let subscribers = subscribers.clone();
            move |value: &Rc<U>| {
                let current_value = current_value.clone();
                let t: T = {
                    let inner: &Rc<(T, U)> = &current_value.borrow();
                    inner.as_ref().0.clone()
                };
                let u: U = value.as_ref().clone();
                current_value.replace(Rc::new((t, u)));
                subscribers.notify_subscribers(&current_value.borrow());
            }
        });
        Value{subscribers, getter, setter: None}
    }
}


