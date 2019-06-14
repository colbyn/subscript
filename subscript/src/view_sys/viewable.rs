use crate::view_sys::dsl::*;
use crate::view_sys::shared::*;

pub fn extend_env_with_viewable<'a, Msg>(env: &mut ViewEnv<'a, Msg>, value: impl Viewable<Msg>) {
    value.extend(ViewEnv {
        styling: &mut env.styling,
        attributes: &mut env.attributes,
        events: &mut env.events,
        children: &mut env.children,
    });
}

pub trait Viewable<Msg> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>);
}

impl<Msg: 'static> Viewable<Msg> for &str {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(View::new_text(self));
    }
}
impl<Msg: 'static> Viewable<Msg> for String {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(View::new_text(&self));
    }
}
impl<Msg: 'static> Viewable<Msg> for View<Msg> {
    fn extend<'a>(self, env: ViewEnv<'a, Msg>) {
        env.children.push(self);
    }
}







