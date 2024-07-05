/* transform.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::models::{Annotation, Format};
use anyhow::Result;

pub trait Transform {
    fn apply(&mut self, annotation: &mut Annotation, source_format: &Format, target_format: &Format) -> Result<()>;
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum RequiredTransformations {
    MapToName,
    MapToId,
    Normalize,
    Denormalize,
    LookupImage,
}