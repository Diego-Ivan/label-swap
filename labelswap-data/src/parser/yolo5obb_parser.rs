use crate::models::Annotation;
use crate::reader_has_data_left;
use crate::{
    models::annotation::ClassRepresentation,
    parser::{format_parser::ParserError, FormatParser},
};
use anyhow::anyhow;
use anyhow::Result;
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
        let mut entry: Option<DirEntry> = None;
        let enumerator: &mut ReadDir = self.file_enumerator.as_mut().unwrap();
        while let Some(next_entry) = enumerator.next() {
            let next_entry = next_entry?;
            if next_entry
                .file_name()
                .into_string()
                .unwrap()
                .ends_with(".txt")
            {
                entry = Some(next_entry);
                break;
            }
        }
        entry.ok_or(anyhow!(ParserError::WrongFormat(format!(
            "Unable to find text file"
        ))))
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
    fn init(&mut self, path: Box<Path>) -> Result<()> {
        let metadata = std::fs::metadata(&path)?;
        if !metadata.is_dir() {
            return Err(anyhow::anyhow!("Expected given path to be a directory"));
        }

        let enumerator = std::fs::read_dir(&path)?;
        self.file_enumerator = Some(enumerator);

        self.source_directory = Some(path);
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
            return Err(anyhow!(ParserError::WrongFormat(format!(
                "Expected at least 9 elements in line {line}"
            ))));
        }

        let mut coordinates = [0.0; 8];
        for i in 0..coordinates.len() {
            let coordinate: f64 = match elements[i].parse() {
                Ok(val) => val,
                Err(err) => {
                    return Err(anyhow!(ParserError::WrongFormat(format!(
                        "Unable to parse element {i} in line {line}: {err}"
                    ))))
                }
            };
            coordinates[i] = coordinate;
        }

        let source_file = self
            .current_entry
            .as_ref()
            .unwrap()
            .file_name()
            .to_string_lossy()
            .to_string();

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
            source_file,
            difficulty: match elements.get(9) {
                Some(i) => i.parse::<i32>().unwrap() != 0,
                None => false,
            },
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
        if let Some(entry) = &self.current_entry {
            println!("{:?}", entry.file_name());
        }

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
