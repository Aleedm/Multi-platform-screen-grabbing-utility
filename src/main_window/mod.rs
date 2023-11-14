mod imp;
use gtk::{gio, glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use gtk4 as gtk;
use glib::VariantType;
use std::time::Duration;
use crate::screenshot::screenshot;
use std::thread;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl MainWindow {    
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn set_application(&self, new_app: gtk::Application) {
        let imp = self.imp();
        *imp.appl.borrow_mut() = new_app;
    }

    pub fn delay_action_setup(&self){
        // Create the action for setting delay and add it to the window
        let set_delay = gio::SimpleAction::new("set_delay", Some(&VariantType::new("t").unwrap()));

        let temp_self = self.clone();
        set_delay.connect_activate(move |action, parameter| {
            let delay_value = parameter
                .unwrap()
                .get::<u64>()
                .expect("The value should be of type u64");

            action.set_state(parameter.unwrap());
            
            // Get the FirstMenuBar instance and call the update_delay method
            temp_self.imp().menubar.update_delay(delay_value);
            // Set the state of the action to the new delay value
            action.set_state(&parameter.unwrap());
        });
        self.add_action(&set_delay);
    }

    pub fn screen_action_setup(&self){
        // Create the action for setting delay and add it to the window
        let new_screen = gio::SimpleAction::new("new_screen", None);


        let window = self.clone();
        let image_clone = self.imp().image.clone();
        new_screen.connect_activate(move |_, _| {
            // Get the FirstMenuBar instance and get delay value
            let delay = window.imp().menubar.get_delay();
            println!("delay: {}", delay);
            eprintln!("Clicked!");
            window.hide();
            while glib::MainContext::default().iteration(false) {}
            if delay > 0 {
                eprintln!("waiting {:?} seconds", delay);
                let sleep_duration = Duration::from_secs(delay);
                thread::sleep(sleep_duration);
                eprintln!("waited {:?} seconds", sleep_duration);
            }
            image_clone.set_pixbuf(Some(&screenshot()));
            if !window.is_maximized() {
                window.maximize();
            }
            window.show();
            window.present();
        });
        self.add_action(&new_screen);    
    }

    pub fn update_shortcut(&self, values:&[&str]){
        self.imp().appl.borrow().set_accels_for_action("win.new_screen", values);
    }

}