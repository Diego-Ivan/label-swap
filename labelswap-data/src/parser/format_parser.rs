use crate::models::Annotation;
use std::path::PathBuf;

use super::ParserError;

pub trait FormatParser {
    fn init(&mut self, path: impl Into<PathBuf>) -> Result<(), ParserError>;
    fn get_next(&mut self) -> Result<Annotation, ParserError>;
    fn has_next(&mut self) -> bool;
}
