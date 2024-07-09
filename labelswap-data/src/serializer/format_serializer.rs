/* format_serializer.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::path::PathBuf;
use anyhow::Result;
use crate::models::Annotation;

pub trait FormatSerializer {
    /// Initializes the resources of self. This function must be called first
    fn init(&mut self, path: impl Into<PathBuf>) -> Result<()>;
    // Adds an annotation to self
    fn push(&mut self, annotation: Annotation) -> Result<()>;
    /// Gives up ownership and completes the serialization of self.
    fn finish(self) -> Result<()>;
}