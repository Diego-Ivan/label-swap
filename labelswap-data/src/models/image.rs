/* image.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::path::PathBuf;

/// Image: a structure that represents an image in memory
/// 
/// The image struct optionally holds the dimensions of the image
/// as well as its location in a file system, in case it has to be
/// opened to obtain them (usually for the purposes of normalization/
/// denormalization)
/// 
pub struct Image {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub path: Option<PathBuf>,
}

impl Image {
    pub fn new() -> Self {
        Self { 
            width: None,
            height: None,
            path: None,
        }
    }

    pub fn new_with_dimensions(width: u32, height: u32) -> Self {
        Self {
            path: None,
            width: Some(width),
            height: Some(height)
        }
    }

    pub fn new_with_path(path: impl Into<PathBuf>) -> Self {
        Self {
            path: Some(path.into()),
            width: None,
            height: None,
        }
    }
}