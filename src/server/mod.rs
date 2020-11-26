// Copyright (c) 2018 Weihang Lo
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! bail {
    ($($tt:tt)*) => {
        return Err(From::from(format!($($tt)*)));
    }
}

mod extensions;
mod http;
mod server;
use std::env;
use std::fs::canonicalize;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use crate::server::server::serve;

pub fn run_server(args: Args) {
    #[tokio::main]
    async fn server_impl(args: Args) {
        use futures_util::future::TryFutureExt;
        fn handle_err<T>(err: Box<dyn std::error::Error>) -> T {
            eprintln!("Server error: {}", err);
            std::process::exit(1);
        }
        serve(args)
            .unwrap_or_else(handle_err)
            .await
    }
    server_impl(args);
}

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Args {
    pub address: String,
    pub port: u16,
    pub cache: u64,
    pub cors: bool,
    pub compress: bool,
    pub path: PathBuf,
    pub all: bool,
    pub ignore: bool,
    pub follow_links: bool,
    pub render_index: bool,
    pub log: bool,
    pub path_prefix: Option<String>,
}

impl Args {
    /// Parse path.
    fn parse_path<P: AsRef<Path>>(path: P) -> BoxResult<PathBuf> {
        let path = path.as_ref();
        if !path.exists() {
            bail!("error: path \"{}\" doesn't exist", path.display());
        }
        env::current_dir()
            .and_then(|mut p| {
                p.push(path); // If path is absolute, it replaces the current path.
                canonicalize(p)
            })
            .or_else(|err| {
                bail!(
                    "error: failed to access path \"{}\": {}",
                    path.display(),
                    err,
                )
            })
    }
    /// Construct socket address from arguments.
    pub fn address(&self) -> BoxResult<SocketAddr> {
        format!("{}:{}", self.address, self.port)
            .parse()
            .or_else(|err| {
                bail!(
                    "error: invalid address {}:{} : {}",
                    self.address,
                    self.port,
                    err,
                )
            })
    }
}

