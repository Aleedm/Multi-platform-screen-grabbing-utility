mod imp;
use gtk::{
    cairo, gdk::pixbuf_get_from_surface, gio, glib, prelude::*,
    subclass::prelude::ObjectSubclassIsExt, FileChooserAction, FileChooserDialog, ResponseType,
};

use gdk_pixbuf::Pixbuf;
use gtk::gdk_pixbuf;

use crate::screenshot::screenshot;
use arboard::{Clipboard, ImageData};
use cairo::{Format, ImageSurface};
use glib::VariantType;
use gtk4 as gtk;
use std::borrow::Cow;
use std::thread;
use std::time::Duration;

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

    pub fn delay_action_setup(&self) {
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

    pub fn screen_action_setup(&self) {
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
            window.imp().menubar.imp().edit.show();
            window.show();
            window.present();
            if !window.is_maximized() {
                window.maximize();
            }
        });
        self.add_action(&new_screen);
    }

    pub fn update_shortcut(&self, values: &[&str]) {
        self.imp()
            .appl
            .borrow()
            .set_accels_for_action("win.new_screen", values);
    }

    pub fn save_action_setup(&self) {
        // Crea l'azione
        let save_screen = gio::SimpleAction::new("save_screen", None);

        let window = self.clone();
        //let image_clone = self.imp().image.clone();
        save_screen.connect_activate(glib::clone!(@weak window =>move |_, _| {
            // Apri la finestra di dialogo per salvare l'immagine
            let dialog = FileChooserDialog::new(
                Some("Save Image"),
               Some(&window),
               FileChooserAction::Save,
              &[]);
            dialog.add_buttons(&[
                ("Cancel", ResponseType::Cancel),
                ("Save", ResponseType::Accept),
            ]);

            // Imposta un nome di file predefinito, se necessario
            dialog.set_current_name("untitled.png");

            // Mostra la finestra di dialogo e attendi la risposta dell'utente
            dialog.show();

            dialog.run_async(|obj, answer| {
                if answer == ResponseType::Accept{
                    if let Some(file_path) = obj.current_name() {
                        println!("Salvataggio dell'immagine in: {:?}", file_path);

                    }
                }
                obj.close();
            });
        }));

        self.add_action(&save_screen);
    }

    pub fn copy_action_setup(&self) {
        // Crea l'azione
        let copy_screen = gio::SimpleAction::new("copy_screen", None);

        let window = self.clone();
        let image_clone = self.imp().image.clone();
        copy_screen.connect_activate(move |_, _| {
            /*//Copy image to clipboard

            let width = image_clone.size(gtk::Orientation::Horizontal);
            let height = image_clone.size(gtk::Orientation::Vertical);

            let surface = cairo::ImageSurface::create(Format::ARgb32, width, height)
                .expect("Couldn't create a surface!");
            let context = cairo::Context::new(&surface).unwrap();

            let paintable = image_clone.paintable().unwrap();
            // Disegna il paintable sul contesto Cairo

            context.set_source_surface(surface, width as f64, height as f64);
            //paintable.snapshot(&mut gtk::Snapshot::new(context), width as f64, height as f64);
            // Converti la superficie Cairo in un GdkPixbuf
            let pixbuf = pixbuf_get_from_surface(&surface, 0, 0, width, height)
                .expect("Failed to create Pixbuf");

            let mut clipboard = Clipboard::new().unwrap();
            // let buf = screenshot();
            // let glib_bytes = buf.pixel_bytes();
            // // Ottieni un puntatore ai dati e convertilo in una fetta di byte
            // let slice = unsafe {
            //     std::slice::from_raw_parts(glib_bytes.clone().unwrap().as_ptr() as *const u8, glib_bytes.clone().unwrap().len())
            // };

            // let bytes1 = Cow::Borrowed(slice);
            let img_data = ImageData {
                width: width as usize,
                height: height as usize,
                bytes: Cow::Borrowed(unsafe { pixbuf.pixels() }),
            };
            clipboard.set_image(img_data).unwrap();*/
        });

        self.add_action(&copy_screen);
    }
}
