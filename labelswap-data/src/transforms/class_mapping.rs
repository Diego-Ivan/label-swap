/* class_mapping.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::collections::HashMap;

use super::Transform;
use crate::models::{annotation::ClassRepresentation::{self, Both,ClassId,ClassName}, Annotation, Format};
use anyhow::anyhow;

pub struct ClassMapping {
    map: HashMap<String, String>,
}

impl ClassMapping {
    pub fn new(map: HashMap<String, String>) -> Self {
        Self {
            map
        }
    }
}

impl Transform for ClassMapping {
    fn apply(
        &mut self,
        annotation: &mut Annotation,
        _source_format: &Format,
        _target_format: &Format,
    ) -> anyhow::Result<()> {
        /* 
         * Cloning is inexpensive here, as the values are usually small strings.
         */
        let new_format = match annotation.class.as_ref() {
            ClassName(name) => {
                if !self.map.contains_key(name) {
                    return Err(anyhow!("Class Name {name} does not appear on the annotation conversion map"));
                }
                Both { name: name.clone(), id: self.map.get(name).unwrap().clone() }
            },
            ClassId(id) => {
                if !self.map.contains_key(id) {
                    return Err(anyhow!("Class id {id} does not appear on the annotation conversion map"));
                }
                Both { name: self.map.get(id).unwrap().clone(), id: id.clone() }
            }
            Both {..} => return Ok(()),
            ClassRepresentation::None => return Err(anyhow!("Expected annotation to have a not none class representation")),
        };
        annotation.class = new_format;
        Ok(())
    }
}