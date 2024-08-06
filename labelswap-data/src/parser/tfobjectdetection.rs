/* tfobjectdetection.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::{fs::File, path::PathBuf};

use crate::models::format::SourceType;

use super::{FormatParser, ParserError};

pub struct TfObjectDetectionParser {
    reader: Option<csv::StringRecordsIntoIter<File>>,
    source_file: PathBuf,
}

impl TfObjectDetectionParser {
    pub fn new() -> Self {
        Self {
            reader: None,
            source_file: PathBuf::new(),
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
            Ok(reader) => Some(reader.into_records()),
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
        let reader = self.reader.as_mut().ok_or(ParserError::OutOfElements);
        todo!()
    }

    fn has_next(&mut self) -> bool {
        match self.reader.as_ref() {
            Some(reader) => reader.is_done(),
            None => false,
        }
    }
}
