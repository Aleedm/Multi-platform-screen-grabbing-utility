use gtk::{glib, subclass::prelude::*};
use gtk4 as gtk;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/org/mpsgu/settings.ui")]
pub struct SettingsModal {
    #[template_child]
    pub some_widget: TemplateChild<gtk::Button>,
    
}

#[glib::object_subclass]
impl ObjectSubclass for SettingsModal {
    const NAME: &'static str = "SettingsModal";
    type Type = super::SettingsModal;
    type ParentType = gtk::Dialog;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
// Trait shared by all GObjects
impl ObjectImpl for SettingsModal {}

// Trait shared by all widgets
impl WidgetImpl for SettingsModal {}

impl WindowImpl for SettingsModal {}

impl DialogImpl for SettingsModal {}