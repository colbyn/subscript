use std::fmt::Debug;
use std::marker::*;
use std::any::*;
use std::cell::*;
use std::rc::*;
use std::collections::*;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use either::{Either, Either::*};


///////////////////////////////////////////////////////////////////////////////
// REACTIVE - GENERIC SIGNAL INTERFACE
///////////////////////////////////////////////////////////////////////////////

pub trait Reactive<T> {
    fn get_either_val_or_ref(&self) -> Either<T, &T>;
    fn add_observer(&self, new: Box<SignalObserver<T>>) -> Either<T, &T>;
}

pub trait SignalObserver<T> {
    fn set_op(&mut self, new: &T);
}


///////////////////////////////////////////////////////////////////////////////
// SIGNAL
///////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct Signal<T> {
    pub(crate) value: T,
    #[serde(skip)]
    observers: RefCell<Vec<Box<SignalObserver<T>>>>,
}

impl<T: Default> Default for Signal<T> {
    fn default() -> Self {
        Signal {
            value: Default::default(),
            observers: RefCell::new(Vec::new()),
        }
    }
}

// EXTERNAL API
impl<T> Signal<T> {
    pub fn new(value: T) -> Self {
        Signal{value, observers: RefCell::new(Vec::new())}
    }
    pub fn set(&mut self, new: T) -> T {
        for observer in self.observers.borrow_mut().iter_mut() {
            observer.set_op(&new);
        }
        let mut new = new;
        std::mem::replace(&mut self.value, new)
    }
    pub fn get_clone(&self) -> T where T: Clone {
        self.value.clone()
    }
    pub fn get(&self) -> &T {
        &self.value
    }
    pub fn map_mut<R>(&mut self, f: impl FnMut(&mut T)->R) -> R {
        let mut f = Box::new(f);
        let result = f(&mut self.value);
        for observer in self.observers.borrow_mut().iter_mut() {
            observer.set_op(&self.value);
        }
        result
    }
    pub fn computed<O>(&self, f: impl Fn(&T) -> O + 'static) -> ComputedSignal<T, O>
    where
        T: Default + 'static,
        O: Default + 'static,
    {
        let f = Box::new(f);
        let output: Rc<RefCell<O>> = Rc::new(RefCell::new(f(&self.value)));
        let observers = Rc::new(RefCell::new(Vec::new()));
        let weak: ComputedValue<T, O> = ComputedValue::Weak{
            value: Rc::downgrade(&output),
            mapper: Some(Rc::new(Mapper(f))),
            observers: Rc::downgrade(&observers),
        };
        let own: ComputedSignal<T, O> = ComputedSignal(RefCell::new(Some(ComputedValue::Own{
            value: output,
            observers,
        })));
        self.observers.borrow_mut().push(Box::new(weak));
        own
    }
}



impl<T> Reactive<T> for &Signal<T> {
    fn get_either_val_or_ref(&self) -> Either<T, &T> {
        Right(&self.value)
    }
    fn add_observer(&self, new: Box<SignalObserver<T>>) -> Either<T, &T> {
        self.observers.borrow_mut().push(new);
        Right(&self.value)
    }
}


impl<T> Reactive<T> for Signal<T> {
    fn get_either_val_or_ref(&self) -> Either<T, &T> {
        Right(&self.value)
    }
    fn add_observer(&self, new: Box<SignalObserver<T>>) -> Either<T, &T> {
        self.observers.borrow_mut().push(new);
        Right(&self.value)
    }
}


///////////////////////////////////////////////////////////////////////////////
// CELL-OBSERVER
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub(crate) struct CellObserver<T: Clone>(CellState<T>);

#[derive(Debug, Clone)]
enum CellState<T: Clone> {
    Weak {
        value: Weak<RefCell<T>>
    },
    Own {
        value: Rc<RefCell<T>>
    }
}


impl<T: Clone> SignalObserver<T> for CellObserver<T> {
    fn set_op(&mut self, new: &T) {
        let terminate = || {
            let x1 = "INTERNAL ERROR - IMPOSSIBLE!";
            let x2 = "Obviously this should not be called from the own side.";
            panic!([x1, x2].join(" "));
        };
        match self {
            CellObserver(CellState::Weak{value}) => {
                if let Some(value) = value.upgrade() {
                    value.replace(new.clone());
                }
            }
            CellObserver(CellState::Own{..}) => terminate()
        }
    }
}


impl<T: Clone + 'static> CellObserver<T> {
    pub(crate) fn new(reactive: &Reactive<T>) -> Self
    {
        let value = match reactive.get_either_val_or_ref() {
            Left(x) => x,
            Right(x) => x.clone(),
        };
        let value = Rc::new(RefCell::new(value));
        let weak = CellObserver(CellState::Weak{
            value: Rc::downgrade(&value),
        });
        let own = CellObserver(CellState::Own{value});
        reactive.add_observer(Box::new(weak));
        own
    }
    pub(crate) fn get(&self) -> T {
        match self {
            CellObserver(CellState::Own{value}) => {value.borrow().clone()}
            CellObserver(CellState::Weak{..}) => panic!()
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// COMPUTED-SIGNAL
///////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Default)]
pub struct ComputedSignal<I, O>(RefCell<Option<ComputedValue<I, O>>>)
where
    I: Default,
    O: Default;

impl<I, O> ComputedSignal<I, O>
where
    I: Default,
    O: Default + Clone,
{
    pub(crate) fn get_or(&self, default: O) -> O {
        let inner: &Option<ComputedValue<I, O>> = &self.0.borrow();
        match &inner {
            None => default,
            Some(ComputedValue::Own{value,..}) => {
                value.borrow().clone()
            }
            Some(ComputedValue::Weak{value,..}) => panic!()
        }
    }
}

impl<I, O> Reactive<O> for &ComputedSignal<I, O>
where
    I: Default,
    O: Default + Clone,
{
    fn get_either_val_or_ref(&self) -> Either<O, &O> {
        let inner: &Option<ComputedValue<I, O>> = &self.0.borrow();
        match &inner {
            Some(ComputedValue::Own{value,..}) => {
                Left(value.borrow().clone())
            }
            Some(ComputedValue::Weak{..}) => panic!(),
            None => panic!(),
        }
    }
    fn add_observer(&self, new: Box<SignalObserver<O>>) -> Either<O, &O> {
        let inner: &Option<ComputedValue<I, O>> = &self.0.borrow();
        match &inner {
            Some(ComputedValue::Own{value,observers}) => {
                observers.borrow_mut().push(new);
                Left(value.borrow().clone())
            }
            Some(ComputedValue::Weak{..}) => panic!(),
            None => panic!(),
        }
    }
}

impl<I, O> Reactive<O> for ComputedSignal<I, O>
where
    I: Default,
    O: Default + Clone,
{
    fn get_either_val_or_ref(&self) -> Either<O, &O> {
        let inner: &Option<ComputedValue<I, O>> = &self.0.borrow();
        match &inner {
            Some(ComputedValue::Own{value,..}) => {
                Left(value.borrow().clone())
            }
            Some(ComputedValue::Weak{..}) => panic!(),
            None => panic!(),
        }
    }
    fn add_observer(&self, new: Box<SignalObserver<O>>) -> Either<O, &O> {
        let inner: &Option<ComputedValue<I, O>> = &self.0.borrow();
        match &inner {
            Some(ComputedValue::Own{value,observers}) => {
                observers.borrow_mut().push(new);
                Left(value.borrow().clone())
            }
            Some(ComputedValue::Weak{..}) => panic!(),
            None => panic!(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
// COMPUTED-SIGNAL INTERNAL
///////////////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
enum ComputedValue<I, O> {
    Own {
        value: Rc<RefCell<O>>,
        #[serde(skip)]
        observers: Rc<RefCell<Vec<Box<SignalObserver<O>>>>>,
    },
    Weak {
        value: Weak<RefCell<O>>,
        #[serde(skip)]
        observers: Weak<RefCell<Vec<Box<SignalObserver<O>>>>>,
        #[serde(skip)]
        mapper: Option<Rc<Mapper<I, O>>>,
    },
}

pub struct Mapper<I, O>(Box<Fn(&I)->O>);

impl<I, O> std::fmt::Debug for Mapper<I, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mapper")
    }
}


impl<I, O> SignalObserver<I> for ComputedValue<I, O> {
    fn set_op(&mut self, new: &I) {
        // SETUP
        let mut deps = None;
        match self {
            ComputedValue::Weak{value,observers,mapper} => {
                if let Some(value) = value.upgrade() {
                    if let Some(observers) = observers.upgrade() {
                        if let Some(mapper) = mapper {
                            deps = Some((value.clone(),observers.clone(),mapper.clone()));
                        }
                    }
                }
            }
            ComputedValue::Own{..} => panic!()
        }
        // GO
        match deps {
            Some((value,observers,mapper)) => {
                let new_output = (mapper.0)(new);
                for observer in observers.borrow_mut().iter_mut() {
                    observer.set_op(&new_output);
                }
                value.replace(new_output);
            }
            None => panic!()
        }
    }
}




///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////




