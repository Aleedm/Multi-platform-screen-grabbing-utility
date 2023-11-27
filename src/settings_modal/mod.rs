mod imp;
use gtk4 as gtk;

use gtk::{
    gio,
    glib::{self, subclass::types::ObjectSubclassIsExt, Propagation},
    prelude::{WidgetExt, ActionMapExt},
    EventControllerKey,
};

glib::wrapper! {
    pub struct SettingsModal(ObjectSubclass<imp::SettingsModal>)
        @extends gtk::Widget, gtk::Window,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl SettingsModal {
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
        let cancel = gio::SimpleAction::new("cancel", None);

        cancel.connect_activate(move |_, _| {
            println!("Cancel button pressed");
        });

        self.add_action(&cancel);
    }
}
