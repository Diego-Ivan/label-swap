/* format_serializer.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::path::PathBuf;
use crate::models::Annotation;

use super::SerializerResult;

pub trait FormatSerializer {
    /// Initializes the resources of self. This function must be called first
    fn init(&mut self, path: impl Into<PathBuf>) -> SerializerResult<()>;
    // Adds an annotation to self
    fn push(&mut self, annotation: Annotation) -> SerializerResult<()>;
    /// Gives up ownership and completes the serialization of self.
    fn finish(self) -> SerializerResult<()>;
}
