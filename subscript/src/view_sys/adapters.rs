use std::rc::*;

use crate::signals_sys::*;
use crate::view_sys::dsl::*;
use crate::view_sys::shared::*;


///////////////////////////////////////////////////////////////////////////////
// Viewable
///////////////////////////////////////////////////////////////////////////////

/// Standalone viewable components.
pub trait Viewable<Msg> {
    fn view(&self) -> View<Msg>;
}


///////////////////////////////////////////////////////////////////////////////
// MIXABLE
///////////////////////////////////////////////////////////////////////////////

pub fn extend_env_with_mixable<'a, Msg>(env: &mut ViewEnv<'a, Msg>, value: impl Mixable<Msg>) {
    value.extend(ViewEnv {
        styling: &mut env.styling,
        attributes: &mut env.attributes,
        events: &mut env.events,
        children: &mut env.children,
    });
}

pub trait Mixable<Msg> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>);
}

impl<Msg: 'static> Mixable<Msg> for &str {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(View::new_text(self));
    }
}
impl<Msg: 'static> Mixable<Msg> for String {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(View::new_text(&self));
    }
}
impl<Msg: 'static> Mixable<Msg> for View<Msg> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(self);
    }
}


impl<Msg: 'static> Mixable<Msg> for &Signal<String> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        let node = View::new_text_signal(self);
        env.children.push(node);
    }
}
impl<Msg: 'static, T: 'static> Mixable<Msg> for &VecSignal<T> where T: Viewable<Msg> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(View::new_linked_control(self, |item| item.view()));
    }
}


