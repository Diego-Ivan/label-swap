/* format_registry.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::collections::HashMap;

use labelswap_data::{
    models::{
        format::{ClassFormat, ClassMapping, ImagePath, SourceType},
        Format,
    },
    parser::*,
    serializer::*,
};

pub struct FormatRegistry {
    formats: HashMap<String, Format>,
}

impl FormatRegistry {
    pub fn new() -> Self {
        Self {
            formats: Self::init_table(),
        }
    }

    fn init_table() -> HashMap<String, Format> {
        let mut table = HashMap::new();
        table.insert(
            String::from("yolo5obb"),
            Format {
                name: String::from("YOLO v5 Oriented Bounding Boxes"),
                id: String::from("yolo5obb"),
                file_extension: None,
                is_normalized: false,
                image_path: ImagePath::NoPath,
                class_mapping: ClassMapping::NoMapping,
                class_format: ClassFormat::Name,
                source_type: SourceType::MultipleFiles,
            },
        );
        table.insert(
            String::from("cocojson"),
            Format {
                name: String::from("COCO JSON"),
                id: String::from("cocojson"),
                file_extension: Some(String::from("json")),
                is_normalized: false,
                image_path: ImagePath::ContainsPath,
                class_mapping: ClassMapping::ContainsMapping,
                class_format: ClassFormat::Both,
                source_type: SourceType::SingleFile,
            },
        );
        table
    }

    pub fn lookup_format(&self, id: &str) -> Option<&Format> {
        self.formats.get(id)
    }

    pub fn get_format_parser(&self, id: &str) -> Option<impl FormatParser> {
        match self.formats.get(id) {
            Some(format) => format,
            None => return None,
        };

        match id {
            "yolo5obb" => Some(Yolo5ObbParser::new()),
            _ => None,
        }
    }

    pub fn get_format_serializer(&self, id: &str) -> Option<impl FormatSerializer> {
        match self.formats.get(id) {
            Some(format) => format,
            None => return None,
        };

        match id {
            "yolo5obb" => Some(Yolo5ObbSerializer::new()),
            _ => None,
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Format> {
        self.formats.iter()
    }
}
