use super::{FormatParser, ParserError};
use crate::models::{self, annotation::ClassRepresentation, format::SourceType, Image};
use std::{
    fs::{DirEntry, ReadDir},
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub struct Yolo5TxtParser {
    source_directory: Option<PathBuf>,
    file_enumerator: Option<ReadDir>,
    current_entry: Option<DirEntry>,
    current_reader: Option<BufReader<std::fs::File>>,
}

impl Yolo5TxtParser {
    pub fn new() -> Self {
        Self {
            source_directory: None,
            file_enumerator: None,
            current_entry: None,
            current_reader: None,
        }
    }
}

impl FormatParser for Yolo5TxtParser {
    fn init(&mut self, path: impl Into<std::path::PathBuf>) -> Result<(), ParserError> {
        let path: PathBuf = path.into();
        let metadata = path.metadata().unwrap();

        if !metadata.is_dir() {
            return Err(ParserError::WrongSource {
                expected: SourceType::MultipleFiles,
                found: SourceType::SingleFile,
            });
        }
        self.file_enumerator = Some(std::fs::read_dir(&path)?);
        self.source_directory = Some(path);
        Ok(())
    }

    fn get_next(&mut self) -> Result<models::Annotation, ParserError> {
        let reader = self
            .current_reader
            .as_mut()
            .ok_or(ParserError::OutOfElements)?;

        let mut line = String::new();
        let _ = reader.read_line(&mut line)?;

        let elements: Vec<&str> = line.trim().split(' ').collect();
        if elements.len() != 5 {
            return Err(ParserError::WrongFormat(format!(
                "Expected 5 elements in line '{line}', but got {}",
                elements.len()
            )));
        }

        let coordinates: Vec<f64> = elements[1..]
            .iter()
            .filter_map(|c| c.parse().ok())
            .collect();

        if coordinates.len() != 4 {
            return Err(ParserError::WrongFormat(format!(
                "Expected 4 valid coordinates in line '{line}', but got {}",
                coordinates.len()
            )));
        }

        Ok(models::Annotation {
            source_file: Some(self.current_entry.as_ref().unwrap().path()),
            class: ClassRepresentation::ClassId(elements[0].to_string()),
            difficulty: false,
            image: Image::empty(),
            ..models::Annotation::from_centers(
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
