mod imp;
use gtk4 as gtk;
use gtk::glib::Propagation;
use gtk::{
    gio,
    glib,
    prelude::*,
    //glib::{self, subclass::types::ObjectSubclassIsExt, Propagation},
    prelude::{WidgetExt, ButtonExt},
    EventControllerKey,
    subclass::prelude::ObjectSubclassIsExt
};

glib::wrapper! {
    pub struct SettingsModal(ObjectSubclass<imp::SettingsModal>)
        @extends gtk::Widget, gtk::Window,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl SettingsModal {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }
    pub fn setup_entry(&self) {
        let entry = self.imp().text.clone();
        entry.set_visible(true);
        entry.set_can_focus(true);
        let key_controller = EventControllerKey::new();
        key_controller.connect_key_pressed(|_, keyval, _, _| {
            let keyname = keyval.name().unwrap();
            println!("Key Pressed: {}", keyname);
            Propagation::Proceed
        });
        entry.add_controller(key_controller);
    }
    pub fn setup_cancel_button(&self) {
        let cancel = self.imp().button_cancel.clone();
        cancel.connect_clicked(|_| {
            println!("BUTTON CLICKED");
    });
    }
}
