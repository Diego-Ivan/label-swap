/* lookup_image.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

use super::Transform;
use crate::models::{Annotation, Format};
use anyhow::{anyhow, Result};

pub struct LookupImage {
    pub image_directory: PathBuf,
    source_to_image_map: HashMap<String, String>,
    sources_without_image: HashSet<String>,
}

impl LookupImage {
    pub fn new(image_directory: PathBuf) -> Result<Self> {
        if !image_directory.is_dir() {
            return Err(anyhow!("Expected image directory, got regular file"));
        }

        Ok(Self {
            image_directory,
            sources_without_image: HashSet::new(),
            source_to_image_map: HashMap::new(),
        })
    }
}

impl Transform for LookupImage {
    fn apply(
        &mut self,
        annotation: &mut Annotation,
        _source_format: &Format,
        _target_format: &Format,
    ) -> Result<()> {
        let annotation_source = annotation
            .source_file
            .as_ref()
            .ok_or(anyhow!("Expected source file to be Some"))?;

        if let Some(image) = self.source_to_image_map.get(annotation_source) {
            annotation.image = Some(Path::new(image).into());
            return Ok(());
        }

        if self.sources_without_image.contains(annotation_source) {
            return Err(anyhow!(
                "Annotation source does not have an image related to it"
            ));
        }

        let dir_entry = self
            .image_directory
            .read_dir()?
            .filter_map(|file| file.ok())
            .find(|file| {
                file.file_name()
                    .to_str()
                    .unwrap()
                    .starts_with(annotation_source)
            });

        return match dir_entry {
            Some(file) => {
                let filename = String::from(file.file_name().to_str().unwrap());
                self.source_to_image_map
                    .insert(annotation_source.clone(), filename);
                annotation.image = Some(file.path().into());
                Ok(())
            }
            None => {
                self.sources_without_image.insert(annotation_source.clone());
                Err(anyhow!("Could not find an image for {annotation_source}"))
            }
        };
    }
}
