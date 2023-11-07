mod imp;
mod screenshot;
use gtk::{gio, glib, prelude::*, subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt}};
use gtk4 as gtk;

glib::wrapper! {
    pub struct AddWindow(ObjectSubclass<imp::AddWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl AddWindow {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn set_delay(&self, new_delay: u64) {
        let imp = imp::AddWindow::from_obj(self);
        *imp.delay.borrow_mut() = new_delay;
    }
    pub fn get_delay(&self) -> u64 {
        let imp = imp::AddWindow::from_obj(self);
        // Ottieni un riferimento immutabile al valore interno di delay.
        *imp.delay.borrow()
    }

    pub fn setup_menu(&self) {
        let menu_button: gtk::MenuButton = self.imp().delay_opt.get();
        let menu_model = gio::Menu::new();
        
        // Aggiungi le voci al menu
        menu_model.append(Some("Nessun ritardo"), Some("app.no-delay"));
        menu_model.append(Some("Ritardo di 3 secondi"), Some("app.delay-3"));
        menu_model.append(Some("Ritardo di 5 secondi"), Some("app.delay-5"));
        menu_model.append(Some("Ritardo di 10 secondi"), Some("app.delay-10"));
        
        // Collega il modello al MenuButton
        menu_button.set_menu_model(Some(&menu_model));
        
        // Connetti le azioni
        let action_group = gio::SimpleActionGroup::new();
        for delay in [0, 3, 5, 10] {
            let action_name = format!("delay-{}", delay);
            let action = gio::SimpleAction::new(&action_name, None);
            action_group.add_action(&action);
            
            let menu_button_clone = menu_button.clone();
            let temp_self = self.clone();
            action.connect_activate(move |_, _| {
                // Cambia il testo del bottone qui
                menu_button_clone.set_label(&format!("Ritardo di {} secondi", delay));
                // Esegui la funzione per cambiare il delay
                temp_self.set_delay(delay);
            });
        }
        self.insert_action_group("app", Some(&action_group));
    }
}