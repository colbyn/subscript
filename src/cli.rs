use std::collections::{HashSet, HashMap};
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

        /// E.g. for setting the base path when using GitHub pages.
        #[structopt(long)]
        base_url: Option<String>,

        /// Output directory.
        #[structopt(long, parse(from_os_str))]
        output: PathBuf,
    },
    Serve {
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

        /// E.g. for setting the base path when using GitHub pages.
        #[structopt(long)]
        base_url: Option<String>,

        /// Output directory.
        #[structopt(long, parse(from_os_str))]
        output: PathBuf,

        /// Output directory.
        /// Currently the server will ignore files that contain the output path.
        #[structopt(long, default_value="3000")]
        port: u16,
    }
}

pub fn compile_file(
    root: &PathBuf,
    out_dir: &PathBuf,
    trim: &Option<PathBuf>,
    base_url: &Option<String>,
    input: PathBuf
) {
    use crate::{data::*, macros, utils};
    let ctx = Context{
        root_dir: root.clone(),
        source: {
            input.strip_prefix(root).unwrap_or(&input).to_owned()
        },
        base_url: base_url.clone(),
        output_dir: out_dir.clone(),
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
    crate::macros::hooks::document(&ctx, &mut html);
    let mut html = html.normalize();
    crate::macros::hooks::finalize_document(&ctx, &mut html);
    std::fs::write(output_file_path, html.to_html_str(0));
}

pub fn compile(
    root: PathBuf,
    inputs: Vec<PathBuf>,
    output: PathBuf,
    trim: Option<PathBuf>,
    base_url: Option<String>,
) {
    println!("[Subscript] Compiled");
    for input in inputs {
        compile_file(&root, &output, &trim, &base_url, input);
    }
}

pub fn serve(
    root: PathBuf,
    inputs: Vec<PathBuf>,
    output: PathBuf,
    trim: Option<PathBuf>,
    base_url: Option<String>,
    port: u16,
) {
    use hotwatch::{Hotwatch, Event};
    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    hotwatch.unwatch(output.clone());
    fn is_output_file(root: &PathBuf, out: &PathBuf, file: &PathBuf) -> bool {
        assert!(file.is_absolute());
        let out = crate::utils::to_abs_path(root, out);
        file.starts_with(out)
    }
    // fn is_source_file(root: &PathBuf, source: &[String], file: &PathBuf) -> bool {
    //     assert!(file.is_absolute());
    //     let out = crate::utils::to_abs_path(root, out);
    //     file.starts_with(out)
    // }
    let abs_inputs = inputs
        .iter()
        .map(|x| (
            crate::utils::to_abs_path(&root, &x),
            x.clone()
        ))
        .collect::<HashMap<_, _>>();
    hotwatch.watch(root.clone(), {
        let output = output.clone();
        let root = root.clone();
        let process = |changed: PathBuf| {
            assert!(changed.is_absolute());
            if !is_output_file(&root, &output, &changed) {
                // Recompile just the source file
                if let Some(file) = abs_inputs.get(&changed) {
                    compile(
                        root.clone(),
                        vec![file.clone()],
                        output.clone(),
                        trim.clone(),
                        base_url.clone(),
                    );
                }
                // Recompile everything
                else {
                    compile(
                        root.clone(),
                        inputs.clone(),
                        output.clone(),
                        trim.clone(),
                        base_url.clone(),
                    );
                }
            }
        };
        move |event: Event| {
            match event {
                Event::Create(path) => {
                    if !is_output_file(&root, &output, &path) {
                        compile(
                            root.clone(),
                            inputs.clone(),
                            output.clone(),
                            trim.clone(),
                            base_url.clone(),
                        );
                    }
                }
                Event::Write(path) => {
                    if !is_output_file(&root, &output, &path) {
                        compile(
                            root.clone(),
                            inputs.clone(),
                            output.clone(),
                            trim.clone(),
                            base_url.clone(),
                        );
                    }
                }
                Event::Remove(path) => {
                    if !is_output_file(&root, &output, &path) {
                        compile(
                            root.clone(),
                            inputs.clone(),
                            output.clone(),
                            trim.clone(),
                            base_url.clone(),
                        );
                    }
                }
                Event::Rename(from, to) => {
                    if !is_output_file(&root, &output, &to) {
                        compile(
                            root.clone(),
                            inputs.clone(),
                            output.clone(),
                            trim.clone(),
                            base_url.clone(),
                        );
                    }
                }
                Event::NoticeWrite(_) => {}
                Event::NoticeRemove(_) => {}
                Event::Chmod(_) => {}
                Event::Rescan => {}
                Event::Error(error, path) => {}
            };
        }
    }).expect("failed to watch file!");
    crate::server::run_server(
        crate::server::Args{
            address: String::from("127.0.0.1"),
            port,
            cache: 0,
            cors: false,
            compress: false,
            path: output.clone(),
            all: true,
            ignore: false,
            follow_links: true,
            render_index: true,
            log: false,
            path_prefix: None,
        }
    );
}

pub fn run() {
    fn process_inputs(input: Vec<String>) -> Vec<PathBuf> {
        input
            .into_iter()
            .flat_map(|x: String| -> Vec<PathBuf> {
                glob::glob(&x).unwrap()
                    .filter_map(Result::ok)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
    match Opt::from_args() {
        Opt::Compile{root, input, output, trim, base_url} => {
            compile(root, process_inputs(input), output, trim, base_url);
        }
        Opt::Serve{root, input, output, trim, base_url, port} => {
            let inputs = process_inputs(input);
            compile(
                root.clone(),
                inputs.clone(),
                output.clone(),
                trim.clone(),
                base_url.clone(),
            );
            serve(
                root,
                inputs,
                output,
                trim,
                base_url,
                port
            );
        }
    }
}

