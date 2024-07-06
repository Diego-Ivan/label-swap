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

use anyhow::{anyhow, Result};

pub struct ConversionPipeline {
    transforms: Vec<Box<dyn Transform>>,
    source_format: Format,
    target_format: Format,
    image_directory: Option<PathBuf>,
    mapping: Option<HashMap<String, String>>,
}

impl ConversionPipeline {
    pub fn new(source_format: Format, target_format: Format) -> Self {
        Self {
            transforms: Vec::new(),
            source_format,
            target_format,
            image_directory: None,
            mapping: None,
        }
    }

    pub fn image_directory(&self) -> &Option<PathBuf> {
        &self.image_directory
    }

    pub fn set_image_directory(&mut self, image_directory: Option<PathBuf>) {
        self.image_directory = image_directory;
    }

    pub fn mapping(&self) -> &Option<HashMap<String, String>> {
        &self.mapping
    }

    pub fn set_mapping(&mut self, mapping: Option<HashMap<String, String>>) {
        self.mapping = mapping;
    }

    pub fn convert(&mut self) -> Result<()> {
        self.configure()?;
        todo!();
    }

    fn configure(&mut self) -> Result<()> {
        let transformations = self.source_format.check_compatibility(&self.target_format);
        if transformations.contains(&RequiredTransformations::Denormalize) {
            if self.image_directory.is_none() {
                return Err(anyhow!("Expected image directory to be Some"))
            }
            let image_directory = self.image_directory.as_ref().unwrap().clone();
            let denorm = Box::new(Denormalize::new(image_directory)?);
            self.transforms.push(denorm);
        } else if transformations.contains(&RequiredTransformations::Normalize) {
            if self.image_directory.is_none() {
                return Err(anyhow!("Expected image directory to be Some"))
            }
            let image_directory = self.image_directory.as_ref().unwrap().clone();
            let norm = Box::new(Normalize::new(image_directory)?);
            self.transforms.push(norm);
        }

        if transformations.contains(&RequiredTransformations::LookupImage) {
            if self.image_directory.is_none() {
                return Err(anyhow!("Expected image directory to be Some"))
            }
            let image_directory = self.image_directory.as_ref().unwrap().clone();

            let lookup = LookupImage::new(image_directory)?;
            self.transforms.push(Box::new(lookup));
        }

        if transformations.contains(&RequiredTransformations::MapToId) || transformations.contains(&RequiredTransformations::MapToName) {
            if self.mapping.is_none() {
                return Err(anyhow!("Expected mapping in the transformations"));
            }
            let map = ClassMapping::new(self.mapping.as_ref().unwrap().clone());
            self.transforms.push(Box::new(map));
        }

        Ok(())
    }
}
