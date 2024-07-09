/* format_object.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */


use gtk::{
    glib::{self, once_cell::sync::Lazy, ParamSpecBuilderExt, value::Value}, subclass::prelude::*
};

use labelswap_data::models::Format;

#[derive(Debug, Default)]
pub struct FormatObjectPriv {
    pub(super) format: Option<Format>,
}

#[glib::object_subclass]
impl ObjectSubclass for FormatObjectPriv {
    const NAME: &'static str = "LabelSwapFormatObject";
    type Type = FormatObject;
    type ParentType = glib::Object;
}

glib::wrapper! {
    pub struct FormatObject(ObjectSubclass<FormatObjectPriv>);
}

impl ObjectImpl for FormatObjectPriv {
    fn constructed(&self) {
        self.parent_constructed();
    }

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![glib::ParamSpecString::builder("name")
                .read_only()
                .build()]
        });
        PROPERTIES.as_ref()
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "name" => Value::from(&self.format.as_ref().unwrap().name),
            _ => unimplemented!()
        }
    }
}


impl FormatObject {
    pub fn something(&self) {
        self.name();
    }

    pub fn name(&self) -> &str {
        &self.imp().format.as_ref().unwrap().name
    }

    pub fn get_format(&self) -> &Format {
        self.imp().format.as_ref().unwrap()
    }
}