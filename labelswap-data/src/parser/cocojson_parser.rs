/* cocojson_parser.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use super::format_parser::ParserError;
use super::FormatParser;
use crate::models::{annotation::ClassRepresentation, Annotation, Image};
use anyhow::{anyhow, Result};
use serde_json::value::Value;
use std::collections::{HashMap, VecDeque};
use std::{fs::File, io::BufReader, path::{Path, PathBuf}};

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
    fn init(&mut self, path: impl Into<PathBuf>) -> Result<()> {
        let path = path.into();
        let metadata = std::fs::metadata(&path)?;
        if metadata.file_type().is_dir() {
            return Err(anyhow!(ParserError::WrongFileType(
                "Expected path to be a file".into()
            )));
        }

        if let Some(extension) = path.extension() {
            if extension != "json" {
                return Err(anyhow!("Expected {:?} to be a json file", path));
            }
        }

        let buf_reader = BufReader::new(File::open(&path)?);

        let mut map: serde_json::Map<String, Value> = serde_json::from_reader(buf_reader)?;

        self.annotation_array = match map
            .remove("annotations")
            .ok_or(anyhow!("Expected annotations"))?
        {
            Value::Array(array) => array.into(),
            _ => return Err(anyhow!("Expected annotations to be an array")),
        };

        let category_array = match map
            .remove("categories")
            .ok_or(anyhow!("Expected categories"))?
        {
            Value::Array(array) => array,
            _ => return Err(anyhow!("Expected categories to be an array")),
        };

        self.parse_category_array(&category_array)?;

        let image_array = match map.get("images").ok_or(anyhow!("Expected images"))? {
            Value::Array(array) => array,
            _ => return Err(anyhow!("Expected images to be an array")),
        };

        self.parse_image_map(&image_array)?;

        Ok(())
    }

    fn get_next(&mut self) -> Result<Annotation> {
        let current_item = self
            .annotation_array
            .pop_front()
            .ok_or(anyhow!("Expected value"))?;

        let mut map = match current_item {
            Value::Object(map) => map,
            _ => return Err(anyhow!("Expected value to be a dictionary")),
        };

        let category_id = match map
            .remove("category_id")
            .ok_or(anyhow!("Expected category_id element"))?
        {
            Value::Number(num) => num.as_i64().unwrap(),
            _ => return Err(anyhow!("Expected category_id element to be a number")),
        };
        let category_name = self.category_map
            .get(&category_id)
            .ok_or(anyhow!(
                "Category id {category_id} not found in category map"
            ))?
            .clone();

        let image = match map
            .get("image_id")
            .ok_or(anyhow!("Expected image_id element"))?
        {
            Value::Number(num) => num.as_i64().unwrap(),
            _ => return Err(anyhow!("Expected image_id element to be a number")),
        };

        let image = self.image_map
            .get(&image)
            .ok_or(anyhow!("Image id {image} not found in image map"))?
            .clone();

        let bbox = match map.remove("bbox").ok_or(anyhow!("Expected bbox array"))? {
            Value::Array(bbox) => bbox,
            _ => return Err(anyhow!("Expected bbox to be an array")),
        };

        let (x, y, width, height) = Self::parse_bbox_array(&bbox)?;
        let class = ClassRepresentation::Both {
            name: category_name,
            id: category_id.to_string(),
        };

        Ok(Annotation {
            class,
            image: Some(Image::new_with_path(image)),
            ..Annotation::from_top_left_corner(x, y, width, height)
        })
    }

    fn has_next(&mut self) -> bool {
        self.annotation_array.len() > 0
    }
}

impl CocoJsonParser {
    fn parse_category_array(&mut self, array: &[Value]) -> Result<()> {
        for value in array {
            let image_object = match value {
                Value::Object(map) => map,
                _ => return Err(anyhow!("Wrong format, expected element to be a map")),
            };

            let id = image_object
                .get("id")
                .ok_or(anyhow!("Expected id member to exist"))?;

            let id = match id {
                Value::Number(num) => num
                    .as_i64()
                    .ok_or(anyhow!("Expected id to be an integer"))?,
                _ => return Err(anyhow!("Expected id to be a number")),
            };

            let category_name = image_object
                .get("name")
                .ok_or(anyhow!("Expected name member"))?;

            let category_name = match category_name {
                Value::String(string) => string,
                _ => return Err(anyhow!("Expected name to be a string")),
            };

            self.category_map.insert(id, category_name.clone());
        }

        Ok(())
    }

    fn parse_image_map(&mut self, array: &[Value]) -> Result<()> {
        for value in array {
            let image_object = match value {
                Value::Object(map) => map,
                _ => return Err(anyhow!("Wrong format, expected element to be a map")),
            };

            let id = image_object
                .get("id")
                .ok_or(anyhow!("Expected id member to exist"))?;

            let id = match id {
                Value::Number(num) => num
                    .as_i64()
                    .ok_or(anyhow!("Expected id to be an integer"))?,
                _ => return Err(anyhow!("Expected id to be a number")),
            };

            let file_name = image_object
                .get("file_name")
                .ok_or(anyhow!("Expected filename member"))?;

            let filename = match file_name {
                Value::String(string) => string,
                _ => return Err(anyhow!("Expected file_name to be a string")),
            };

            self.image_map.insert(id, PathBuf::from(filename));
        }

        Ok(())
    }

    fn parse_bbox_array(array: &[Value]) -> Result<(f64, f64, f64, f64)> {
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
            return Err(anyhow!(
                "Expected four valid elements in bbox array, got {}",
                array.len()
            ));
        }

        Ok((array[0], array[1], array[2], array[3]))
    }
}
