mod imp;
use gtk::accelerator_name;
use gtk::glib::Propagation;
use gtk::{
    gio,
    glib,
    //glib::{self, subclass::types::ObjectSubclassIsExt, Propagation},
    prelude::WidgetExt,
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
    EventControllerKey,
};
use gtk4 as gtk;

glib::wrapper! {
    pub struct SettingsModal(ObjectSubclass<imp::SettingsModal>)
        @extends gtk::Widget, gtk::Window,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl SettingsModal {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    /* Function to set the save directory */
    pub fn set_current_directory(&self, directory: String) {
        *self.imp().current_directory.borrow_mut() = directory;
        self.imp()
            .directory_entry
            .set_text(self.imp().current_directory.borrow().as_str());
    }

    /* Function to set the shortcut */
    pub fn set_current_shortcut(&self, shortcut: String) {
        *self.imp().current_shortcut.borrow_mut() = shortcut;
        self.imp()
            .shortcut_entry
            .set_text(self.imp().current_shortcut.borrow().as_str());
    }

    pub fn hide_buttons(&self) {
        self.imp().edit_dir.hide();
        self.imp().edit_ss.hide();
        self.imp().directory_entry.set_can_focus(false);
        self.imp().shortcut_entry.set_can_focus(false);
    }

    pub fn setup_entry(&self) {
        let entry = self.imp().shortcut_entry.clone();
        let window = self.clone();
        let key_controller = EventControllerKey::new();
        let edit_ss = self.imp().edit_ss.clone();
        key_controller.connect_key_pressed(move |_, keyval, _, mods| {
            let flag = mods.is_empty();

            let shortcut = accelerator_name(keyval, mods);
            println!("Shortcut: {}", shortcut);
            if !flag && edit_ss.is_visible() {
                window.imp().shortcut_entry.set_text(shortcut.as_str());
            }

            Propagation::Proceed
        });
        entry.add_controller(key_controller);
    }

    pub fn setup_discard_shortcut_button(&self) {
        let discard_shortcut = gio::SimpleAction::new("discard_shortcut", None);
        let window = self.clone();
        let entry = self.imp().shortcut_entry.clone();
        let shortcut = self.imp().current_shortcut.clone();
        discard_shortcut.connect_activate(move |_, _| {
            //TODO: discard changes
            window.imp().edit_shortcut_button.show();
            window.imp().edit_ss.hide();
            entry.set_text(shortcut.borrow().as_str());
            entry.set_can_focus(false);
        });

        self.add_action(&discard_shortcut);
    }

    pub fn setup_save_shortcut_button(&self) {
        let save_shortcut = gio::SimpleAction::new("save_shortcut", None);
        let window = self.clone();
        let entry = self.imp().shortcut_entry.clone();
        save_shortcut.connect_activate(move |_, _| {
            //TODO: save changes
            window.imp().edit_shortcut_button.show();
            window.imp().edit_ss.hide();
            entry.set_can_focus(false);
        });

        self.add_action(&save_shortcut);
    }

    pub fn setup_edit_shortcut_button(&self) {
        let edit_shortcut = gio::SimpleAction::new("edit_shortcut", None);
        let entry = self.imp().shortcut_entry.clone();
        let window = self.clone();
        edit_shortcut.connect_activate(move |_, _| {
            window.imp().edit_shortcut_button.hide();
            window.imp().edit_ss.show();
            entry.set_can_focus(true);
            entry.grab_focus();
        });

        self.add_action(&edit_shortcut);
    }

    pub fn setup_edit_directory_button(&self) {
        let edit_directory = gio::SimpleAction::new("edit_directory", None);
        let window = self.clone();
        edit_directory.connect_activate(move |_, _| {
            window.imp().edit_directory.hide();
            window.imp().edit_dir.show();  
            window.imp().directory_entry.set_can_focus(true);
            window.imp().directory_entry.set_editable(true);
        });

        self.add_action(&edit_directory);
    }

    pub fn setup_save_directory_button(&self) {
        let save_directory = gio::SimpleAction::new("save_directory", None);
        let window = self.clone();
        save_directory.connect_activate(move |_, _| {
            //TODO: save changes
            window.imp().edit_directory.show();
            window.imp().edit_dir.hide();  
            window.imp().directory_entry.set_can_focus(false);
        });

        self.add_action(&save_directory);
    }

    pub fn setup_discard_directory_button(&self) {
        let discard_directory = gio::SimpleAction::new("discard_directory", None);
        let window = self.clone();
        discard_directory.connect_activate(move |_, _| {
            //discard changes
            window.imp().edit_directory.show();
            window.imp().edit_dir.hide();    
            window.imp().directory_entry.set_text(window.imp().current_directory.borrow().as_str());
            window.imp().directory_entry.set_can_focus(false);
        });

        self.add_action(&discard_directory);
    }
}
