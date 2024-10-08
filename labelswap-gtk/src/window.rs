/* window.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::widgets;
//use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use labelswap_data::models::{format::{ClassFormat, SourceType}, Format};

    use crate::models::FormatObject;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/diegoivan/label_swap/ui/window.ui")]
    pub struct LabelSwapWindow {
        #[template_child]
        pub source_row: TemplateChild<widgets::LabelSwapFileChooserRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LabelSwapWindow {
        const NAME: &'static str = "LabelSwapWindow";
        type Type = super::LabelSwapWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LabelSwapWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let formats = [
                Format {
                    name: String::from("YOLO v5 Oriented Bounding Boxes"),
                    id: String::from("yolo5obb"),
                    file_extension: None,
                    is_normalized: false,
                    image_path: labelswap_data::models::format::ImagePath::NoPath,
                    class_mapping: labelswap_data::models::format::ClassMapping::NoMapping,
                    class_format: ClassFormat::Name,
                    source_type: SourceType::SingleFile,
                }
            ];
        }
    }
    impl WidgetImpl for LabelSwapWindow {}
    impl WindowImpl for LabelSwapWindow {}
    impl ApplicationWindowImpl for LabelSwapWindow {}
    impl AdwApplicationWindowImpl for LabelSwapWindow {}
}

glib::wrapper! {
    pub struct LabelSwapWindow(ObjectSubclass<imp::LabelSwapWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl LabelSwapWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
