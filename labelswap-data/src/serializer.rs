mod format_serializer;
mod yolo5obb_serializer;
mod tfobjectdetection;

pub use format_serializer::FormatSerializer;
pub use yolo5obb_serializer::Yolo5ObbSerializer;
pub use tfobjectdetection::TfObjectDetectionSerializer;

use crate::models::{annotation, format};

#[derive(thiserror::Error, Debug)]
pub enum SerializerError {
    #[error("Class Representation {0} is not supported")]
    WrongClassRepresentation(String),
    #[error("Wrong destination, expected {expected}, but got {found}")]
    WrongDestination {
        expected: format::SourceType,
        found: format::SourceType,
    },
    #[error("An annotation is missing a source file")]
    MissingSourceFile,
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

pub type SerializerResult<T> = Result<T, SerializerError>;

