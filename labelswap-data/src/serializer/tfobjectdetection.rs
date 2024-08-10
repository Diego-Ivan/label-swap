use super::{FormatSerializer, SerializerResult, SerializerError};
use crate::models::{Annotation, format::SourceType, annotation::ClassRepresentation};

use std::path::PathBuf;

pub struct TfObjectDetectionSerializer {
    destination: PathBuf,
    writer: Option<csv::Writer<std::fs::File>>
}

#[derive(Debug, serde::Serialize)]
struct Record {
    filename: String,
    width: u32,
    height: u32,
    class: String,
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
}

impl FormatSerializer for TfObjectDetectionSerializer {
    fn init(&mut self, path: impl Into<std::path::PathBuf>) -> SerializerResult<()> {
        let mut path: PathBuf = path.into();

        if !path.metadata()?.is_file() {
            return Err(SerializerError::WrongDestination {
                expected: SourceType::SingleFile,
                found: SourceType::MultipleFiles,
            })
        }

        match path.extension() {
            Some(extension) => {
                if extension != "csv" {
                    return Err(SerializerError::WrongExtension {
                        expected: String::from("csv"),
                        found: extension.to_string_lossy().to_string()
                    })
                }
            }
            None => {
                path.set_extension("csv");
            }
        };

        let mut writer = csv::Writer::from_path(&path)?;
        writer.write_record(&["filename", "width", "height", "class", "xmin", "ymin", "xmax", "xmin"])?;
        self.writer = Some(writer);
        self.destination.push(path);
        Ok(())
    }

    fn push(&mut self, annotation: Annotation) -> SerializerResult<()> {
        let writer = self.writer.as_mut().ok_or(SerializerError::StreamClosed)?;
        let image_path = annotation.image.path.as_ref().ok_or(SerializerError::MissingImagePath)?;
        let class = match annotation.class.as_ref() {
            ClassRepresentation::ClassName(name) => name,
            ClassRepresentation::Both { name, .. } => name,
            _ => return Err(SerializerError::MissingClassName)
        };

        let width = annotation
            .image
            .width
            .ok_or(SerializerError::MissingImageDimensions(String::from("Width")))?;
        let height = annotation
            .image
            .height
            .ok_or(SerializerError::MissingImageDimensions(String::from("Width")))?;

        let tf_record = Record {
            filename: image_path.file_name().unwrap().to_string_lossy().to_string(),
            class: class.clone(),
            width,
            height,
            xmin: annotation.get_xmin(),
            xmax: annotation.get_xmax(),
            ymin: annotation.get_ymin(),
            ymax: annotation.get_ymax(),
        };

        writer.serialize(tf_record)?;

        Ok(())
    }

    fn finish(self) -> SerializerResult<()> {
        let writer = self.writer.as_mut().ok_or(SerializerError::StreamClosed)?;
        writer.flush()?;

        Ok(())
    }
}
