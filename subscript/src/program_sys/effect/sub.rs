#[macro_export]
macro_rules! i_subs_entry {
    ($subs:expr; signal $name:ident ($value:expr => $new_value:ident) -> $msg:ty {$($body:tt)*}) => {{
        unimplemented!()
    }};
    ($subs:expr; mail $name:ident (ref $value:ident : $type:ty) -> $msg:ty {$body:expr}) => {{
        $subs.add_mail_sub(move |something: Rc<Any>| -> Option<$msg> {
            let mut result: Option<$msg> = None;
            if let Some($value) = something.downcast_ref::<$type>() {
                result = Some($body);
            }
            result
        });
    }};
}

#[macro_export]
macro_rules! subs {
    () => {{
        use crate::program_sys::spec::*;
        let subs = Subscriptions::default();
        subs
    }};
    ($($kind:ident $fn_name:ident $args:tt -> $msg:ty {$($body:tt)*})*) => {{
        use std::any::{Any, TypeId};
        use std::rc::Rc;
        use crate::program_sys::spec::*;
        let mut subs = Subscriptions::default();
        $({
            i_subs_entry!(subs; $kind $fn_name $args -> $msg {{$($body)*}});
        })*
        subs
    }};
}


///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////

use crate::program_sys::spec::*;

enum Msg {
    NoOp
}

pub struct UrlChanged(String);

pub fn run() {
    let subscriber: Subscriptions<Msg> = subs!{
        signal session_changed(self.session => new_session) -> Msg {}
        mail new_url(ref value: UrlChanged) -> Msg {
            Msg::NoOp
        }
    };
}
