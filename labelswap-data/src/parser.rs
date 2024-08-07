mod cocojson_parser;
mod format_parser;
mod tfobjectdetection;
mod yolo5obb_parser;
mod yolo5txt_parser;
mod yolodarknet_parser;

use std::io::{BufRead, BufReader, Read};

pub use cocojson_parser::CocoJsonParser;
pub use format_parser::FormatParser;
pub use tfobjectdetection::TfObjectDetectionParser;
pub use yolo5obb_parser::Yolo5ObbParser;
pub use yolo5txt_parser::Yolo5TxtParser;
pub use yolodarknet_parser::YoloDarknetParser;

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
    #[error("Wrong format: {0}")]
    WrongFormat(String),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error: {0}")]
    Other(String),
    #[error("No more elements to parse")]
    OutOfElements,
}

fn reader_has_data_left<R>(reader: &mut BufReader<R>) -> bool
where
    R: ?Sized + Read,
{
    match reader.fill_buf().map(|b| !b.is_empty()) {
        Ok(result) => result,
        Err(_) => false,
    }
}
