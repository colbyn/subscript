use crate::view::dsl::*;


pub trait Viewable<Msg> {
    fn extend<'a>(self, env: Env<'a, Msg>);
}


