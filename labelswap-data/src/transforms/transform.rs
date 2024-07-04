/* transform.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::models::{Annotation, Format};

pub trait Transform {
    fn apply(&self, annotation: &mut Annotation, source_format: &Format, target_format: &Format);
}

#[derive(PartialEq, Eq, Hash)]
pub enum RequiredTransformations {
    MapToName,
    MapToId,
    Normalize,
    Denormalize,
    LookupImage,
}