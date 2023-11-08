mod imp;
use gtk4 as gtk;

use gtk::{glib, subclass::prelude::*};

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


    /* #[template_callback]
    fn toggle_toggled(&self, toggle: &gtk::ToggleButton) {
        if toggle.is_active() {
            self.popover.popup();
        }
    }
    #[template_callback(name = "popover_closed")]
    fn unset_toggle(&self) {
        self.toggle.set_active(false);
    } */
}