/* cocojson_parser.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::models::{annotation::ClassRepresentation, Annotation};
use super::FormatParser;
use super::format_parser::ParserError;
use std::{clone, fs::File, io::BufReader, path::Path};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use serde_json::value::Value;

pub struct CocoJsonParser {
    category_map: Option<HashMap<i64, String>>,
    image_map: Option<HashMap<i64, String>>,
    annotation_array: Option<Vec<Value>>,
}

impl CocoJsonParser {
    pub fn new() -> Self {
        Self {
            category_map: None,
            image_map: None,
            annotation_array: None,
        }
    }
}

impl FormatParser for CocoJsonParser {
    fn init(&mut self, path: Box<Path>) -> Result<()> {
        let metadata = std::fs::metadata(&path)?;
        if metadata.file_type().is_dir() {
            return Err(anyhow!(ParserError::WrongFileType("Expected path to be a file".into())));
        }

        // if !path.ends_with("json") {
        //     return Err(anyhow!("Expected {:?} to be a JSON file", path));
        // }

        let buf_reader = BufReader::new(File::open(&path)?);
        
        let mut map: serde_json::Map<String, Value> = 
            serde_json::from_reader(buf_reader)?;
        
        let annotation_array = match map.remove("annotations").ok_or(anyhow!("Expected annotations"))? {
            Value::Array(array) => array,
            _ => return Err(anyhow!("Expected annotations to be an array"))
        };

        let category_array = match map.remove("categories").ok_or(anyhow!("Expected categories"))? {
            Value::Array(array) => array,
            _ => return Err(anyhow!("Expected categories to be an array")),
        };

        self.annotation_array = Some(annotation_array);
        self.category_map = Some(self.parse_category_array(&category_array)?);

        let image_array = match map.get("images").ok_or(anyhow!("Expected images"))? {
            Value::Array(array) => array,
            _ => return Err(anyhow!("Expected images to be an array")),
        };

        self.image_map = Some(self.parse_image_map(&image_array)?);

        Ok(())
    }

    fn get_next(&mut self) -> Result<Annotation> {
        let current_item = self.annotation_array.as_mut().unwrap().pop().unwrap();
        let category_map = self.category_map.as_ref().unwrap();
        let image_map = self.image_map.as_ref().unwrap();

        let mut map = match current_item {
            Value::Object(map) => map,
            _ => return Err(anyhow!("Expected value to be a dictionary")),
        };

        let category_id = match map.remove("category_id").ok_or(anyhow!("Expected category_id element"))? {
            Value::Number(num) => num.as_i64().unwrap(),
            _ => return Err(anyhow!("Expected category_id element to be a number"))
        };
        let category_name = category_map
            .get(&category_id)
            .ok_or(anyhow!("Category id {category_id} not found in category map"))?
            .clone();

        let image = match map.get("image_id").ok_or(anyhow!("Expected image_id element"))? {
            Value::Number(num) => num.as_i64().unwrap(),
            _ => return Err(anyhow!("Expected image_id element to be a number"))
        };

        let image = image_map
            .get(&image)
            .ok_or(anyhow!("Image id {image} not found in image map"))?
            .clone();

        let bbox = match map.remove("bbox").ok_or(anyhow!("Expected bbox array"))? {
            Value::Array(bbox) => bbox,
            _ => return Err(anyhow!("Expected bbox to be an array"))
        };

        let bbox = Self::parse_bbox_array(&bbox)?;
        let class = ClassRepresentation::Both(category_name, category_id.to_string());

        Ok(Annotation {
            class,
            image: Some(Path::new(&image).into()),
            ..Annotation::from_top_left_corner(bbox.0, bbox.1, bbox.2, bbox.3)
        })
    }

    fn has_next(&mut self) -> bool {
        self.annotation_array.as_ref().unwrap().len() != 0
    }
}

impl CocoJsonParser {
    fn parse_category_array(&self, array: &[Value]) -> Result<HashMap<i64, String>> {
        let mut table = HashMap::new();

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

            table.insert(id, category_name.clone());
        }

        Ok(table)
    }

    fn parse_image_map(&self, array: &[Value]) -> Result<HashMap<i64, String>>{
        let mut table = HashMap::new();

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

            table.insert(id, filename.clone());
        }

        Ok(table)
    }

    fn parse_bbox_array(array: &[Value]) -> Result<(f64, f64, f64, f64)> {
        let array = &array
            .iter()
            .filter_map(|value| {
                if let Value::Number(num) = value {
                    Some(num.as_f64().unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<f64>>();
        if array.len() == 4 {
            Ok((array[0], array[1], array[2], array[3]))
        } else {
            Err(anyhow!("Expected four valid elements in bbox array, got {}", array.len()))
        }
    }
}