/* tfobjectdetection.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::{fs:: File, path::PathBuf, cell::Cell};

use crate::models::{Annotation, Image, annotation::ClassRepresentation, format::SourceType};

use super::{FormatParser, ParserError};

pub struct TfObjectDetectionParser {
    reader: Option<csv::DeserializeRecordsIntoIter<File, Record>>,
    next_item: Cell<Option<Record>>,
    source_file: PathBuf,
}

#[derive(Debug, serde::Deserialize)]
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

impl TfObjectDetectionParser {
    pub fn new() -> Self {
        Self {
            reader: None,
            source_file: PathBuf::new(),
            next_item: Cell::new(None),
        }
    }
}

impl FormatParser for TfObjectDetectionParser {
    fn init(&mut self, path: impl Into<PathBuf>) -> Result<(), super::ParserError> {
        let path: PathBuf = path.into();
        if !path.metadata()?.is_file() {
            return Err(ParserError::WrongSource {
                expected: SourceType::SingleFile,
                found: SourceType::MultipleFiles,
            });
        }
        self.source_file.push(path);

        self.reader = match csv::Reader::from_path(&self.source_file) {
            Ok(reader) => Some(reader.into_deserialize()),
            Err(e) => match e.into_kind() {
                csv::ErrorKind::Io(io) => return Err(ParserError::Io(io)),
                _ => {
                    return Err(ParserError::WrongFormat(format!(
                        "CSV is malformed. Please verify its integrity"
                    )))
                }
            },
        };

        Ok(())
    }

    fn get_next(&mut self) -> Result<crate::models::Annotation, super::ParserError> {
        let next = self.next_item.take().ok_or(ParserError::OutOfElements)?;
        Ok(Annotation {
            image: Image {
                height: Some(next.height),
                width: Some(next.width),
                path: Some(PathBuf::from(next.filename)),
                id: None,
            },
            class: ClassRepresentation::ClassName(next.class),
            source_file: Some(self.source_file.clone()),
            difficulty: false,
            ..Annotation::from_min_max(next.xmin, next.xmax, next.ymin, next.ymax)
        })
    }

    fn has_next(&mut self) -> bool {
        let iter = match self.reader.as_mut() {
            Some(iter) => iter,
            None => return false,
        };

        let next_item = iter.find_map(|s| s.ok());
        let is_some = next_item.is_some();
        self.next_item.set(next_item);
        is_some
    }
}
