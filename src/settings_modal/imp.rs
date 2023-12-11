use gtk::{glib, subclass::prelude::*};
use gtk4 as gtk;
use crate::settings_manager::Settings;
use std::cell::RefCell;
#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/org/mpsgu/settings.ui")]
pub struct SettingsModal {
    #[template_child]
    pub shortcut_entry: TemplateChild<gtk::Entry>,
    #[template_child]
    pub edit_shortcut_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub edit_ss: TemplateChild<gtk::Box>,
    #[template_child]
    pub directory_entry: TemplateChild<gtk::Entry>,
    #[template_child]
    pub edit_directory: TemplateChild<gtk::Button>,
    #[template_child]
    pub edit_dir: TemplateChild<gtk::Box>,

    pub current_shortcut: RefCell<String>, 
    pub current_directory: RefCell<String>  

}

/* impl Default for MainWindow {
    fn default() -> Self {
        Self {
            shortcut_entry: Default::default(),
            edit_shortcut_button: Default::default(),
            edit_ss: Default::default(),
            directory_entry: Default::default(),
            edit_directory: Default::default(),
            edit_dir: Default::default(),
            settings_manager: Rc::new(RefCell::new(false))
        }
    }
} */

#[glib::object_subclass]
impl ObjectSubclass for SettingsModal {
    const NAME: &'static str = "SettingsModal";
    type Type = super::SettingsModal;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}
// Trait shared by all GObjects
impl ObjectImpl for SettingsModal {
    fn constructed(&self) {
        self.obj().hide_buttons();
        self.obj().setup_entry();
        self.obj().setup_edit_shortcut_button();
        self.obj().setup_save_shortcut_button();
        self.obj().setup_discard_shortcut_button();
        self.obj().setup_edit_directory_button();
        self.obj().setup_save_directory_button();
        self.obj().setup_discard_directory_button();
    }
}

// Trait shared by all widgets
impl WidgetImpl for SettingsModal {}

impl WindowImpl for SettingsModal {}
impl ApplicationWindowImpl for SettingsModal {}