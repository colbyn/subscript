use std::collections::{HashSet, HashMap};
use std::path::PathBuf;
use structopt::StructOpt;
use crate::data::FilePath;


/// The Subscript CLI frontend. 
/// 
/// Notes, 
/// * Subscripts currently assumes the output is nested under the root directory.
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

        /// Automatically open chrome in kiosk mode.
        #[structopt(long)]
        open_browser: bool,
    }
}

pub fn compile_file(
    fast_upate_mode: bool,
    root: &FilePath,
    out_dir: &FilePath,
    trim: &Option<PathBuf>,
    base_url: &Option<String>,
    input: FilePath,
    changed_file: Option<FilePath>,
) {
    use crate::{data::*, macros, utils};
    let mut ctx = Context::new(
        root,
        out_dir,
        &input,
    );
    ctx.base_url = base_url.clone();
    ctx.fast_upate_mode = fast_upate_mode;
    ctx.changed_file = changed_file;
    let output_file_path = input.to_output_path(
        &ctx,
        trim,
    );
    let output_dir = output_file_path.parent();
    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir);
    }
    let source: String = input.load_text_file();
    let mut html = Node::parse_str(&source);
    crate::macros::hooks::document(&ctx, &mut html);
    let mut html = html.normalize();
    crate::macros::hooks::finalize_document(&ctx, &mut html);
    std::fs::write(output_file_path, html.to_html_str(0));
}

pub fn compile(
    fast_upate_mode: bool,
    root: FilePath,
    inputs: Vec<FilePath>,
    output: FilePath,
    trim: Option<PathBuf>,
    base_url: Option<String>,
    changed_file: Option<FilePath>,
) {
    use rayon::prelude::*;
    inputs
        .into_iter()
        .for_each(|input| {
            compile_file(
                fast_upate_mode,
                &root,
                &output,
                &trim,
                &base_url,
                input,
                changed_file.clone(),
            );
        });
    println!("[Subscript] Compiled");
}

pub fn serve(
    root: FilePath,
    inputs: Vec<FilePath>,
    output: FilePath,
    trim: Option<PathBuf>,
    base_url: Option<String>,
    port: u16,
) {
    use hotwatch::{Hotwatch, Event};
    let mut hotwatch = Hotwatch::new().expect("hotwatch failed to initialize!");
    let fast_upate_mode = false;
    hotwatch.unwatch(output.clone());
    hotwatch.watch(root.clone(), {
        let output = output.clone();
        let root = root.clone();
        move |event: Event| {
            match event {
                Event::Create(path) => {
                    let path = FilePath::new(path).unwrap();
                    let is_output_file: bool = path.is_child_path(&output);
                    if !is_output_file {
                        compile(
                            fast_upate_mode,
                            root.clone(),
                            inputs.clone(),
                            output.clone(),
                            trim.clone(),
                            base_url.clone(),
                            Some(path),
                        );
                    }
                }
                Event::Write(path) => {
                    let path = FilePath::new(path).unwrap();
                    let is_output_file: bool = path.is_child_path(&output);
                    if !is_output_file {
                        compile(
                            fast_upate_mode,
                            root.clone(),
                            inputs.clone(),
                            output.clone(),
                            trim.clone(),
                            base_url.clone(),
                            Some(path),
                        );
                    }
                }
                Event::Remove(path) => {
                    let path = FilePath::new(path).unwrap();
                    let is_output_file: bool = path.is_child_path(&output);
                    if !is_output_file {
                        compile(
                            fast_upate_mode,
                            root.clone(),
                            inputs.clone(),
                            output.clone(),
                            trim.clone(),
                            base_url.clone(),
                            Some(path),
                        );
                    }
                }
                Event::Rename(from, to) => {
                    let path = FilePath::new(to).unwrap();
                    let is_output_file: bool = path.is_child_path(&output);
                    if !is_output_file {
                        compile(
                            fast_upate_mode,
                            root.clone(),
                            inputs.clone(),
                            output.clone(),
                            trim.clone(),
                            base_url.clone(),
                            Some(path),
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
            path: output.clone().to_path_buffer(),
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
    fn process_inputs(input: Vec<String>) -> Vec<FilePath> {
        input
            .into_iter()
            .flat_map(|x: String| -> Vec<PathBuf> {
                glob::glob(&x).unwrap()
                    .filter_map(Result::ok)
                    .collect::<Vec<_>>()
            })
            .filter_map(FilePath::new)
            .collect::<Vec<_>>()
    }
    match Opt::from_args() {
        Opt::Compile{root, input, output, trim, base_url} => {
            let root = FilePath::new(root).unwrap();
            let output = FilePath::new(output).unwrap();
            compile(
                false,
                root,
                process_inputs(input),
                output,
                trim,
                base_url,
                None,
            );
        }
        Opt::Serve{root, input, output, trim, base_url, port, open_browser} => {
            let inputs = process_inputs(input);
            let root = FilePath::new(root).unwrap();
            let output = FilePath::new(output).unwrap();
            if open_browser {
                std::thread::spawn({
                    let pot = port.clone();
                    move || {
                        crate::browser::run(port);
                    }
                });
            }
            compile(
                false,
                root.clone(),
                inputs.clone(),
                output.clone(),
                trim.clone(),
                base_url.clone(),
                None,
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

