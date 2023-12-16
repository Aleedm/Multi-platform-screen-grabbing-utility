mod imp;
use crate::settings_manager::Settings;
use gtk::accelerator_name;
use gtk::glib::Propagation;
use gtk::{
    gio,
    glib,
    prelude::WidgetExt,
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
    EventControllerKey,
    FileChooserAction, FileChooserDialog,
    ResponseType
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

    pub fn get_settings_manager(&self) -> Option<Settings> {
        self.imp().settings_manager.borrow().clone()
    }

   
    pub fn set_settings_manager(&self, settings_manager: Settings) {
        *self.imp().settings_manager.borrow_mut() = Some(settings_manager);
        let settings = self
            .imp()
            .settings_manager
            .borrow()
            .clone()
            .expect("Settings not available");
        self.imp()
            .shortcut_entry
            .set_text(settings.get_screen_shortcut().as_str());
        self.imp()
            .directory_entry
            .set_text(settings.get_save_dir().as_str());
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
        discard_shortcut.connect_activate(move |_, _| {
            window.imp().edit_shortcut_button.show();
            window.imp().edit_ss.hide();
            let shortcut = window
                .imp()
                .settings_manager
                .borrow()
                .clone()
                .expect("Settings not available")
                .get_screen_shortcut();
            entry.set_text(shortcut.as_str());
            entry.set_can_focus(false);
        });
        self.add_action(&discard_shortcut);
    }

    pub fn setup_save_shortcut_button(&self) {
        let save_shortcut = gio::SimpleAction::new("save_shortcut", None);
        let window = self.clone();
        let entry = self.imp().shortcut_entry.clone();
        save_shortcut.connect_activate(move |_, _| {
            let new_shortcut = entry.text().as_str().to_string();
            window.imp().edit_shortcut_button.show();
            window.imp().edit_ss.hide();
            entry.set_can_focus(false);
            let mut settings_manager = window.imp().settings_manager.borrow_mut();
            if let Some(ref mut settings) = *settings_manager {
                settings.set_screen_shortcut(new_shortcut.clone());
            }

            
            if let Some(settings) = settings_manager.clone() {
                settings.write_settings("config.json".to_string());
            }
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
            let dialog = FileChooserDialog::new(
                Some("Default save folder"),
                Some(&window),
                FileChooserAction::SelectFolder,
                &[
                    ("Cancel", ResponseType::Cancel),
                    ("Save", ResponseType::Accept),
                ],
            );
            dialog.show();
            let window_clone = window.clone();
            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(folder_path) = dialog.current_folder() {
                       window_clone.imp().directory_entry.set_text(folder_path.path().expect("Incorrect Folder name").to_str().expect("Incorrect path"));
                    }
                }
                dialog.close();
            }); 
        });

        self.add_action(&edit_directory);
    }

    pub fn setup_save_directory_button(&self) {
        let save_directory = gio::SimpleAction::new("save_directory", None);
        let window = self.clone();
        save_directory.connect_activate(move |_, _| {
            
            window.imp().edit_directory.show();
            window.imp().edit_dir.hide();
            window.imp().directory_entry.set_can_focus(false);

            let mut settings_manager = window.imp().settings_manager.borrow_mut();
            if let Some(ref mut settings) = *settings_manager {
                settings.set_save_dir(window.imp().directory_entry.text().as_str().to_string());
            }
            if let Some(settings) = settings_manager.clone() {
                settings.write_settings("config.json".to_string());
            }
        });

        self.add_action(&save_directory);
    }

    pub fn setup_discard_directory_button(&self) {
        let discard_directory = gio::SimpleAction::new("discard_directory", None);
        let window = self.clone();
        discard_directory.connect_activate(move |_, _| {
            
            window.imp().edit_directory.show();
            window.imp().edit_dir.hide();
            let old_dir = window
                .imp()
                .settings_manager
                .borrow()
                .clone()
                .expect("Settings not available")
                .get_save_dir();
            window.imp().directory_entry.set_text(old_dir.as_str());
            window.imp().directory_entry.set_can_focus(false);
        });

        self.add_action(&discard_directory);
    }
}
