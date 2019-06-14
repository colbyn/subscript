use core::fmt::Debug;
use std::marker::*;
use std::any::*;
use std::cell::*;
use std::rc::*;
use std::collections::*;

use crate::signals_sys::basic::{Signal, SignalObserver};


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
    pub(crate) fn new(signal: &Signal<T>) -> Self {
        let value = Rc::new(RefCell::new(signal.value.clone()));
        let weak = CellObserver(CellState::Weak{
            value: Rc::downgrade(&value),
        });
        let own = CellObserver(CellState::Own{value});
        signal.add_observer(weak);
        own
    }
    pub(crate) fn get(&self) -> T {
        match self {
            CellObserver(CellState::Own{value}) => {value.borrow().clone()}
            CellObserver(CellState::Weak{..}) => panic!()
        }
    }
}




