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
    #[error("Expected the annotation to contain an image path")]
    MissingImagePath,
    #[error("Expected the annotation to contain a class name")]
    MissingClassName,
    #[error("Expected the annotation to contain a class ID")]
    MissingClassID,
    #[error("Image is missing {}")]
    MissingImageDimensions(String),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("The expected the destination to have the .{expected} extension, but got .{found}")]
    WrongExtension {
        expected: String,
        found: String,
    },
    #[error("The internal stream is closed. No annotations can be written")]
    StreamClosed,
    #[error("{0}")]
    Other(String),
    #[error("CSV Error: {0}")]
    Csv(#[from] csv::Error),
}

pub type SerializerResult<T> = Result<T, SerializerError>;

