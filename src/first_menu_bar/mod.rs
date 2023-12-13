mod imp;
use gtk4 as gtk;

use gtk::{glib, prelude::*, subclass::prelude::*, gio};

glib::wrapper! {
    pub struct FirstMenuBar(ObjectSubclass<imp::FirstMenuBar>)
        @extends gtk::Widget;
}

#[gtk::template_callbacks]
impl FirstMenuBar {
    pub fn set_delay(&self, new_delay: u64) {
        let imp = self.imp();
        *imp.delay.borrow_mut() = new_delay;
    }
    pub fn get_delay(&self) -> u64 {
        let imp = self.imp();
        // Ottieni un riferimento immutabile al valore interno di delay.
        *imp.delay.borrow()
    }

    pub fn update_delay(&self, delay: u64) {
        self.set_delay(delay); // Update the delay_timer variable
        let label_text = match delay {
            0 => "Nessun ritardo",
            3 => "Ritardo di 3 secondi",
            5 => "Ritardo di 5 secondi",
            10 => "Ritardo di 10 secondi",
            _ => "Tempo di ritardo sconosciuto", // Default case
        };
        self.imp().delay_label.set_label(label_text); // Update the label
    }

    pub fn populate_monitors_menu(&self, monitors:Vec<String>){
        if monitors.len() > 1 {
            let menu = gio::Menu::new();
            for (index, monitor) in monitors.iter().enumerate() {
                let menu_item = gio::MenuItem::new(Some(&monitor), Some("win.select_monitor"));
                let index_variant = (index as u32).to_variant();
                menu_item.set_attribute_value("target", Some(&index_variant));
                menu.append_item(&menu_item);
            }
            self.imp().monitors_menu.set_menu_model(Some(&menu));
        } else {
            self.imp().monitors_menu.hide();
        }
        
    }
}