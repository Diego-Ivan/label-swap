use super::{FormatSerializer, SerializerError, SerializerResult};
use crate::models::{annotation::ClassRepresentation, format::SourceType, Annotation};
use std::collections::HashMap;
use std::path::PathBuf;

use chrono::Datelike;
use serde_json as json;

pub struct CocoJsonSerializer {
    destination: PathBuf,
    root_object: json::Value,
    categories: HashMap<u32, json::Value>,
    images: HashMap<u32, json::Value>,
    annotations: Vec<json::Value>,
    datetime_now: chrono::DateTime<chrono::Utc>,
}

impl CocoJsonSerializer {
    pub fn new() -> Self {
        Self {
            destination: PathBuf::new(),
            root_object: json::Value::Null,
            categories: HashMap::new(),
            images: HashMap::new(),
            annotations: Vec::new(),
            datetime_now: chrono::Utc::now(),
        }
    }
}

impl FormatSerializer for CocoJsonSerializer {
    fn init(&mut self, path: impl Into<PathBuf>) -> SerializerResult<()> {
        let mut path: PathBuf = path.into();
        if !path.metadata()?.is_file() {
            return Err(SerializerError::WrongDestination {
                expected: SourceType::SingleFile,
                found: SourceType::MultipleFiles,
            });
        }

        match path.extension() {
            Some(extension) => {
                if extension != "json" {
                    return Err(SerializerError::WrongExtension {
                        expected: String::from("json"),
                        found: extension.to_string_lossy().to_string(),
                    });
                }
            }
            None => {
                path.set_extension("json");
            }
        }
        self.destination.push(path);
        let root_object = json::json!({
            "info": {
                "year": self.datetime_now.year(),
                "version": 1,
                "description": "A dataset created using Label Swap",
                "contributor": "Diego Iván M.E, Label Swap author.",
                "url": "Not Available",
                "data_created": self.datetime_now.to_rfc3339(),
            },
        });
        self.root_object = root_object;
        Ok(())
    }

    fn push(&mut self, annotation: Annotation) -> SerializerResult<()> {
        let (class_id, class_name) = match annotation.class.as_ref() {
            ClassRepresentation::Both { name, id } => (id, name),
            _ => {
                return Err(SerializerError::Other(String::from(
                    "Expected annotation to contain both image name and id",
                )))
            }
        };

        let class_id: u32 = match class_id.parse() {
            Ok(class_id) => class_id,
            Err(e) => {
                return Err(SerializerError::Other(format!(
                    "Failed to parse class ID into a number: {e}"
                )))
            }
        };

        // We have to add the images and categories to the format map,
        // as must add them to the final document

        if !self.categories.contains_key(&class_id) {
            let category = json::json!({
                "id": class_id,
                "name": class_name.clone(),
                "supercategory": "none",
            });
            self.categories.insert(class_id, category);
        }

        let image_id = annotation.image.id.ok_or(SerializerError::MissingImageId)?;
        let image_filename = annotation
            .image
            .path
            .as_ref()
            .ok_or(SerializerError::MissingImagePath)?
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let height = annotation
            .image
            .height
            .ok_or(SerializerError::MissingImageDimensions(String::from(
                "height",
            )))?;

        let width = annotation
            .image
            .height
            .ok_or(SerializerError::MissingImageDimensions(String::from(
                "width",
            )))?;

        if !self.images.contains_key(&image_id) {
            let image_object = json::json!({
                "id": image_id,
                "file_name": image_filename,
                "height": height,
                "width": width,
                "date_captured": self.datetime_now.to_rfc3339(),
            });
            self.images.insert(image_id, image_object);
        }

        let annot_width = annotation.get_xmax() - annotation.get_xmin();
        let annot_height = annotation.get_ymax() - annotation.get_ymin();

        let annotation = json::json!({
            "id": self.annotations.len(),
            "image_id": image_id,
            "category_id": class_id,
            "bbox": [
                annotation.get_xmin(),
                annotation.get_ymin(),
                annot_width,
                annot_height,
            ],
            "area": annot_width * annot_height,
            "iscrowd": 0,
            "segmentation": [],
        });

        self.annotations.push(annotation);
        Ok(())
    }

    fn finish(mut self) -> SerializerResult<()> {
        let root_map = self.root_object.as_object_mut().unwrap();
        let categories: Vec<json::Value> = self.categories.into_values().collect();
        let images: Vec<json::Value> = self.images.into_values().collect();

        root_map.insert("categories".into(), json::Value::Array(categories));
        root_map.insert("images".into(), json::Value::Array(images));
        root_map.insert("annotations".into(), json::Value::Array(self.annotations));

        let stream = std::fs::File::open(self.destination)?;
        json::to_writer_pretty(stream, &self.root_object);

        Ok(())
    }
}
