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

pub fn resolve_relative_path(path: &Path, relative_path: &Path) -> Result<PathBuf, io::Error> {
    let canonical_path = path.canonicalize()?;
    Ok(canonical_path.join(relative_path))
}
