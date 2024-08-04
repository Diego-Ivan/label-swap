pub mod cocojson_parser;
pub mod format_parser;
pub mod yolo5obb_parser;

pub use format_parser::FormatParser;
pub use yolo5obb_parser::Yolo5ObbParser;

use crate::models::format::SourceType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Wrong file type: Expected: {expected}, found {found}")]
    WrongFileType { expected: String, found: String },
    #[error("Wrong destination, expected {expected}, but got {found}")]
    WrongSource {
        expected: SourceType,
        found: SourceType,
    },
    #[error("Wrong format: {msg}")]
    WrongFormat { msg: String },
}
