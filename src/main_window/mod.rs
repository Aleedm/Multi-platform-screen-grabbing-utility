mod imp;
use gtk::glib::{clone, VariantType};
use gtk::{
    gdk, 
    GestureClick, 
    GestureDrag,
    gio, 
    glib, 
    prelude::*, 
    subclass::prelude::ObjectSubclassIsExt, 
    FileChooserAction,
    FileChooserDialog, 
    ResponseType,
    gdk_pixbuf::Pixbuf};
use screenshots::image::EncodableLayout;
use arboard::{Clipboard, ImageData};
use gtk4 as gtk;
use std::{borrow::Cow, 
    cell::RefCell,
    path::PathBuf,
    time::Duration,
    thread,
    cmp::{min, max},
    rc::Rc};

use self::imp::ImageOffset;
use crate::screenshot::screenshot;

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
    @implements gio::ActionMap, gio::ActionGroup;
}
#[derive(Clone, Debug)]
pub struct CropArea {
    start_x: i64,
    start_y: i64,
    end_x: i64,
    end_y: i64,
}

impl MainWindow {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn set_application(&self, new_app: gtk::Application) {
        let imp = self.imp();
        *imp.appl.borrow_mut() = new_app;
    }
    pub fn set_pixbuf(&self, new_pixbuf: Pixbuf) {
        let imp = self.imp();
        *imp.pixbuf.borrow_mut() = new_pixbuf;
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

    pub fn exit_action_setup(&self) {
        // Create the action for setting delay and add it to the window
        let exit = gio::SimpleAction::new("exit", None);

        let window = self.clone();
        exit.connect_activate(move |_, _| {
            window.imp().cropbar.hide();
            window.imp().menubar.show();
        });
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
            let pixbuf = screenshot();
            window.set_pixbuf(pixbuf.clone());
            image_clone.set_pixbuf(Some(&pixbuf));
            window.imp().menubar.imp().edit.show();
            window.show();
            window.present();
            if !window.is_maximized() {
                window.maximize();
            }

            let paintable = image_clone.paintable();
                if let Some(pain) = paintable {
                    let image_offset = calculate_image_dimension(image_clone.width(), image_clone.height(), pain.intrinsic_aspect_ratio());
                    window.set_image_offset(image_offset);
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
        save_screen.connect_activate(move |_, _| {
            let pixbuf_clone = window.imp().pixbuf.clone().into_inner();
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
            dialog.run_async(clone!(@strong pixbuf_clone => move |obj, answer| {
                if answer == ResponseType::Accept {
                    if let Some(filename) = obj.current_name() {
                        let mut path = PathBuf::from(obj.current_folder().unwrap().path().unwrap());
                        path.push(filename);
                        println!("Salvataggio dell'immagine in: {:?}", path);
                        
                        if let Err(err) = pixbuf_clone.savev(&path, "png", &[]) {
                            eprintln!("Errore nel salvataggio dell'immagine: {}", err);
                        }
                    }
                }
                obj.close();
            }));

        });

        self.add_action(&save_screen);
    }

    pub fn copy_action_setup(&self) {
        // Crea l'azione
        let copy_screen = gio::SimpleAction::new("copy_screen", None);

        let window = self.clone();
        copy_screen.connect_activate(move |_, _| {
            //Copy image to clipboard
            let mut clipboard = Clipboard::new().unwrap();
            let pixbuf: Pixbuf = window.imp().pixbuf.clone().into_inner();
            let bytes = pixbuf.pixel_bytes().unwrap();

            let img_data = ImageData {
                width: pixbuf.width() as usize,
                height: pixbuf.height() as usize,
                bytes: Cow::Borrowed(bytes.as_bytes()),
            };
            clipboard.set_image(img_data).unwrap();
        });

        self.add_action(&copy_screen);
    }

    pub fn crop_action_setup(&self) {
        //let window = self.clone();
        let crop = gio::SimpleAction::new("crop", None);

        let picture = self.imp().image.clone();

        let drawing_area = self.imp().drawing_area.clone();
        let drawing_area_clone = drawing_area.clone();
        let gesture_click = GestureClick::new();
        let gesture_click_clone = gesture_click.clone();
        let gesture_drag = GestureDrag::new();
        let gesture_drag_clone = gesture_drag.clone();

        gesture_drag.set_button(gdk::BUTTON_PRIMARY);
        gesture_click.set_button(gdk::BUTTON_PRIMARY);
        drawing_area.add_controller(gesture_drag);
        drawing_area.add_controller(gesture_click);

        let crop_area = Rc::new(RefCell::new(CropArea {
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
        }));

        let crop_mode_active = Rc::new(RefCell::new(false));

        // Impostazione della funzione di disegno
        drawing_area.set_draw_func(clone!(@strong crop_area => move |_, cr, _, _| {
            let area = crop_area.borrow();
            cr.rectangle(area.start_x as f64, area.start_y as f64, (area.end_x - area.start_x) as f64, (area.end_y - area.start_y) as f64);
            cr.set_source_rgba(0.0, 0.0, 1.0, 0.5);
            let _ = cr.fill_preserve();
            cr.set_source_rgb(0.0, 0.0, 1.0);
            let _ =cr.stroke();
        }));

        let window = self.clone();
        crop.connect_activate(
            clone!(@strong crop_mode_active, @strong crop_area => move |_, _| {
                *crop_mode_active.borrow_mut() = true;
                window.imp().menubar.hide();
                window.imp().cropbar.show();
                // Resetta l'area di selezione
                let mut area = crop_area.borrow_mut();
                *area = CropArea {
                    start_x: 0,
                    start_y: 0,
                    end_x: 0,
                    end_y: 0,
                };
                drawing_area.queue_draw();
            }),
        );

        // Gestione del click del mouse
        /*gesture_drag_clone.connect_drag_begin(
            clone!(@strong crop_mode_active,@strong crop_area => move | _, x, y| {
                eprintln!("pressed, crop_mode_active: {}", *crop_mode_active.borrow());
                if *crop_mode_active.borrow() {
                    let mut area = crop_area.borrow_mut();
                    area.start_x = x;
                    area.start_y = y;
                }
            }),
        );*/
        let window_1 = self.clone();
        gesture_click_clone.connect_pressed(
            clone!(@strong crop_mode_active, @strong crop_area, @strong drawing_area_clone, => move |_, _, x, y| {
                println!("PIXBUF: {}, {}", window_1.imp().pixbuf.clone().into_inner().width(), window_1.imp().pixbuf.clone().into_inner().height() );
                eprintln!("picture dimensions: {:?} {:?}", picture.width(), picture.height());
                eprintln!("drawing_area dimensions: {:?} {:?}", drawing_area_clone.width(), drawing_area_clone.height());
                let image_offset = window_1.imp().image_offset.clone();
                eprintln!("image_offset: {:?}", image_offset);
                eprintln!("x: {}, y: {}", x, y);
                if *crop_mode_active.borrow() && x >= image_offset.borrow().x as f64 && y >= image_offset.borrow().y as f64 
                && x <= (picture.width() as i64 - image_offset.borrow().x) as f64 && y <= (picture.height() as i64 - image_offset.borrow().y) as f64{
                    let mut area = crop_area.borrow_mut();
                    area.start_x = x as i64;
                    area.start_y = y as i64;
                }
            }),
        );
        // Gestione del rilascio del mouse
        let window_2 = self.clone();
        gesture_drag_clone.connect_drag_update(
            clone!(@strong crop_mode_active, @strong drawing_area_clone, @strong crop_area => move |_, offset_x, offset_y| {
                println!("drag update");
                let image_offset = window_2.imp().image_offset.clone();
                let image = window_2.imp().image.clone();
                let mut area = crop_area.borrow_mut();
                println!("area: {:?}", area);
                if *crop_mode_active.borrow() && area.start_x >= image_offset.borrow().x && area.start_y >= image_offset.borrow().y
                && area.start_x <= (image.width() as i64-image_offset.borrow().x) && area.start_y <= (image.height() as i64 -image_offset.borrow().y){
                    
                    area.end_x = min(max(area.start_x + offset_x as i64, image_offset.borrow().x), image.width() as i64 - image_offset.borrow().x);
                    area.end_y = min(max(area.start_y + offset_y as i64, image_offset.borrow().y), image.height() as i64 - image_offset.borrow().y);
                    drawing_area_clone.queue_draw();
                }
            }));
        let window_3 = self.clone();
        gesture_drag_clone.connect_drag_end(clone!(@strong crop_mode_active, @strong crop_area => move |_, _, _| {
            println!("drag end");
            let mut area = crop_area.borrow_mut();
            println!("area: {:?}", area);
            //check if the area of the rectangle is bigger than 0
            if *crop_mode_active.borrow() && area.start_x != area.end_x && area.start_y != area.end_y{
                println!("MUOIOOOOOOO");
                *crop_mode_active.borrow_mut() = false;
                
                let offset_x = window_3.imp().image_offset.borrow().x;
                let offset_y = window_3.imp().image_offset.borrow().y;

                let x_start_temp = min(area.start_x, area.end_x) - offset_x;
                let y_start_temp = min(area.start_y, area.end_y) - offset_y;
                let x_end_temp = max(area.start_x, area.end_x) - offset_x;
                let y_end_temp = max(area.start_y, area.end_y) - offset_y;
                
                let pixbuf = window_3.imp().pixbuf.clone().into_inner();
                let image = window_3.imp().image.clone();

                let width_pixbuf = pixbuf.width() as i64;
                let height_pixbuf = pixbuf.height() as i64;
                let width_image = image.width() as i64 - (offset_x * 2);
                let height_image = image.height() as i64 - (offset_y * 2);

                let x_start = ((x_start_temp as f64 / width_image as f64) * width_pixbuf as f64).round() as i64;
                let y_start = ((y_start_temp as f64 / height_image as f64) * height_pixbuf as f64).round() as i64;
                let x_end = ((x_end_temp as f64 / width_image as f64) * width_pixbuf as f64).round() as i64;
                let y_end = ((y_end_temp as f64 / height_image as f64) * height_pixbuf as f64).round() as i64;

                let width = x_end - x_start;
                let height = y_end - y_start;


                let cropped_pixbuf = pixbuf.new_subpixbuf(x_start as i32, y_start as i32, width as i32, height as i32);

                let image = window_3.imp().image.clone();
                image.set_pixbuf(Some(&cropped_pixbuf));
                window_3.set_pixbuf(cropped_pixbuf.clone());
                let paintable = image.paintable();
                if let Some(pain) = paintable {
                    let image_offset = calculate_image_dimension(image.width(), image.height(), pain.intrinsic_aspect_ratio());
                    window_3.set_image_offset(image_offset);
                }
            }

            area.start_x = 0;
            area.start_y = 0;
            area.end_x = 0;
            area.end_y = 0;
            let drawing_a = window_3.imp().drawing_area.clone();
            drawing_a.queue_draw();
        }));

        self.add_action(&crop);
    }

    pub fn set_image_offset(&self, new_offset: ImageOffset) {
        let imp = self.imp();
        *imp.image_offset.borrow_mut() = new_offset;
    }

    pub fn setup_size_allocate(&self) {
        let drawing_area = self.imp().drawing_area.clone();
        let image = self.imp().image.clone();
        let window = self.clone();

        drawing_area.connect_resize(move |_, width, height| {
            let paintable = image.paintable();
            if let Some(pain) = paintable {
                println!("Paintable dimensioni: {:?}", pain.intrinsic_aspect_ratio());
                let image_offset =
                    calculate_image_dimension(width, height, pain.intrinsic_aspect_ratio());
                window.set_image_offset(image_offset);
            }
        });
    }
}

fn calculate_image_dimension(width: i32, height: i32, aspect_ratio: f64) -> ImageOffset {
    let x = (width as f64 - (height as f64 * aspect_ratio)) / 2.0;
    if x > 0.0 {
        ImageOffset {
            x: x as i64,
            y: 0,
            aspect_ratio: aspect_ratio,
        }
    } else {
        let y = (height as f64 - (width as f64 / aspect_ratio)) / 2.0;
        ImageOffset {
            x: 0,
            y: y as i64,
            aspect_ratio: aspect_ratio,
        }
    }
}
