use gtk::{glib, subclass::prelude::*};
use gtk4 as gtk;
#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/org/mpsgu/edit_shortcut.ui")]
pub struct EditShortCut {
    #[template_child]
    pub edit: TemplateChild<gtk::Button>
    // #[template_child]
    // pub save: TemplateChild<gtk::Button>,
    // #[template_child]
    // pub discard: TemplateChild<gtk::Button>
}

#[glib::object_subclass]
impl ObjectSubclass for EditShortCut {
    const NAME: &'static str = "EditShortCut";
    type Type = super::EditShortCut;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
// Trait shared by all GObjects
impl ObjectImpl for EditShortCut {}

// Trait shared by all widgets
impl WidgetImpl for EditShortCut {}

// Trait shared by all boxes
impl BoxImpl for EditShortCut {}