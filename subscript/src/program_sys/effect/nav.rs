use std::any::*;
use std::rc::*;
use std::collections::*;

use crate::backend::browser;
use crate::reactive_sys::*;

///////////////////////////////////////////////////////////////////////////////
// GENERIC URL INTERFACE
///////////////////////////////////////////////////////////////////////////////

pub trait UrlString {
    fn url_string(&self) -> String;
}

impl UrlString for &str {
    fn url_string(&self) -> String {String::from(*self)}
}
impl UrlString for String {
    fn url_string(&self) -> String {self.clone()}
}



///////////////////////////////////////////////////////////////////////////////
// URL CORE
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq)]
pub struct Url {
    path: String,
    parameters: HashMap<String, String>,
}

impl Url {
    pub(crate) fn get_current(window: &browser::Window) -> Url {
        let path = window.location.pathname();
        Url {
            path,
            parameters: HashMap::default(),
        }
    }
}


pub fn init_binders(this: &Url, pattern: Vec<PathSegment>) -> Vec<PathSegment> {
    let path_segs: Vec<String> = this.path
        .split("/")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x))
        .collect();
    path_segs
        .into_iter()
        .zip(pattern.into_iter())
        .map(|(seg, pat)| {
            match pat {
                PathSegment::Static(x) => {
                    PathSegment::Static(x)
                }
                PathSegment::Wildcard => {
                    PathSegment::Wildcard
                }
                PathSegment::Binder(None) => {
                    PathSegment::Binder(Some(seg.clone()))
                }
                PathSegment::Binder(Some(_)) => panic!()
            }
        })
        .collect::<Vec<_>>()
}
pub fn pattern_matches(this: &Url, pattern: Vec<PathSegment>) -> bool {
    let path_segs: Vec<String> = this.path
        .split("/")
        .filter(|x| !x.is_empty())
        .map(|x| String::from(x))
        .collect();
    let valid_length = path_segs.len() == pattern.len();
    let valid_patterns = path_segs
        .into_iter()
        .zip(pattern.into_iter())
        .all(|(seg, pat)| {
            match pat {
                PathSegment::Static(x) => {
                    seg == x
                }
                PathSegment::Wildcard | PathSegment::Binder(_) => true
            }
        });
    valid_length && valid_patterns
}

///////////////////////////////////////////////////////////////////////////////
// URL EVENTS
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq)]
pub struct UrlChanged(pub(crate) Url);

impl UrlChanged {
    pub fn url(&self) -> Url {
        Url {
            path: self.0.path.clone(),
            parameters: self.0.parameters.clone(),
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// URL PARSER
///////////////////////////////////////////////////////////////////////////////

pub struct UrlParser<T>(pub Rc<Fn(Url)->T>);

impl<T> UrlParser<T> {
    pub fn parse(&self, url: &Url) -> T {
        let url = Url {
            path: url.path.clone(),
            parameters: url.parameters.clone(),
        };
        (self.0)(url)
    }
}

impl<T> Clone for UrlParser<T> {
    fn clone(&self) -> Self {
        UrlParser(self.0.clone())
    }
}

#[derive(PartialEq, Clone)]
pub enum PathSegment {
    Static(String),
    Wildcard,
    Binder(Option<String>)
}

impl PathSegment {
    pub(crate) fn is_static(&self) -> bool {
        match self {
            PathSegment::Static(_) => true,
            _ => false
        }
    }
    pub(crate) fn is_wildcard(&self) -> bool {
        match self {
            PathSegment::Wildcard => true,
            _ => false
        }
    }
    pub(crate) fn is_binder(&self) -> bool {
        match self {
            PathSegment::Binder(_) => true,
            _ => false
        }
    }
    pub(crate) fn unsafe_get_binder_str(&self) -> String {
        match self {
            PathSegment::Binder(Some(x)) => x.clone(),
            _ => panic!()
        }
    }
}

pub enum UrlPattern {
    AlwaysMatch,
    MatchPath(Vec<PathSegment>)
}

#[macro_export]
macro_rules! try_parse_binders  {
    ($path_data:expr; $result:expr;) => {{
        // DONE
        $result
    }};
    ($path_data:expr; $result:expr; _) => {{
        // DONE
        $result
    }};
    ($path_data:expr; $result:expr; $name:ident : $type:ty) => {{
        // PARSE
        let value: String = $path_data.remove(0).unsafe_get_binder_str();
        let parsed: Option<$type> = std::str::FromStr::from_str(value.as_str()).ok();
        // DONE
        if let Some($name) = parsed {
            $result
        }
    }};
    ($path_data:expr; $result:expr; $static:expr) => {{
        // SANITY CHECK
        let static_value = $path_data.remove(0);
        assert!(static_value == PathSegment::Static(String::from($static)));
        // DONE
        $result
    }};
    ($path_data:expr; $result:expr; _, $($rest:tt)*) => {{
        // NEXT
        try_parse_binders!($path_data; $result; $($rest)*);
    }};
    ($path_data:expr; $result:expr; $name:ident : $type:ty, $($rest:tt)*) => {{
        // PARSE
        let value: String = $path_data.remove(0).unsafe_get_binder_str();
        let parsed: Option<$type> = std::str::FromStr::from_str(value.as_str()).ok();
        // DONE
        if let Some($name) = parsed {
            // NEXT
            try_parse_binders!($path_data; $result; $($rest)*);
        }
    }};
    ($path_data:expr; $result:expr; $static:expr, $($rest:tt)*) => {{
        // SANITY CHECK
        let static_value = $path_data.remove(0);
        assert!(static_value == PathSegment::Static(String::from($static)));
        // NEXT
        try_parse_binders!($path_data; $result; $($rest)*);
    }};
}

#[macro_export]
macro_rules! with_ident_binders {
    ($path_data:expr; _; $result:expr) => {
        $result
    };
    ($path_data:expr; []; $result:expr) => {
        $result
    };
    ($path_data:expr; [$($x:tt)*]; $result:expr) => {{
        try_parse_binders!($path_data; $result; $($x)*)
    }};
}

#[macro_export]
macro_rules! url_path_segments {
    ($xs:expr;) => {};
    ($xs:expr; _) => {{
        $xs.push(PathSegment::Wildcard);
    }};
    ($xs:expr; $name:ident : $type:ty) => {{
        $xs.push(PathSegment::Binder(None));
    }};
    ($xs:expr; $static:expr) => {{
        $xs.push(PathSegment::Static(String::from($static)));
    }};
    ($xs:expr; _, $($rest:tt)*) => {{
        $xs.push(PathSegment::Wildcard);
        url_path_segments!($xs; $($rest)*);
    }};
    ($xs:expr; $name:ident : $type:ty, $($rest:tt)*) => {{
        $xs.push(PathSegment::Binder);
        url_path_segments!($xs; $($rest)*);
    }};
    ($xs:expr; $static:expr, $($rest:tt)*) => {{
        $xs.push(PathSegment::Static(String::from($static)));
        url_path_segments!($xs; $($rest)*);
    }};
}


#[macro_export]
macro_rules! url_pattern {
    (_) => {
        UrlPattern::AlwaysMatch
    };
    ([]) => {
        UrlPattern::MatchPath(Vec::new())
    };
    ([$($x:tt)*]) => {{
        let mut xs = Vec::new();
        url_path_segments!(xs; $($x)*);
        UrlPattern::MatchPath(xs)
    }};
}


#[macro_export]
macro_rules! url_parser_impl {
    ($url_changed:expr; $($pattern:tt => $body:tt)*) => {{
        let mut result = None;
        $(
        {
            let pattern: UrlPattern = url_pattern!($pattern);
            let mut with_path_data: Option<Vec<PathSegment>> = None;
            let matched_path: bool = match pattern {
                UrlPattern::AlwaysMatch => {true}
                UrlPattern::MatchPath(ps) => {
                    with_path_data = Some(init_binders($url_changed, ps.clone()));
                    pattern_matches($url_changed, ps)
                }
            };
            if matched_path {
                if let Some(mut path_data) = with_path_data {
                    with_ident_binders!(path_data; $pattern; {
                        if result.is_none() {
                            result = Some($body);
                        }
                    })
                } else {
                    let mut empty_path_data: Vec<PathSegment> = Vec::new();
                    with_ident_binders!(empty_path_data; $pattern; {
                        if result.is_none() {
                            result = Some($body);
                        }
                    })
                }
            }
        }
        )*
        if let Some(result) = result {
            result
        } else {
            panic!()
        }
    }};
}

#[macro_export]
macro_rules! check_for_totality  {
    () => {
        // BAD
        compile_error!(
            "The url parser must cover all possible cases. \
            E.g. add a `_ => {...}` \
            at the end."
        );
    };
    ([] => {$($xs:tt)*}) => {
        // BAD
        compile_error!(
            "The url parser must cover all possible cases. \
            E.g. add a `_ => {...}` at the end.\n\n\
            FYI your last branch (`[] => {...}`) doesn’t count. Empty brackets (`[]`) represents the root or index path."
        );
    };
    ([$($ps:tt)*] => {$($xs:tt)*}) => {
        // BAD
        compile_error!(
            "The url parser must cover all possible cases. \
            E.g. add a `_ => {...}` \
            at the end."
        );
    };
    (_ => {$($xs:tt)*} $($rest:tt)*) => {
        // GOOD
    };
    (_ => {$($xs:tt)*} $($rest:tt)*) => {
        // GOOD
    };
    ([] => {$($xs:tt)*} $($rest:tt)*) => {
        // CONS
        check_for_totality!($($rest)*);
    };
    ([$($ps:tt)*] => {$($xs:tt)*} $($rest:tt)*) => {
        // CONS
        check_for_totality!($($rest)*);
    };
}

#[macro_export]
macro_rules! url_parser {
    ($($x:tt)*) => {{
        use std::str::FromStr;
        use ::subscript::program_sys::effect::nav::*;

        UrlParser(Rc::new(move |url_changed: Url| {
            check_for_totality!($($x)*);
            url_parser_impl!(&url_changed; $($x)*)
        }))
    }};
}


///////////////////////////////////////////////////////////////////////////////
// DEV
///////////////////////////////////////////////////////////////////////////////
// pub mod dev {
//     use uuid::Uuid;
//     use super::*;

//     pub enum Page {
//         Home,
//         Item(Uuid),
//         Account,
//         NotFound,
//     }


//     pub fn dev() {
//         let parser: UrlParser<Page> = url_parser!{
//             [] => {
//                 Page::Home
//             }
//             ["item", uid: Uuid] => {
//                 Page::Item(uid)
//             }
//             ["account"] => {
//                 Page::Account
//             }
//             _ => {
//                 Page::NotFound
//             }
//         };
//     }
// }

