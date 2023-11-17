mod imp;
use gtk::glib::clone;
use gtk::{gdk, GestureClick};
use gtk::{
    gio, glib, prelude::*, subclass::prelude::ObjectSubclassIsExt, FileChooserAction,
    FileChooserDialog, ResponseType,
};

use crate::screenshot::screenshot;
use arboard::{Clipboard, ImageData};
use glib::VariantType;
use gtk4 as gtk;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use std::{borrow::Cow, cell::RefCell};

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
    @implements gio::ActionMap, gio::ActionGroup;
}
#[derive(Clone)]
struct CropArea {
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
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

        //let window = self.clone();
        //let image_clone = self.imp().image.clone();
        copy_screen.connect_activate(move |_, _| {
            //Copy image to clipboard
            let mut clipboard = Clipboard::new().unwrap();
            let buf = screenshot();
            let glib_bytes = buf.pixel_bytes();
            // Ottieni un puntatore ai dati e convertilo in una fetta di byte
            let slice = unsafe {
                std::slice::from_raw_parts(
                    glib_bytes.clone().unwrap().as_ptr() as *const u8,
                    glib_bytes.clone().unwrap().len(),
                )
            };

            let bytes1 = Cow::Borrowed(slice);
            let img_data = ImageData {
                width: buf.width() as usize,
                height: buf.height() as usize,
                bytes: bytes1.clone(),
            };
            clipboard.set_image(img_data).unwrap();
        });

        self.add_action(&copy_screen);
    }

    pub fn crop_action_setup(&self) {
        //let window = self.clone();
        let crop = gio::SimpleAction::new("crop", None);

        let drawing_area = self.imp().drawing_area.clone();
        let drawing_area_clone = drawing_area.clone();
        let gesture = GestureClick::new();
        let gesture_clone = gesture.clone();

        gesture.set_button(gdk::BUTTON_PRIMARY);
        drawing_area.add_controller(gesture);

        let crop_area = Rc::new(RefCell::new(CropArea {
            start_x: 0.0,
            start_y: 0.0,
            end_x: 0.0,
            end_y: 0.0,
        }));

        let crop_mode_active = Rc::new(RefCell::new(false));

        // Impostazione della funzione di disegno
        drawing_area.set_draw_func(clone!(@strong crop_area => move |_, cr, _, _| {
            let area = crop_area.borrow();
            cr.rectangle(area.start_x, area.start_y, area.end_x - area.start_x, area.end_y - area.start_y);
            cr.set_source_rgba(0.0, 0.0, 1.0, 0.5);
            let _ = cr.fill_preserve();
            cr.set_source_rgb(0.0, 0.0, 1.0);
            let _ =cr.stroke();
        }));

        crop.connect_activate(clone!(@strong crop_mode_active, @strong gesture_clone, @strong crop_area => move |_, _| {
            eprintln!("crop");
            *crop_mode_active.borrow_mut() = true;
            eprintln!("crop_mode_active: {}", *crop_mode_active.borrow());

            // Resetta l'area di selezione
            let mut area = crop_area.borrow_mut();
            *area = CropArea {
                start_x: 0.0,
                start_y: 0.0,
                end_x: 0.0,
                end_y: 0.0,
            };
            drawing_area.queue_draw();
        }));

        // Gestione del click del mouse
        gesture_clone.connect_pressed(
            clone!(@strong crop_mode_active,@strong crop_area => move |_, _, x, y| {
                eprintln!("pressed, crop_mode_active: {}", *crop_mode_active.borrow());
                if *crop_mode_active.borrow() {
                    let mut area = crop_area.borrow_mut();
                    area.start_x = x;
                    area.start_y = y;
                }
            }),
        );
        // Gestione del rilascio del mouse
        gesture_clone.connect_released(
            clone!(@strong crop_mode_active, @strong drawing_area_clone, @strong crop_area => move |_, _, x, y| {
                eprintln!("released, crop_mode_active: {}", *crop_mode_active.borrow());
                if *crop_mode_active.borrow() {
                    let mut area = crop_area.borrow_mut();
                    area.end_x = x;
                    area.end_y = y;

                    drawing_area_clone.queue_draw();
                }
            }),
        );

        self.add_action(&crop);
    }
}
