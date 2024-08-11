/* tfobjectdetection.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use super::{FormatParser, ParserError};
use crate::models::{annotation::ClassRepresentation, format::SourceType, Annotation, Image};
use std::fs::{DirEntry, ReadDir};
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct YoloDarknetParser {
    source_directory: PathBuf,
    file_enumerator: Option<ReadDir>,
    current_entry: Option<DirEntry>,
    current_reader: Option<BufReader<std::fs::File>>,
    class_map: Vec<String>,
}

impl YoloDarknetParser {
    pub fn new() -> Self {
        Self {
            source_directory: PathBuf::new(),
            class_map: Vec::new(),
            file_enumerator: None,
            current_entry: None,
            current_reader: None,
        }
    }

    fn parse_class_map(&mut self) -> Result<(), io::Error> {
        let file = std::fs::read_dir(&self.source_directory)?.find_map(|f| match f {
            Ok(f) => {
                if f.file_name() == "darknet.labels" {
                    return Some(f);
                }
                None
            }
            Err(_) => None,
        });
        let file = match file {
            Some(file) => file,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "darknet.labels file was not found",
                ))
            }
        };

        if !file.metadata()?.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "darknet.labels is expected to be a regular file",
            ));
        }

        let file = std::fs::File::open(file.path())?;
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            self.class_map.push(line?);
        }

        Ok(())
    }
}

impl FormatParser for YoloDarknetParser {
    fn init(&mut self, path: impl Into<std::path::PathBuf>) -> Result<(), ParserError> {
        let path: PathBuf = path.into();
        if !path.metadata().unwrap().is_dir() {
            return Err(ParserError::WrongSource {
                expected: SourceType::MultipleFiles,
                found: SourceType::SingleFile,
            });
        }

        self.source_directory.push(path);
        self.parse_class_map()?;
        self.file_enumerator = std::fs::read_dir(&self.source_directory).ok();

        Ok(())
    }

    fn get_next(&mut self) -> Result<crate::models::Annotation, ParserError> {
        let reader = self
            .current_reader
            .as_mut()
            .ok_or(ParserError::OutOfElements)?;
        let current_entry = self
            .current_entry
            .as_ref()
            .ok_or(ParserError::OutOfElements)?;
        let mut line = String::new();
        reader.read_line(&mut line)?;

        let elements: Vec<&str> = line.split(' ').collect();
        if elements.len() != 5 {
            return Err(ParserError::WrongFormat(format!(
                "Expected 5 elements in line '{line}', but got {}",
                elements.len()
            )));
        }

        let class_id: usize = elements[0]
            .parse()
            .map_err(|e| ParserError::Other(format!("Invalid Class ID: {e}")))?;

        // Indexing the class map to get the class name
        let class_name = match self.class_map.get(class_id) {
            Some(class_name) => class_name.clone(),
            None => return Err(ParserError::WrongFormat(format!(
                "Class id {class_id} does not have a corresponding class name in darknet.labels"
            ))),
        };

        let coordinates: Vec<f64> = elements[1..]
            .iter()
            .filter_map(|c| c.trim().parse().ok())
            .collect();

        if coordinates.len() != 4 {
            return Err(ParserError::WrongFormat(format!(
                "Expected 4 coordinates in line '{line}', but got {}",
                coordinates.len()
            )));
        }

        Ok(Annotation {
            source_file: Some(current_entry.path()),
            class: ClassRepresentation::Both {
                name: class_name,
                id: class_id.to_string(),
            },
            image: Image::empty(),
            difficulty: false,
            ..Annotation::from_centers(
                coordinates[0],
                coordinates[1],
                coordinates[2],
                coordinates[3],
            )
        })
    }

    fn has_next(&mut self) -> bool {
        // Checks if we still have lines remaining in the buffer.
        if let Some(reader) = self.current_reader.as_mut() {
            if super::reader_has_data_left(reader) {
                return true;
            }
        }

        // If we've gotten here, it means we have to look
        // for the next file, and if it exisits,
        // open a buffer for it.
        let file_enumerator = self.file_enumerator.as_mut().unwrap();

        self.current_entry = file_enumerator.find_map(|entry| {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => return None,
            };

            match entry.path().extension() {
                Some(extension) => {
                    if extension == "txt" {
                        return Some(entry);
                    }
                    None
                }
                None => None,
            }
        });

        match self.current_entry.as_ref() {
            // Open the next reader and return true
            Some(entry) => {
                let file = std::fs::File::open(entry.path()).unwrap();
                self.current_reader = Some(BufReader::new(file));
                true
            }
            // Means we have reached the end of the folder
            None => {
                self.current_reader = None;
                false
            }
        }
    }
}
