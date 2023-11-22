use gtk::{glib, subclass::prelude::*, prelude::WidgetExt};
use gtk4 as gtk;
use std::cell::RefCell;

use crate::edit_menu_bar::EditMenuBar;
#[derive(Debug, gtk::CompositeTemplate)]
#[template(resource = "/org/mpsgu/first_menu_bar.ui")]
pub struct FirstMenuBar {
    #[template_child]
    pub add_ss: TemplateChild<gtk::Button>,
    #[template_child]
    pub delay_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub edit: TemplateChild<EditMenuBar>,
    pub delay: RefCell<u64>
}

impl Default for FirstMenuBar {
    fn default() -> Self {
        Self {
            add_ss: Default::default(),
            delay_label: Default::default(),
            edit: Default::default(),
            delay: RefCell::new(0),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for FirstMenuBar {
    const NAME: &'static str = "FirstMenuBar";
    type Type = super::FirstMenuBar;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
// Trait shared by all GObjects
impl ObjectImpl for FirstMenuBar {
    fn constructed(&self) {
        self.edit.hide();
    }
}

// Trait shared by all widgets
impl WidgetImpl for FirstMenuBar {}

// Trait shared by all boxes
impl BoxImpl for FirstMenuBar {}