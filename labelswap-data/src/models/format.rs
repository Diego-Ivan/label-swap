/* format.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::collections::HashSet;

use crate::transforms::transform::RequiredTransformations;

#[derive(PartialEq)]
pub enum ClassFormat {
    Name,
    Id,
    Both,
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

impl ClassMapping {
    #[inline]
    pub fn has_mapping(&self) -> bool {
        match *self {
            Self::ContainsMapping => true,
            Self::NoMapping => false,
        }
    }
}

#[derive(PartialEq, Eq)]
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

impl Format {
    pub fn check_compatibility(&self, other: &Format) -> HashSet<RequiredTransformations> {
        let mut transformations = HashSet::new();

        if self.is_normalized && !other.is_normalized {
            transformations.insert(RequiredTransformations::Denormalize);
        } else if !self.is_normalized && other.is_normalized {
            transformations.insert(RequiredTransformations::Normalize);
        }

        if !self.class_mapping.has_mapping() {
            let mapping = match (&self.class_format, &other.class_format) {
                (ClassFormat::Name, ClassFormat::Id) => Some(RequiredTransformations::MapToId),
                (ClassFormat::Name, ClassFormat::Both) => Some(RequiredTransformations::MapToId),
                (ClassFormat::Id, ClassFormat::Name) => Some(RequiredTransformations::MapToName),
                (ClassFormat::Id, ClassFormat::Both) => Some(RequiredTransformations::MapToName),
                _ => None,
            };
            if let Some(mapping) = mapping {
                transformations.insert(mapping);
            }
        }

        if self.image_path == ImagePath::NoPath && other.image_path == ImagePath::ContainsPath {
            transformations.insert(RequiredTransformations::LookupImage);
        }

        transformations
    }
}
