/* format.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#[derive(PartialEq)]
pub enum ClassFormat {
    Name,
    Id,
    Both
}

#[derive(PartialEq)]
pub enum SourceType {
    MultipleFiles,
    SingleFile,
}

pub enum ClassMapping {
    ContainsMapping,
    NoMapping,
}

pub enum ImagePath {
    ContainsPath,
    NoPath,
}

pub struct Format {
    pub name: String,
    pub id: String,
    pub file_extension: Option<String>,
    pub is_normalized: bool,
    pub image_path: ImagePath,
    pub class_mapping: ClassMapping,
    pub class_format: ClassFormat,
    pub source_type: SourceType,
}