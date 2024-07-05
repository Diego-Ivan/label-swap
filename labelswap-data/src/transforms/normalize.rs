/* normalize.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::{
    models::{Annotation, Format},
    resolve_relative_path,
};
use anyhow::{anyhow, Result};
use image::io::Reader as ImageReader;
use std::path::PathBuf;

use super::Transform;

pub struct Normalize {
    image_directory: PathBuf,
}

impl Normalize {
    pub fn new(image_directory: PathBuf) -> Result<Self> {
        if !image_directory.is_dir() {
            return Err(anyhow!("Expected {:?} to be a directory", image_directory));
        }
        Ok(Self { image_directory })
    }
}

impl Transform for Normalize {
    fn apply(
        &mut self,
        annotation: &mut Annotation,
        _source_format: &Format,
        _target_format: &Format,
    ) -> Result<()> {
        let image_path = annotation
            .image
            .as_ref()
            .ok_or(anyhow!("Expected image path in annotation"))?;

        let image_path = resolve_relative_path(&self.image_directory, &image_path)?;
        let (width, height) = ImageReader::open(&image_path)?.into_dimensions()?;

        let width = f64::from(width);
        let height = f64::from(height);

        annotation.x1 /= width;
        annotation.x2 /= width;
        annotation.x3 /= width;
        annotation.x4 /= width;

        annotation.y1 /= height;
        annotation.y2 /= height;
        annotation.y3 /= height;
        annotation.y4 /= height;

        Ok(())
    }
}

pub struct Denormalize {
    image_directory: PathBuf,
}

impl Denormalize {
    pub fn new(image_directory: PathBuf) -> Result<Self> {
        if !image_directory.is_dir() {
            return Err(anyhow!("Expected {:?} to be a directory", image_directory));
        }
        Ok(Self { image_directory })
    }
}

impl Transform for Denormalize {
    fn apply(
        &mut self,
        annotation: &mut Annotation,
        _source_format: &Format,
        _target_format: &Format,
    ) -> Result<()> {
        let image_path = annotation
            .image
            .as_ref()
            .ok_or(anyhow!("Expected image path in annotation"))?;

        let image_path = resolve_relative_path(&self.image_directory, &image_path)?;
        let (width, height) = ImageReader::open(&image_path)?.into_dimensions()?;

        let width = f64::from(width);
        let height = f64::from(height);

        annotation.x1 *= width;
        annotation.x2 *= width;
        annotation.x3 *= width;
        annotation.x4 *= width;

        annotation.y1 *= height;
        annotation.y2 *= height;
        annotation.y3 *= height;
        annotation.y4 *= height;

        Ok(())
    }
}
