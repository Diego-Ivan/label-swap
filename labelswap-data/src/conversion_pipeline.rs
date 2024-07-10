/* conversion_pipeline.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::transforms::*;
use crate::models::Format;
use std::collections::HashMap;
use std::path::PathBuf;
use std::cell::Cell;

use anyhow::{anyhow, Result};

pub struct ConversionPipeline<'a> {
    transforms: Vec<Box<dyn Transform>>,
    source_format: &'a Format,
    target_format: &'a Format,
    image_directory: Option<PathBuf>,
    mapping: Cell<Option<HashMap<String, String>>>,
}

impl<'a> ConversionPipeline<'a> {
    pub fn new(source_format: &'a Format, target_format: &'a Format) -> Self {
        Self {
            transforms: Vec::new(),
            source_format,
            target_format,
            image_directory: None,
            mapping: Cell::new(None),
        }
    }

    pub fn image_directory(&self) -> &Option<PathBuf> {
        &self.image_directory
    }

    pub fn set_image_directory(&mut self, image_directory: Option<PathBuf>) {
        self.image_directory = image_directory;
    }

    pub fn set_mapping(&mut self, mapping: Option<HashMap<String, String>>) {
        self.mapping.set(mapping);
    }

    pub fn convert(&mut self) -> Result<()> {
        self.configure()?;
        todo!();
    }

    fn configure(&mut self) -> Result<()> {
        let transformations = self.source_format.check_compatibility(&self.target_format);
        if transformations.contains(&RequiredTransformations::Denormalize) {
            match self.image_directory.as_ref() {
                Some(image_directory) => {
                    let denorm = Denormalize::new(PathBuf::from(image_directory))?;
                    self.transforms.push(Box::new(denorm));
                },
                None => return Err(anyhow!("Expected image directory to be Some"))
            }
        } else if transformations.contains(&RequiredTransformations::Normalize) {
            match self.image_directory.as_ref() {
                Some(image_directory) => {
                    let norm = Normalize::new(PathBuf::from(image_directory))?;
                    self.transforms.push(Box::new(norm));
                },
                None => return Err(anyhow!("Expected image directory to be Some"))
            }
        }

        if transformations.contains(&RequiredTransformations::LookupImage) {
            match self.image_directory.as_ref() {
                Some(image_directory) => {
                    let lookup = LookupImage::new(PathBuf::from(image_directory))?;
                    self.transforms.push(Box::new(lookup));
                }
                None => return Err(anyhow!("Expected image directory to be Some"))
            }
        }

        if transformations.contains(&RequiredTransformations::MapToId) || transformations.contains(&RequiredTransformations::MapToName) {
            match self.mapping.take() {
                Some(map) => {
                    let map = ClassMapping::new(map);
                    self.transforms.push(Box::new(map));
                },
                None => return Err(anyhow!("Expected mapping in the transformations")),
            }
        }

        Ok(())
    }
}
