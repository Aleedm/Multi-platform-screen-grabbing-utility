use gtk::{glib, subclass::prelude::*};
use gtk4 as gtk;
#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/org/mpsgu/edit_menu_bar.ui")]
pub struct EditMenuBar {
    #[template_child]
    pub save: TemplateChild<gtk::Button>,
    #[template_child]
    pub copy: TemplateChild<gtk::Button>
}

#[glib::object_subclass]
impl ObjectSubclass for EditMenuBar {
    const NAME: &'static str = "EditMenuBar";
    type Type = super::EditMenuBar;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
// Trait shared by all GObjects
impl ObjectImpl for EditMenuBar {}

// Trait shared by all widgets
impl WidgetImpl for EditMenuBar {}

// Trait shared by all boxes
impl BoxImpl for EditMenuBar {}