use crate::models::Annotation;
use anyhow::Result;
use std::path::Path;

pub trait FormatParser {
    fn init(&mut self, path: Box<Path>) -> Result<()>;
    fn get_next(&mut self) -> anyhow::Result<Annotation>;
    fn has_next(&mut self) -> bool;
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ParserError {
    #[error("Wrong file type")]
    WrongFileType(String),
    #[error("Wrong Format")]
    WrongFormat(String),
}
