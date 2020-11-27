use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::borrow::Cow;
use std::collections::HashSet;
use std::path::{PathBuf, Path};
use std::convert::AsRef;
use std::hash::Hash;

use crate::parser;
use crate::macros;


///////////////////////////////////////////////////////////////////////////////
// HELPERS
///////////////////////////////////////////////////////////////////////////////

pub enum Either<L, R> {
    Left(L),
    Right(R),
}


#[derive(Clone)]
struct MacroCallback(MacroFunction<Result<(), ()>>);

impl std::fmt::Debug for MacroCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MacroCallback").finish()
    }
}

/// `Ret` is the **return type**.
pub type MacroFunction<Ret> = Rc<dyn Fn(&mut Node) -> Ret>;


///////////////////////////////////////////////////////////////////////////////
// MACRO DATA TYPES
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Macro {
    name: String,
    callback: MacroCallback,
}

impl Macro {
    pub fn new(name: &str, callback: MacroFunction<Option<Result<(), ()>>>) -> Self {
        Macro{
            name: name.to_owned(),
            callback: MacroCallback(Rc::new(move |x| {
                match callback(x) {
                    Some(x) => x,
                    _ => Ok(())
                }
            })),
        }
    }
    pub fn new_void(name: &str, callback: MacroFunction<Option<()>>) -> Self {
        Macro{
            name: name.to_owned(),
            callback: MacroCallback(Rc::new(move |x| {
                let res = callback(x);
                Ok(())
            })),
        }
    }
    pub fn match_tag(tag: &str, callback: MacroFunction<()>) -> Self {
        Macro::new_void(tag, {
            let tag = String::from(tag);
            Rc::new(move |node: &mut Node| -> Option<()> {
                if tag == node.tag()? {
                    callback(node);
                }
                Some(())
            })
        })
    }
    pub fn eval(&self, node: &mut Node) {
        match self.callback.0(node) {
            Err(_) => {
                eprintln!("macro <{}> failed", self.name);
                panic!()
            }
            _ => (),
        }
    }
}



///////////////////////////////////////////////////////////////////////////////
// HTML TREE AST
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Element(Box<Element>),
    Text(String),
    Fragment(Vec<Node>),
}

impl Node {
    pub fn to_html_str(&self, indent_level: usize) -> String {
        let level = {
            if indent_level == 0 {
                String::from("")
            } else {
                std::iter::repeat(" ").take(indent_level * 2).collect::<String>()
            }
        };
        match self {
            Node::Element(element) => {
                // COMMON
                let attrs = element.attrs
                    .iter()
                    .map(|(key, value)| {
                        if value.is_empty() {
                            format!("{}", key)
                        } else {
                            format!("{}=\"{}\"", key, value)
                        }
                    })
                    .collect::<Vec<_>>();
                let attrs = attrs.join(" ");
                let attrs = {
                    if !element.attrs.is_empty() {
                        format!(" {}", attrs)
                    } else {
                        String::new()
                    }
                };
                // AD-HOC IMPLIMENTATIONS
                let single_tag = |tag: &str| {
                    let attrs = attrs.clone();
                    format!(
                        "{lvl}<{tag} {attrs} >\n",
                        lvl=level,
                        tag=tag,
                        attrs=attrs
                    )
                };
                let no_children = element.children.is_empty();
                if element.tag == String::from("img") && no_children {
                    return single_tag("img");
                }
                if element.tag == String::from("meta") && no_children {
                    return single_tag("meta");
                }
                if element.tag == String::from("link") && no_children {
                    return single_tag("link");
                }
                // GENERAL
                let children = element.children
                    .iter()
                    .map(|child| {
                        child.to_html_str(indent_level + 1)
                    })
                    .collect::<Vec<_>>();
                let children = children.join("");
                if element.children.len() == 0 {
                    format!(
                        "\n{lvl}<{tag}{attrs}></{tag}>\n",
                        lvl=level,
                        tag=element.tag,
                        attrs=attrs,
                    )
                } else if self.is_inline_node() {
                    format!(
                        "<{tag}{attrs}>{children}</{tag}>",
                        tag=element.tag,
                        attrs=attrs,
                        children=children
                    )
                } else if self.only_inline_children() {
                    format!(
                        "{lvl}<{tag}{attrs}>{children}</{tag}>\n",
                        lvl=level,
                        tag=element.tag,
                        attrs=attrs,
                        children=children
                    )
                } else {
                    format!(
                        "{lvl}<{tag}{attrs}>\n{children}{lvl}</{tag}>\n",
                        lvl=level,
                        tag=element.tag,
                        attrs=attrs,
                        children=children
                    )
                }
            }
            Node::Text(text) => {
                text.clone()
            }
            Node::Fragment(xs) => {
                let children = xs
                    .iter()
                    .map(|child| {
                        child.to_html_str(indent_level)
                    })
                    .collect::<Vec<_>>();
                children.join("\n")
            }
        }
    }
    pub fn into_fragment(self) -> Vec<Node> {
        match self {
            Node::Fragment(xs) => {xs}
            _ => vec![]
        }
    }
    pub fn only_text_children(&self) -> bool {
        self.get_children()
            .into_iter()
            .all(|x| {
                x.is_text()
            })
    }
    pub fn only_inline_children(&self) -> bool {
        self.get_children()
            .into_iter()
            .all(|x| {
                x.is_inline_node()
            })
    }
    pub fn parse_str(html_str: &str) -> Self {
        Node::Fragment(crate::parser::parse_html_str(html_str).payload)
    }
    pub fn eval(&mut self, f: Rc<dyn Fn(&mut Node)>) {
        match self {
            Node::Element(element) => {
                for child in element.children.iter_mut() {
                    child.eval(f.clone());
                }
            }
            Node::Fragment(xs) => {
                for x in xs.iter_mut() {
                    x.eval(f.clone());
                }
            }
            _ => {}
        }
        f(self);
    }
    pub fn apply(&mut self, f: Macro) {
        self.eval(Rc::new(move |x| {
            f.eval(x)
        }))
    }
    pub fn apply_all(&mut self, macros: Vec<Macro>) {
        for mut m in macros {
            self.apply(m);
        }
    }
    pub fn tag(&self) -> Option<String> {
        match self {
            Node::Element(element) => Some(element.tag.clone()),
            _ => None
        }
    }
    pub fn is_tag(&self, tag: &str) -> bool {
        self.tag() == Some(String::from(tag))
    }
    pub fn has_attr(&self, key: &str) -> bool {
        match self {
            Node::Element(element) => {
                element.attrs.contains_key(key)
            },
            _ => false
        }
    }
    pub fn has_attr_value(&self, key: &str, value: &str) -> bool {
        match self {
            Node::Element(element) => {
                element.attrs.get(key).map(|x| x == value).unwrap_or(false)
            },
            _ => false
        }
    }
    pub fn get_attr(&self, key: &str) -> Option<String> {
        match self {
            Node::Element(element) => {
                if let Some(key) = element.attrs.get(key) {
                    Some(key.clone())
                } else {
                    None
                }
            },
            _ => None
        }
    }
    pub fn set_attr(&mut self, key: &str, value: String) {
        match self {
            Node::Element(element) => {
                element.attrs.insert(key.to_owned(), value);
            },
            _ => ()
        }
    }
    pub fn replace_children(&mut self, new_children: Vec<Node>) {
        match self {
            Node::Element(element) => {
                element.children = new_children;
            },
            _ => ()
        }
    }
    pub fn append_children(&mut self, mut new_children: Vec<Node>) {
        match self {
            Node::Element(element) => {
                element.children.append(&mut new_children);
            },
            _ => ()
        }
    }
    pub fn get_children(&self) -> Vec<Node> {
        match self {
            Node::Element(element) => {
                element.children.clone()
            },
            _ => vec![]
        }
    }
    pub fn normalize(self) -> Self {
        match self {
            Node::Element(mut element) => {
                let mut new_children = Vec::<Node>::new();
                for child in element.children.into_iter() {
                    match child {
                        Node::Fragment(mut xs) => {
                            for x in xs {
                                new_children.push(x.normalize())
                            }
                        }
                        node => {
                            new_children.push(node.normalize())
                        }
                    }
                }
                element.children = new_children;
                Node::Element(element)
            }
            Node::Fragment(elements) => {
                let mut new_children = Vec::<Node>::new();
                for child in elements.into_iter() {
                    match child {
                        Node::Fragment(mut xs) => {
                            for x in xs {
                                new_children.push(x.normalize())
                            }
                        }
                        node => {
                            new_children.push(node.normalize())
                        }
                    }
                }
                Node::Fragment(new_children)
            }
            node => node
        }
    }
    pub fn get_children_as_text(&self) -> Vec<String> {
        let mut texts = Vec::<String>::new();
        match self {
            Node::Text(text) => vec![text.clone()],
            _ => {
                let mut ys = self
                    .get_children()
                    .into_iter()
                    .flat_map(|x| x.get_children_as_text())
                    .collect::<Vec<_>>();
                return ys;
            }
        }
    }
    pub fn get_text_contents(&self) -> Option<String> {
        let txts = self.get_children_as_text();
        if txts.is_empty() {
            None
        } else {
            Some(txts.join(" "))
        }
    }
    pub fn is_text(&self) -> bool {
        match self {
            Node::Text(_) => true,
            _ => false,
        }
    }
    pub fn is_element(&self) -> bool {
        match self {
            Node::Element(_) => true,
            _ => false,
        }
    }
    pub fn new_element(
        tag: &str,
        mut attrs: HashMap<String, String>,
        children: &[Node],
    ) -> Self {
        let mut element = Element{
            tag: String::from(tag),
            styling: Styling::default(),
            attrs,
            children: children.to_owned(),
        };
        macros::hooks::new_element(&mut element);
        Node::Element(Box::new(element))
    }
    pub fn new_text(value: &str) -> Self {
        Node::Text(String::from(value))
    }
    pub fn is_inline_node(&self) -> bool {
        if self.get_attr("block").is_some() {
            return false;
        }
        match self {
            Node::Element(element) => {
                if crate::utils::is_inline_tag(&element.tag) {
                    return true;
                }
                if element.tag == String::from("tex") {
                    return true;
                }
                false
            },
            Node::Fragment(xs) => {
                xs.iter().all(|x| x.is_inline_node())
            }
            Node::Text(..) => true,
        }
    }
}

impl Default for Node {
    fn default() -> Self {Node::Fragment(vec![])}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub tag: String,
    pub styling: Styling,
    pub attrs: HashMap<String, String>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Styling {

}

///////////////////////////////////////////////////////////////////////////////
// COMPILER MEAT
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Context {
    pub source: FilePath,
    pub root_dir: FilePath,
    pub output_dir: FilePath,
    pub base_url: Option<String>,
    /// When in server/watch mode, we don't want to process e.g. images
    /// for every file change.
    pub fast_upate_mode: bool,
    pub changed_file: Option<FilePath>,
}

impl Context {
    pub fn new<P: AsRef<Path>>(
        root_dir: P,
        output_dir: P,
        source: P,
    ) -> Self {
        Context {
            source: FilePath::new(source.as_ref()).unwrap(),
            root_dir: FilePath::new(root_dir.as_ref()).unwrap(),
            output_dir: FilePath::new(output_dir.as_ref()).unwrap(),
            base_url: None,
            fast_upate_mode: false,
            changed_file: None,
        }
    }
    pub fn source_dir(&self) -> FilePath {
        let path = self.source.0.parent().unwrap().to_owned();
        FilePath(path)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FilePath(PathBuf);

impl std::fmt::Debug for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(path) = self.0.to_str() {
            let formatted = format!("FilePath(\"{}\")", path);
            f.debug_struct(&formatted).finish()
        } else {
            f.debug_struct("FilePath").finish()
        }
    }
}
impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_str().unwrap())
    }
}


/// We need to be careful where this is used, since this is used in places where URLs may occur.
impl FilePath {
    /// All paths in Subsystem are resolved
    /// into absolute path files (to keep things consistent). 
    pub fn new<P: AsRef<Path>>(path: P) -> Option<Self> {
        let path = path.as_ref().to_owned();
        let is_http_path = || {
            let path = path.to_str().unwrap();
            path.starts_with("http")
        };
        if path.is_absolute() {
            Some(FilePath(path))
        } else if is_http_path() {
            None
        } else {
            let pwd = std::env::current_dir().unwrap();
            Some(FilePath(pwd.join(path)))
        }
    }
    /// All paths in Subsystem are resolved
    /// into absolute path files (to keep things consistent). 
    /// This is relative to the root dir.
    pub fn resolve_child_path<P: AsRef<Path>>(
        parent: P,
        path: P,
    ) -> Option<Self> {
        if path.as_ref().is_absolute() {
            FilePath::new(path)
        } else {
            FilePath::new(parent.as_ref().join(path))
        }
    }
    /// This is relative to the source dir.
    /// This is used for e.g. `<include>` paths that are relative to
    /// the source file.
    pub fn resolve_include_path<P: AsRef<Path>>(
        ctx: &Context,
        path: P,
    ) -> Option<Self> {
        let source_dir = ctx.source_dir();
        FilePath::resolve_child_path(
            &ctx.root_dir,
            &source_dir.join(&ctx.root_dir, path.as_ref())?
        )
    }
    pub fn is_child_path(&self, parent_path: &FilePath) -> bool {
        self.0.starts_with(parent_path)
    }
    pub fn join<B: AsRef<Path>, P: AsRef<Path>>(&self, base_path: B, sub_path: P) -> Option<FilePath> {
        if sub_path.as_ref().is_relative() {
            return FilePath::new(self.0.join(sub_path));
        }
        FilePath::new(self.0.join(sub_path))
    }
    pub fn to_str(&self) -> &str {
        self.0.to_str().unwrap()
    }
    pub fn to_path_buffer(self) -> PathBuf {
        self.0
    }
    pub fn load_text_file(&self) -> String {
        if !self.0.exists() {
            eprintln!("missing file {:?}", self.to_str());
            panic!()
        }
        let contents = std::fs::read(&self).unwrap();
        String::from_utf8(contents).unwrap()
    }
    pub fn load_binary_file(&self) -> Vec<u8> {
        match self.try_load_binary_file() {
            Ok(x) => x,
            Err(_) => {
                eprintln!("missing file {:?}", self.to_str());
                panic!()
            }
        }
    }
    pub fn try_load_binary_file(&self) -> Result<Vec<u8>, ()> {
        match std::fs::read(&self) {
            Ok(x) => Ok(x),
            Err(_) => Err(())
        }
    }
    pub fn parent(&self) -> FilePath {
        FilePath::new(self.0.parent().unwrap()).unwrap()
    }
    /// For source file paths.
    /// Ensure `trim` is a PathBuf because it is a path fragment.
    pub fn to_output_path(
        &self,
        ctx: &Context,
        trim: &Option<PathBuf>,
    ) -> FilePath {
        let mut relative_path: PathBuf = {
            self.strip_prefix(&ctx.root_dir).unwrap()
        };
        if let Some(trim) = trim {
            relative_path = relative_path
                .strip_prefix(trim)
                .map(|x| x.to_owned())
                .unwrap_or(relative_path);
        }
        let output_path = FilePath::new(ctx.output_dir.0.join(relative_path)).unwrap();
        output_path
    }
    pub fn exists(&self) -> bool {
        self.0.exists()
    }
    pub fn extension(&self) -> Option<String> {
        self.0
            .extension()
            .map(|x| x.to_str().unwrap().to_owned())
    }
    pub fn strip_prefix<P: AsRef<Path>>(&self, prefix: P) -> Result<PathBuf, ()> {
        self.0.strip_prefix(prefix)
            .map(|x| x.to_owned())
            .map_err(|_| ())
    }
}

impl AsRef<Path> for FilePath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}
impl AsRef<FilePath> for FilePath {
    fn as_ref(&self) -> &FilePath {
        self
    }
}

///////////////////////////////////////////////////////////////////////////////
// CACHE
///////////////////////////////////////////////////////////////////////////////

pub type SourcePath = FilePath;
pub type OutputPath = FilePath;

pub struct Cache(Arc<Mutex<HashMap<SourcePath, CachedFile>>>);

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref GLOBAL_CACHE: Cache = Cache::new();
}

pub fn cache(ctx: &Context, source_path: &FilePath) -> String {
    GLOBAL_CACHE.cache(ctx, source_path)
}

#[derive(Debug, Clone)]
pub struct CachedFile {
    output: String,
}


impl Cache {
    fn new() -> Self {
        Cache(Arc::new(Mutex::new(HashMap::default())))
    }
    fn lookup(&self, path: &FilePath) -> Option<CachedFile> {
        self.0.lock().unwrap().get(path).map(|x| x.clone())
    }
    fn insert(&self, source_path: &FilePath, cached_file: CachedFile) {
        self.0.lock().unwrap().insert(source_path.clone(), cached_file);
    }
    fn cache(&self, ctx: &Context, source_path: &FilePath) -> String {
        if let Some(cached) = self.lookup(source_path) {
            return cached.output
        }
        let out_path = crate::utils::cache_file_dep(ctx, source_path);
        let cached_file = CachedFile {
            output: out_path.clone(),
        };
        self.insert(source_path, cached_file);
        out_path
    }
}


