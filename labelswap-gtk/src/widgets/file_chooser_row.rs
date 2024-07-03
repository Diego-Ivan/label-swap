/* file_chooser_row.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::{prelude::ActionRowExt, subclass::prelude::*};
use gtk::{
    gio,
    glib::{self},
    prelude::*,
};

#[derive(Default, Clone, Debug, Copy, Eq, PartialEq, glib::Enum)]
#[enum_type(name = "FileSource")]
pub enum FileSource {
    #[default]
    File,
    Folder,
}

mod imp {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::LabelSwapFileChooserRow)]
    #[template(resource = "/io/github/diegoivan/label_swap/ui/file-chooser-row.ui")]
    pub struct LabelSwapFileChooserRow {
        #[property(get, set, nullable)]
        pub (super) file: RefCell<Option<gio::File>>,
        #[property(get, set, builder(Default::default()))]
        pub (super) file_type: Cell<FileSource>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LabelSwapFileChooserRow {
        const NAME: &'static str = "LabelSwapFileChooserRow";
        type Type = super::LabelSwapFileChooserRow;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.install_action_async("chooser-row.choose", None, |row, _, _| async move {
                row.choose().await;
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for LabelSwapFileChooserRow {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
    impl WidgetImpl for LabelSwapFileChooserRow {}
    impl ActionRowImpl for LabelSwapFileChooserRow {}
    impl PreferencesRowImpl for LabelSwapFileChooserRow {}
    impl ListBoxRowImpl for LabelSwapFileChooserRow {}
}

glib::wrapper! {
    pub struct LabelSwapFileChooserRow(ObjectSubclass<imp::LabelSwapFileChooserRow>)
        @extends gtk::Widget, adw::ActionRow, adw::PreferencesRow;
}

impl LabelSwapFileChooserRow {
    pub async fn choose(&self) {
        let file_dialog = gtk::FileDialog::new();
        let root = self.root().and_downcast::<gtk::Window>().unwrap();

        let file = match self.file_type() {
            FileSource::File => file_dialog.open_future(Some(&root)).await,
            FileSource::Folder => file_dialog.select_folder_future(Some(&root)).await,
        };

        match file {
            Ok(file) => {
                let mut file_name = String::from(file.basename().unwrap().to_str().unwrap());
                if file.query_file_type(gio::FileQueryInfoFlags::NOFOLLOW_SYMLINKS, gio::Cancellable::NONE) == gio::FileType::Directory {
                    file_name.push_str("/");
                }
                self.set_subtitle(&file_name);
                self.set_file(Some(file));
            }
            Err(e) => eprintln!("{e}"),
        }
    }
}