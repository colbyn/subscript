use core::fmt::Debug;
use std::collections::*;
use std::any::*;
use std::marker::*;
use std::cell::*;
use std::rc::*;
use either::{Either, Either::*};

use crate::backend::browser;
use crate::view_sys::{dsl::View, dom::Dom};
use crate::reactive_sys::vec::{VecSignal, VecObserver};


///////////////////////////////////////////////////////////////////////////////
// EXTERNAL INTERFACE
///////////////////////////////////////////////////////////////////////////////

pub(crate) struct ViewVecObserver<Msg>(Rc<ViewVecObserverApi<Msg>>);

impl<Msg> Clone for ViewVecObserver<Msg> {
    fn clone(&self) -> Self {
        ViewVecObserver(self.0.clone())
    }
}

impl<Msg> std::fmt::Debug for ViewVecObserver<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ViewVecObserver")
    }
}


trait ViewVecObserverApi<Msg> {
    // TRAVERSAL UTILS
    fn for_each_dom_node(&self, f: &mut FnMut(&Dom<Msg>));
    // CORE
    fn build(&self, f: &Fn(Vec<View<Msg>>)->Vec<Dom<Msg>>);
    fn tick(&self, args: TickArgs<Msg>);
    fn terminate(&self) -> Either<ViewSegment<Msg>, DomSegment<Msg>>;
}

pub struct TickArgs<'a, Msg> {
    pub(crate) update: &'a mut FnMut(&mut Vec<ViewItem<Msg>>),
    pub(crate) removed: &'a mut FnMut(Dom<Msg>),
}

impl<Msg: 'static> ViewVecObserver<Msg> {
    pub(crate) fn new<T: 'static>(vec_signal: &VecSignal<T>, new: impl Fn(&T) -> View<Msg> + 'static) -> Self {
        // SETUP
        let view_segment: Vec<View<Msg>> = vec_signal.value
            .iter()
            .map(|x| new(x))
            .collect::<Vec<View<Msg>>>();
        let state: Rc<RefCell<ObserverState<Msg>>> =
            Rc::new(RefCell::new(ObserverState::Value(SegmentValue::View(ViewSegment(view_segment)))));
        // VEC SIDE
        let vec_entry = ViewVecSignal::VecSide {
            init: ToView(Box::new(new)),
            state: Rc::downgrade(&state),
        };
        vec_signal.add_observer(vec_entry);
        // VIEW SIDE
        let view_entry: ViewVecSignal<T, Msg> = ViewVecSignal::ViewSide {
            state,
        };
        ViewVecObserver(Rc::new(view_entry))
    }
    // TRAVERSAL UTILS
    pub(crate) fn for_each_dom_node(&self, f: &mut FnMut(&Dom<Msg>)) {
        self.0.for_each_dom_node(f);
    }
    // OPERATIONS
    pub(crate) fn build(&self, f: &Fn(Vec<View<Msg>>)->Vec<Dom<Msg>>) {
        self.0.build(f);
    }
    pub(crate) fn tick(&mut self, args: TickArgs<Msg>) {
        self.0.tick(args);
    }
}

impl<Msg> ViewVecObserver<Msg> {
    pub(crate) fn terminate(self) -> Either<ViewSegment<Msg>, DomSegment<Msg>> {
        self.0.terminate()
    }
}

///////////////////////////////////////////////////////////////////////////////
// INTERNAL INTERFACE
///////////////////////////////////////////////////////////////////////////////

enum ViewVecSignal<T, Msg> {
    VecSide {
        init: ToView<T, Msg>,
        state: Weak<RefCell<ObserverState<Msg>>>,
    },
    ViewSide {
        state: Rc<RefCell<ObserverState<Msg>>>,
    },
}

impl<T, Msg> VecObserver<T> for ViewVecSignal<T, Msg> {
    fn push_op(&mut self, new: &T) {
        self.ensure_from_vec_op();
        self.for_each_segment(ForEachSegment {
            if_view: &mut |data: &mut Vec<View<Msg>>| {
                data.push(self.to_view(new));
            },
            if_dom: &mut |data: &mut DomSegment<Msg>| {
                data.active.push(ViewItem::View(self.to_view(new)));
            },
        });
    }
    fn insert_op(&mut self, ix: usize, new: &T) {
        self.ensure_from_vec_op();
        self.for_each_segment(ForEachSegment {
            if_view: &mut |data: &mut Vec<View<Msg>>| {
                data.insert(ix, self.to_view(new));
            },
            if_dom: &mut |data: &mut DomSegment<Msg>| {
                data.active.insert(ix, ViewItem::View(self.to_view(new)));
            },
        });
    }
    fn remove_op(&mut self, ix: usize) {
        self.ensure_from_vec_op();
        self.for_each_segment(ForEachSegment {
            if_view: &mut |data: &mut Vec<View<Msg>>| {
                data.remove(ix);
            },
            if_dom: &mut |data: &mut DomSegment<Msg>| {
                match data.active.remove(ix) {
                    ViewItem::Dom(dom) => {
                        data.removed.push(dom);
                    }
                    ViewItem::View(_) => {}
                }
            },
        });
    }
}

impl<T, Msg> ViewVecObserverApi<Msg> for ViewVecSignal<T, Msg> {
    // TRAVERSAL UTILS
    fn for_each_dom_node(&self, f: &mut FnMut(&Dom<Msg>)) {
        self.for_each_segment(ForEachSegment {
            if_view: &mut |data: &mut Vec<View<Msg>>| {},
            if_dom: &mut |data: &mut DomSegment<Msg>| {
                for node in data.active.iter() {
                    match node {
                        ViewItem::Dom(x) => {
                            f(x);
                        }
                        _ => {}
                    }
                }
            },
        });
    }
    // CORE
    fn build(&self, f: &Fn(Vec<View<Msg>>)->Vec<Dom<Msg>>) {
        self.map_segment(&mut |segment| {
            match segment {
                SegmentValue::View(view) => {
                    let active = f(view.0);
                    let active = active
                        .into_iter()
                        .map(|x| ViewItem::Dom(x))
                        .collect::<Vec<_>>();
                    SegmentValue::Dom(DomSegment {
                        active,
                        removed: Vec::new(),
                    })
                }
                SegmentValue::Dom(dom) => panic!()
            }
        });
    }
    fn tick(&self, args: TickArgs<Msg>) {
        self.map_segment(&mut |segment| {
            match segment {
                SegmentValue::Dom(mut dom) => {
                    for node in dom.removed.drain(..) {
                        (args.removed)(node);
                    }
                    (args.update)(&mut dom.active);
                    SegmentValue::Dom(dom)
                }
                SegmentValue::View(view) => panic!()
            }
        });
    }
    fn terminate(&self) -> Either<ViewSegment<Msg>, DomSegment<Msg>> {
        match self {
            ViewVecSignal::ViewSide{state} => {
                match state.replace(ObserverState::Terminated) {
                    ObserverState::Value(SegmentValue::Dom(dom)) => Right(dom),
                    ObserverState::Value(SegmentValue::View(view)) => Left(view),
                    ObserverState::Terminated => panic!("already terminated"),
                    ObserverState::Borrowed => panic!(),
                }
            }
            ViewVecSignal::VecSide{..} => {
                panic!("Only the view/dom should terminate the view-segment")
            }
        }
    }
}

impl<T, Msg> ViewVecSignal<T, Msg> {
    pub(crate) fn ensure_from_vec_op(&self){
        let terminate = || {
            let x1 = "INTERNAL ERROR - IMPOSSIBLE!";
            let x2 = "Obviously this should not be called from the view side.";
            panic!([x1, x2].join(" "));
        };
        match self {
            ViewVecSignal::VecSide{..} => {()}
            ViewVecSignal::ViewSide{..} => {terminate()}
        }
    }
    pub(crate) fn to_view(&self, new: &T) -> View<Msg> {
        match self {
            ViewVecSignal::VecSide{init, ..} => {
                (init.0)(new)
            }
            ViewVecSignal::ViewSide{..} => {panic!()}
        }
    }
    pub(crate) fn for_each_segment(&self, f: ForEachSegment<Msg>) {
        self.map_segment(&mut |value| {
            match value {
                SegmentValue::Dom(mut dom) => {
                    (f.if_dom)(&mut dom);
                    SegmentValue::Dom(dom)
                }
                SegmentValue::View(mut view) => {
                    (f.if_view)(&mut view.0);
                    SegmentValue::View(view)
                }
            }
        });
    }
    pub(crate) fn map_segment(&self, f: &mut FnMut(SegmentValue<Msg>)->SegmentValue<Msg>) {
        let mut go = |panic_on_terminated: bool, state: &RefCell<ObserverState<Msg>>| {
            match state.replace(ObserverState::Borrowed) {
                ObserverState::Value(value) => {
                    let new_value = f(value);
                    state.replace(ObserverState::Value(new_value));
                }
                ObserverState::Terminated => {
                    if panic_on_terminated {
                        panic!()
                    }
                }
                ObserverState::Borrowed  => panic!(),
            }
        };
        match self {
            ViewVecSignal::ViewSide{state} => {
                go(true, state)
            }
            ViewVecSignal::VecSide{init, state} => {
                if let Some(state) = state.upgrade() {
                    go(false, &state);
                }
            }
        }
    }
}

pub(crate) struct ForEachSegment<'a, Msg> {
    if_view: &'a mut FnMut(&mut Vec<View<Msg>>),
    if_dom: &'a mut FnMut(&mut DomSegment<Msg>),
}


///////////////////////////////////////////////////////////////////////////////
// STATE
///////////////////////////////////////////////////////////////////////////////

enum ObserverState<Msg> {
    Borrowed,
    Terminated,
    Value(SegmentValue<Msg>),
}

enum SegmentValue<Msg> {
    View(ViewSegment<Msg>),
    Dom(DomSegment<Msg>),
}

pub(crate) struct ViewSegment<Msg>(pub Vec<View<Msg>>);

pub(crate) struct DomSegment<Msg> {
    pub active: Vec<ViewItem<Msg>>,
    pub removed: Vec<Dom<Msg>>,
}

pub(crate) enum ViewItem<Msg> {
    View(View<Msg>),
    Dom(Dom<Msg>),
}


///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub(crate) struct ToView<T, Msg>(Box<Fn(&T)->View<Msg>>);

impl<T, Msg> std::fmt::Debug for ToView<T, Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ToView")
    }
}
