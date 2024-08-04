/* cocojson_parser.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use super::{FormatParser, ParserError};
use crate::models::format::SourceType;
use crate::models::{annotation::ClassRepresentation, Annotation, Image};
use serde_json::value::Value;
use std::collections::{HashMap, VecDeque};
use std::{fs::File, io::BufReader, path::PathBuf};

pub struct CocoJsonParser {
    category_map: HashMap<i64, String>,
    image_map: HashMap<i64, PathBuf>,
    annotation_array: VecDeque<Value>,
}

impl CocoJsonParser {
    pub fn new() -> Self {
        Self {
            category_map: HashMap::new(),
            image_map: HashMap::new(),
            annotation_array: VecDeque::new(),
        }
    }
}

impl FormatParser for CocoJsonParser {
    fn init(&mut self, path: impl Into<PathBuf>) -> Result<(), ParserError> {
        let path: PathBuf = path.into();
        let metadata = std::fs::metadata(&path);
        if metadata.is_ok() {
            return Err(ParserError::WrongSource {
                expected: SourceType::SingleFile,
                found: SourceType::MultipleFiles,
            });
        }
        if let Some(extension) = path.extension() {
            if extension != "json" {
                return Err(ParserError::WrongFileType {
                    expected: String::from("JSON"),
                    found: extension.to_string_lossy().into(),
                });
            }
        }

        let buf_reader = BufReader::new(File::open(&path)?);

        let mut map: serde_json::Map<String, Value> = match serde_json::from_reader(buf_reader) {
            Ok(map) => map,
            Err(e) => return Err(ParserError::Other(e.to_string())),
        };

        self.annotation_array = match map.remove("annotations").ok_or(ParserError::WrongFormat(
            "Expected 'annotations' field in main object".into(),
        ))? {
            Value::Array(array) => array.into(),
            _ => {
                return Err(ParserError::WrongFormat(
                    "JSON: Expected 'annotations' to be an array".into(),
                ))
            }
        };

        let category_array = match map.remove("categories").ok_or(ParserError::WrongFormat(
            "Expected 'annotations' field in main object".into(),
        ))? {
            Value::Array(array) => array,
            _ => {
                return Err(ParserError::WrongFormat(
                    "JSON: Expected 'annotations' to be an array".into(),
                ))
            }
        };

        self.parse_category_array(&category_array)?;

        let image_array = match map.get("images").ok_or(ParserError::WrongFormat(
            "Expected 'annotations' field in main object".into(),
        ))? {
            Value::Array(array) => array,
            _ => {
                return Err(ParserError::WrongFormat(
                    "JSON: Expected 'annotations' to be an array".into(),
                ))
            }
        };

        self.parse_image_map(&image_array)?;

        Ok(())
    }
    fn get_next(&mut self) -> Result<Annotation, ParserError> {
        let current_item = match self.annotation_array.pop_front() {
            Some(i) => i,
            None => return Err(ParserError::OutOfElements),
        };
        let current_item = self.annotation_array.pop_front().unwrap();

        let mut map = match current_item {
            Value::Object(map) => map,
            _ => {
                return Err(ParserError::WrongFormat(
                    "Expected elements in annotation array to be a dictionaries".into(),
                ))
            }
        };

        let category_id = match map.remove("category_id").ok_or(ParserError::WrongFormat(
            "Expected category_id element".into(),
        ))? {
            Value::Number(num) => num.as_i64().unwrap(),
            _ => {
                return Err(ParserError::WrongFormat(
                    "Expected category_id element to be a number".into(),
                ))
            }
        };
        let category_name = self
            .category_map
            .get(&category_id)
            .ok_or(ParserError::WrongFormat(format!(
                "Category id {category_id} not found in category map"
            )))?
            .clone();

        let image = match map
            .get("image_id")
            .ok_or(ParserError::WrongFormat("Expected image_id element".into()))?
        {
            Value::Number(num) => num.as_i64().unwrap(),
            _ => {
                return Err(ParserError::WrongFormat(
                    "Expected image_id element to be a number".into(),
                ))
            }
        };

        let image = self
            .image_map
            .get(&image)
            .ok_or(ParserError::WrongFormat(
                "Image id {image} not found in image map".into(),
            ))?
            .clone();

        let bbox = match map
            .remove("bbox")
            .ok_or(ParserError::WrongFormat("Expected bbox array".into()))?
        {
            Value::Array(bbox) => bbox,
            _ => {
                return Err(ParserError::WrongFormat(
                    "Expected bbox to be an array".into(),
                ))
            }
        };

        let (x, y, width, height) = Self::parse_bbox_array(&bbox)?;
        let class = ClassRepresentation::Both {
            name: category_name,
            id: category_id.to_string(),
        };

        Ok(Annotation {
            class,
            image: Image::new_with_path(image),
            ..Annotation::from_top_left_corner(x, y, width, height)
        })
    }

    fn has_next(&mut self) -> bool {
        self.annotation_array.len() > 0
    }
}

impl CocoJsonParser {
    fn parse_category_array(&mut self, array: &[Value]) -> Result<(), ParserError> {
        for value in array {
            let image_object = match value {
                Value::Object(map) => map,
                _ => {
                    return Err(ParserError::WrongFormat(
                        "Wrong format, expected element to be a map".into(),
                    ))
                }
            };

            let id = match image_object.get("id") {
                Some(id) => id,
                None => {
                    return Err(ParserError::WrongFormat(
                        "Expected id member to exist".into(),
                    ))
                }
            };

            let id = match id {
                Value::Number(num) => num.as_i64().ok_or(ParserError::WrongFormat(
                    "Expected id to be an integer".into(),
                ))?,
                _ => {
                    return Err(ParserError::WrongFormat(
                        "Expected id to be a number".into(),
                    ))
                }
            };

            let category_name = image_object
                .get("name")
                .ok_or(ParserError::WrongFormat("Expected name member".into()))?;

            let category_name = match category_name {
                Value::String(string) => string,
                _ => {
                    return Err(ParserError::WrongFormat(
                        "Expected name to be a string".into(),
                    ))
                }
            };

            self.category_map.insert(id, category_name.clone());
        }

        Ok(())
    }

    fn parse_image_map(&mut self, array: &[Value]) -> Result<(), ParserError> {
        for value in array {
            let image_object = match value {
                Value::Object(map) => map,
                _ => {
                    return Err(ParserError::WrongFormat(
                        "Wrong format, expected element to be a map".into(),
                    ))
                }
            };

            let id = image_object.get("id").ok_or(ParserError::WrongFormat(
                "Expected id member to exist".into(),
            ))?;

            let id = match id {
                Value::Number(num) => num.as_i64().ok_or(ParserError::WrongFormat(
                    "Expected id to be an integer".into(),
                ))?,
                _ => {
                    return Err(ParserError::WrongFormat(
                        "Expected id to be a number".into(),
                    ))
                }
            };

            let file_name = image_object
                .get("file_name")
                .ok_or(ParserError::WrongFormat("Expected filename member".into()))?;

            let filename = match file_name {
                Value::String(string) => string,
                _ => {
                    return Err(ParserError::WrongFormat(
                        "Expected file_name to be a string".into(),
                    ))
                }
            };

            self.image_map.insert(id, PathBuf::from(filename));
        }

        Ok(())
    }

    fn parse_bbox_array(array: &[Value]) -> Result<(f64, f64, f64, f64), ParserError> {
        let array: Vec<f64> = array
            .iter()
            .filter_map(|value| {
                if let Value::Number(num) = value {
                    num.as_f64()
                } else {
                    None
                }
            })
            .collect();

        if array.len() != 4 {
            return Err(ParserError::WrongFormat(format!(
                "Expected four valid elements in bbox array, got {}",
                array.len()
            )));
        }

        Ok((array[0], array[1], array[2], array[3]))
    }
}
