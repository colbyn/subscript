use std::path::PathBuf;
use structopt::StructOpt;


/// The Subscript CLI frontend. 
#[derive(Debug, StructOpt)]
enum Opt {
    /// Compile the given HTML files.
    Compile {
        /// Root directory.
        #[structopt(long, parse(from_os_str), default_value=".")]
        root: PathBuf,

        /// Input files (with glob support).
        #[structopt(long)]
        input: Vec<String>,

        /// Trim from output path
        /// 
        /// For example, to remove 'pages' from './output/pages/index.html',
        /// use '--trim pages'.
        /// 
        /// Take care to ensure this won't clash with anything.
        #[structopt(long, parse(from_os_str))]
        trim: Option<PathBuf>,

        /// Output directory.
        #[structopt(long, parse(from_os_str))]
        output: PathBuf,
    }
}

pub fn compile_file(root: &PathBuf, out_dir: &PathBuf, trim: &Option<PathBuf>, input: PathBuf) {
    use crate::{data::*, macros, utils};
    let ctx = Context{
        root_dir: root.clone(),
        source: {
            input.strip_prefix(root).unwrap_or(&input).to_owned()
        },
    };
    let input_path_str = input.to_str().unwrap();
    let output_file_path = {
        let mut res = input
            .strip_prefix(&root)
            .unwrap_or(&input);
        if let Some(trim) = trim {
            res = res.strip_prefix(trim).unwrap_or(res);
        }
        out_dir.join(res)
    };
    let output_dir = output_file_path.parent().unwrap();
    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir);
    }
    let source = utils::read_file_or_panic(input);
    let mut html = Node::parse_str(&source);
    html.apply(macros::include_tag(&ctx));
    html.apply(macros::items_tag(&ctx));
    html.apply(macros::latex_suit(&ctx));
    html.apply(macros::note_tag(&ctx));
    let mut html = html.normalize();
    std::fs::write(output_file_path, html.to_html_str(0));
}

pub fn run() {
    match Opt::from_args() {
        Opt::Compile{root, input, output, trim} => {
            let xs = input
                .into_iter()
                .flat_map(|x: String| -> Vec<PathBuf> {
                    glob::glob(&x).unwrap()
                        .filter_map(Result::ok)
                        .collect::<Vec<_>>()
                })
                .map(|input: PathBuf| {
                    compile_file(&root, &output, &trim, input);
                })
                .collect::<Vec<_>>();
        }
    }
}

