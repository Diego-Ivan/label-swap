/* yolo8obb_parser.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use super::{FormatParser, ParserError};
use crate::models::{annotation::ClassRepresentation, format::SourceType, Annotation, Image};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct Yolo8ObbParser {
    source_directory: PathBuf,
    current_reader: Option<BufReader<std::fs::File>>,
    file_enumerator: Option<std::fs::ReadDir>,
    current_entry: Option<std::fs::DirEntry>,
}

impl FormatParser for Yolo8ObbParser {
    fn init(&mut self, path: impl Into<PathBuf>) -> Result<(), ParserError> {
        let path: PathBuf = path.into();

        if !path.metadata()?.is_dir() {
            return Err(ParserError::WrongSource {
                expected: SourceType::MultipleFiles,
                found: SourceType::SingleFile,
            });
        }

        self.source_directory = path;
        self.file_enumerator = Some(std::fs::read_dir(&self.source_directory)?);
        todo!()
    }

    fn get_next(&mut self) -> Result<Annotation, ParserError> {
        let reader = match self.current_reader.as_mut() {
            Some(reader) => reader,
            None => return Err(ParserError::OutOfElements),
        };

        let current_entry = match self.current_entry.as_ref() {
            Some(entry) => entry,
            None => return Err(ParserError::OutOfElements),
        };

        let mut line = String::new();
        reader.read_line(&mut line)?;

        let tokens: Vec<&str> = line.split(' ').collect();
        if tokens.len() != 9 {
            return Err(ParserError::WrongFormat(format!(
                "Expected 9 elements in line '{line}, but got {}",
                tokens.len()
            )));
        }

        let class_index: usize = match tokens[0].parse() {
            Ok(index) => index,
            Err(e) => return Err(ParserError::Other(format!("{e}"))),
        };

        let coordinates: Vec<f64> = tokens[1..]
            .iter()
            .filter_map(|c| c.trim().parse().ok())
            .collect();

        if coordinates.len() != 8 {
            return Err(ParserError::WrongFormat(format!(
                "Expected 8 coordinates in line '{line}, but got {}.",
                tokens.len()
            )));
        }

        Ok(Annotation {
            image: Image::empty(),
            difficulty: false,
            class: ClassRepresentation::ClassId(class_index.to_string()),
            source_file: Some(current_entry.path()),
            x1: coordinates[0],
            y1: coordinates[1],
            x2: coordinates[2],
            y2: coordinates[3],
            x3: coordinates[4],
            y3: coordinates[5],
            x4: coordinates[6],
            y4: coordinates[7],
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
