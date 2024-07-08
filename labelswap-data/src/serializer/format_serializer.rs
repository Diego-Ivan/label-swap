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
    fn init(&mut self, path: impl Into<PathBuf>) -> Result<()>;
    fn push(&mut self, annotation: Annotation) -> Result<()>;
    fn finish(&mut self) -> Result<()>;
}