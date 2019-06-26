use std::rc::*;

use crate::reactive_sys::*;
use crate::view_sys::dsl::*;
use crate::view_sys::shared::*;
use crate::program_sys::instances::Component;
use crate::program_sys::spec::Spec;

pub use crate::view_sys::dsl::{ViewEnv};


///////////////////////////////////////////////////////////////////////////////
// Viewable
///////////////////////////////////////////////////////////////////////////////

/// Standalone viewable components.
pub trait Viewable<Msg> {
    fn view(&self) -> View<Msg>;
}



///////////////////////////////////////////////////////////////////////////////
// VIEW-EXT
///////////////////////////////////////////////////////////////////////////////

pub fn run_view_extendable<'a, Msg>(env: &mut ViewEnv<'a, Msg>, value: impl ViewExt<Msg>) {
    value.extend(ViewEnv {
        styling: &mut env.styling,
        attributes: &mut env.attributes,
        events: &mut env.events,
        children: &mut env.children,
    });
}

pub trait ViewExt<Msg> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>);
}

impl<Msg: 'static> ViewExt<Msg> for () {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {}
}
impl<Msg: 'static> ViewExt<Msg> for &str {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(View::new_text(self));
    }
}
impl<Msg: 'static> ViewExt<Msg> for &String {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(View::new_text(&self));
    }
}
impl<Msg: 'static> ViewExt<Msg> for String {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(View::new_text(&self));
    }
}
impl<Msg: 'static> ViewExt<Msg> for View<Msg> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(self);
    }
}
impl<Msg: 'static> ViewExt<Msg> for Vec<View<Msg>> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.extend(self);
    }
}
impl<Msg: 'static> ViewExt<Msg> for Signal<String> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        let node = View::new_text_signal(&self);
        env.children.push(node);
    }
}
impl<Msg: 'static> ViewExt<Msg> for Formula<String> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        let node = View::new_text_signal(&self);
        env.children.push(node);
    }
}
impl<Msg: 'static> ViewExt<Msg> for &Signal<String> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        let node = View::new_text_signal(self);
        env.children.push(node);
    }
}
impl<Msg: 'static> ViewExt<Msg> for &Formula<String> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        let node = View::new_text_signal(self);
        env.children.push(node);
    }
}
impl<Msg: 'static, T: 'static> ViewExt<Msg> for &VecSignal<T> where T: Viewable<Msg> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(View::new_linked_control(self, |item| item.view()));
    }
}
impl<Msg: 'static, S: Spec + 'static> ViewExt<Msg> for Component<S> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(View::new_component(self.name.as_str(), self.spec));
    }
}

