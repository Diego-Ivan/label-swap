use std::{
    io::{self, BufRead, BufReader, Read},
    path::{Path, PathBuf},
};

use anyhow::Result;

pub mod conversion_pipeline;
pub mod models;
pub mod parser;
pub mod serializer;
pub mod transforms;

pub fn resolve_relative_path<P, R>(path: P, relative_path: R)-> Result<PathBuf, io::Error>
where P: AsRef<Path>,
      R: AsRef<Path> {
    let path: &Path = path.as_ref();
    let canonical_path = path.canonicalize()?;
    Ok(canonical_path.join(relative_path))
}
