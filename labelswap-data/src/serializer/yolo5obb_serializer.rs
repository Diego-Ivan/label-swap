/* yolo5obb_serializer.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::{collections::HashMap, fs::File, io::Write, path::{Path, PathBuf}};
use crate::models::{annotation::ClassRepresentation, Annotation};

use super::{FormatSerializer, SerializerError, SerializerResult};

pub struct Yolo5ObbSerializer {
    destination: Option<PathBuf>,
    annotation_map: HashMap<PathBuf, Vec<Annotation>>,
}

impl Yolo5ObbSerializer {
    pub fn new() -> Self {
        Self {
            destination: None,
            annotation_map: HashMap::new(),
        } 
    }

    fn write_to_file(path: &Path, annotations: &[Annotation]) -> SerializerResult<()> {
        let mut stream = File::open(path)?;

        for annotation in annotations {
            let class = match annotation.class.as_ref() {
                ClassRepresentation::ClassName(name) => name,
                ClassRepresentation::Both { name,..} => name,
                _ => return Err(SerializerError::WrongClassRepresentation(
                    format!("Expected class representation to contain name")
                ))
            };

            let difficulty = if annotation.difficulty {
                1
            } else {
                0
            };
            let format = format!("{} {} {} {} {} {} {} {} {} {}",
                annotation.x1,
                annotation.y1,
                annotation.x2,
                annotation.y2,
                annotation.x3,
                annotation.y3,
                annotation.x4,
                annotation.y4,
                class,
                difficulty
            );
            stream.write(format.as_bytes())?;
        }

        Ok(())
    }
}

impl FormatSerializer for Yolo5ObbSerializer {
    fn init(&mut self, path: impl Into<PathBuf>) -> SerializerResult<()> {
        let path: PathBuf = path.into();

        if !path.is_dir() {
            return Err(SerializerError::WrongDestination {
                expected: crate::models::format::SourceType::MultipleFiles,
                found: crate::models::format::SourceType::SingleFile
            });
        }

        if !path.exists() {
            std::fs::create_dir_all(&path);
        }

        self.destination = Some(path);

        Ok(())
    }

    fn push(&mut self, annotation: Annotation) -> SerializerResult<()> {
        let path = match annotation.source_file.as_ref() {
            Some(path) => path,
            None => return Err(SerializerError::MissingSourceFile),
        };

        // We won't use .entry here as it would require making a copy of the PathBuf
        // everytime we have to use it. We will only create the necessary copies.
        if !self.annotation_map.contains_key(path) {
            self.annotation_map.insert(path.clone(), Vec::new());
        }

        let array = self.annotation_map.get_mut(path).unwrap();
        array.push(annotation);

        Ok(())
    }

    fn finish(self) -> SerializerResult<()> {
        // TODO: Remove unwrap
        let destination = self.destination.as_ref().unwrap();

        for (path, annotations) in self.annotation_map {
            let file_name = match path.file_name() {
                Some(file_name) => PathBuf::from(file_name),
                None => {
                    eprintln!("Could not get a file name for path {:?}", path);
                    continue;
                }
            };

            let relative_path = match crate::resolve_relative_path(destination, &file_name) {
                Ok(relative_path) => relative_path,
                Err(e) => {
                    eprintln!("Failed to parse {:?}, got {}", file_name, e);
                    continue;
                }
            };

            Self::write_to_file(&relative_path, &annotations)?;
        }

        Ok(())
    }
}

