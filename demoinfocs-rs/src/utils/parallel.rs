use rayon::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::parser::Parser;

/// Runs the given function for every demo path in parallel.
///
/// The closure receives a mutable [`Parser`] for the demo file and the path
/// to the file. Any parsing work (such as calling [`Parser::parse_to_end`])
/// is left to the closure.
pub fn run<I, P, F>(paths: I, func: F)
where
    I: IntoParallelIterator<Item = P>,
    P: Into<PathBuf> + Send,
    F: Fn(&mut Parser<File>, &Path) + Send + Sync,
{
    paths.into_par_iter().for_each(|p| {
        let path: PathBuf = p.into();
        let file = File::open(&path).expect("failed to open demo file");
        let mut parser = Parser::new(file);
        func(&mut parser, &path);
    });
}
