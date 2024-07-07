use crate::models::Annotation;
use crate::reader_has_data_left;
use crate::{
    models::annotation::ClassRepresentation,
    parser::{format_parser::ParserError, FormatParser},
};
use anyhow::anyhow;
use anyhow::Result;
use std::path::PathBuf;
use std::{
    fs::{DirEntry, File, ReadDir},
    io::{BufRead, BufReader},
    path::Path,
};

pub struct Yolo5ObbParser {
    source_directory: Option<Box<Path>>,
    file_enumerator: Option<ReadDir>,
    current_entry: Option<DirEntry>,
    current_reader: Option<BufReader<File>>,
}

impl Yolo5ObbParser {
    pub fn new() -> Yolo5ObbParser {
        Self {
            source_directory: None,
            file_enumerator: None,
            current_entry: None,
            current_reader: None,
        }
    }

    fn look_for_next_entry(&mut self) -> Result<DirEntry> {
        let enumerator: &mut ReadDir = self.file_enumerator.as_mut().unwrap();

        while let Some(next_entry) = enumerator.next() {
            let next_entry = next_entry?;
            let path = next_entry.path();
            let extension = path
                .extension()
                .ok_or(anyhow!("Expected entry to have a file extension"))?;
            if extension == "txt" {
                return Ok(next_entry);
            }
        }

        Err(anyhow!("Could not find a text file"))
    }

    fn reader_has_data(&mut self) -> bool {
        let reader = self.current_reader.as_mut();

        if let Some(reader) = reader {
            let has_data = reader_has_data_left(reader);
            if has_data.is_ok() && has_data.unwrap() {
                return true;
            }
        }
        return false;
    }
}

impl FormatParser for Yolo5ObbParser {
    fn init(&mut self, path: impl Into<PathBuf>) -> Result<()> {
        let path = path.into();
        let metadata = std::fs::metadata(&path)?;
        if !metadata.is_dir() {
            return Err(anyhow::anyhow!("Expected given path to be a directory"));
        }

        let enumerator = std::fs::read_dir(&path)?;
        self.file_enumerator = Some(enumerator);

        self.source_directory = Some(path.into());
        Ok(())
    }

    fn get_next(&mut self) -> Result<Annotation> {
        let reader: &mut BufReader<File> = self
            .current_reader
            .as_mut()
            .ok_or(anyhow!("The stream is closed"))?;

        let mut line = String::new();
        let _ = reader.read_line(&mut line)?;

        let elements: Vec<&str> = line.trim().split(" ").collect();
        if elements.len() < 9 {
            return Err(anyhow!("Expected at least 9 elements in line {line}"));
        }

        let coordinates: Vec<f64> = elements[..8]
            .iter()
            .filter_map(|c| c.parse::<f64>().ok())
            .collect();
        if coordinates.len() < 8 {
            return Err(anyhow!("Expected 4 (x, y) pairs, got {}", coordinates.len()))
        }

        let source_file = self
            .current_entry
            .as_ref()
            .unwrap()
            .path();

        Ok(Annotation {
            x1: coordinates[0],
            y1: coordinates[1],
            x2: coordinates[2],
            y2: coordinates[3],
            x3: coordinates[4],
            y3: coordinates[5],
            x4: coordinates[6],
            y4: coordinates[7],
            class: ClassRepresentation::ClassName(elements[8].to_string()),
            source_file: Some(source_file),
            difficulty: match elements.get(9) {
                Some(i) => i.parse::<i32>().unwrap() != 0,
                None => false,
            },
            image: None,
        })
    }

    fn has_next(&mut self) -> bool {
        if self.reader_has_data() {
            return true;
        }

        let reader = self.current_reader.as_mut();

        if reader.is_none() || !reader_has_data_left(reader.unwrap()).unwrap() {
            self.current_entry = self.look_for_next_entry().ok();
            if self.current_entry.is_none() {
                return false;
            }
        }

        let current_entry = &self.current_entry;
        let text_file = match File::open(current_entry.as_ref().unwrap().path()) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("{e}");
                return false;
            }
        };
        self.current_reader = Some(BufReader::new(text_file));
        return true;
    }
}
