use gtk::{glib, subclass::prelude::*};
use gtk4 as gtk;
#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/org/mpsgu/crop_menu_bar.ui")]
pub struct CropMenuBar {
    #[template_child]
    pub confirm: TemplateChild<gtk::Button>,
    #[template_child]
    pub exit: TemplateChild<gtk::Button>
}

#[glib::object_subclass]
impl ObjectSubclass for CropMenuBar {
    const NAME: &'static str = "CropMenuBar";
    type Type = super::CropMenuBar;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
// Trait shared by all GObjects
impl ObjectImpl for CropMenuBar {}

// Trait shared by all widgets
impl WidgetImpl for CropMenuBar {}

// Trait shared by all boxes
impl BoxImpl for CropMenuBar {}