use std::io::{self, BufRead, BufReader, Read};

use anyhow::Result;

pub mod models;
pub mod conversion_pipeline;
pub mod parser;

pub fn reader_has_data_left<R>(reader: &mut BufReader<R>) -> Result<bool, io::Error>
where
    R: ?Sized + Read,
{
    reader.fill_buf().map(|b| !b.is_empty())
}
