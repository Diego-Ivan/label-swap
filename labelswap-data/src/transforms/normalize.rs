/* normalize.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::{
    models::{Annotation, Format, Image},
    resolve_relative_path,
};
use anyhow::{anyhow, Result};
use image::io::Reader as ImageReader;
use std::path::{Path, PathBuf};

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

    fn normalize(annotation: &mut Annotation, width: u32, height: u32) {
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
    }
}

impl Transform for Normalize {
    fn apply(
        &mut self,
        annotation: &mut Annotation,
        _source_format: &Format,
        _target_format: &Format,
    ) -> Result<()> {
        let mut image = annotation
            .image
            .as_mut()
            .ok_or(anyhow!("Expected image path in annotation"))?;

        let (width, height) = match (image.width, image.height) {
            (Some(width), Some(height)) => (width, height),
            _ => read_image_dimensions(&mut image, &self.image_directory)?,
        };

        Self::normalize(annotation, width, height);

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

    fn denormalize(annotation: &mut Annotation, image_directory: &Path, width: u32, height: u32) {
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
    }
}

impl Transform for Denormalize {
    fn apply(
        &mut self,
        annotation: &mut Annotation,
        _source_format: &Format,
        _target_format: &Format,
    ) -> Result<()> {
        let mut image = annotation
            .image
            .as_mut()
            .ok_or(anyhow!("Expected image path in annotation"))?;

        let (width, height) = match (image.width, image.height) {
            (Some(width), Some(height)) => (width, height),
            _ => read_image_dimensions(&mut image, &self.image_directory)?,
        };

        Self::denormalize(annotation, &self.image_directory, width, height);

        Ok(())
    }
}

/// Reads the image dimensions and writes them to the instance
fn read_image_dimensions(image: &mut Image, image_directory: &Path) -> Result<(u32, u32)> {
    let image_path = match image.path.as_ref() {
        Some(path) => path,
        None => return Err(anyhow!("Expected image path to be Some")),
    };

    let image_path = resolve_relative_path(image_directory, image_path)?;
    let (width, height) = ImageReader::open(&image_path)?.into_dimensions()?;

    image.width = Some(width);
    image.height = Some(height);

    Ok((width, height))
}