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
    path::PathBuf,
    time::Duration,
    thread,
    cmp::{min, max}};

use crate::screenshot::screenshot;
use crate::utility::{ImageOffset, CropArea};

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
    @implements gio::ActionMap, gio::ActionGroup;
}


impl MainWindow {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    /* Function to set the application */
    pub fn set_application(&self, new_app: gtk::Application) {
        let imp = self.imp();
        *imp.appl.borrow_mut() = new_app;
    }

    /* "show_setting" action to show settings modal */
    pub fn settings_setup(&self){
        let show_setting = gio::SimpleAction::new("show_setting", None);
        let window = self.clone();
        show_setting.connect_activate(move |_, _| {
            // let settings_dialog:SettingsModal = glib::Object::new().expect("Failed to create Settings Dialog");
            window.imp().settings.set_transient_for(Some(&window));
            window.imp().settings.set_modal(true);
            window.imp().settings.grab_focus();
            window.imp().settings.present();
        });
        self.add_action(&show_setting);
    
    
    }

    /* Function to update the current pixbuf value */
    pub fn set_pixbuf(&self, new_pixbuf: Pixbuf) {
        let imp = self.imp();
        *imp.pixbuf.borrow_mut() = new_pixbuf;
    }

    /* Function to update the current CropArea value */
    pub fn set_crop_area(&self, new_crop_area: CropArea) {
        let imp = self.imp();
        *imp.crop_area.borrow_mut() = new_crop_area;
    }

    /* Function to update crop-mode value */
    pub fn set_crop_mode_active(&self, new_crop_mode_active: bool) {
        let imp = self.imp();
        *imp.crop_mode_active.borrow_mut() = new_crop_mode_active;
    }

    /* "set_delay" action to set new delay value */
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

    /* "Exit" action to exit from crop mode */
    pub fn exit_action_setup(&self) {
        let exit = gio::SimpleAction::new("exit", None);

        let window = self.clone();
        exit.connect_activate(move |_, _| {
            window.set_crop_mode_active(false);
            let area = CropArea::new();
            window.set_crop_area(area);
            
            window.imp().cropbar.hide();
            window.imp().menubar.show();
            let drawing_area = window.imp().drawing_area.clone();
            drawing_area.queue_draw();
        });
        self.add_action(&exit);
    }

    /* "Confrim" action to confirm area you want to crop */
    pub fn confirm_action_setup(&self) {
        let confirm = gio::SimpleAction::new("confirm", None);

        let window = self.clone();
        confirm.connect_activate(move |_, _| {
            let mut area = window.imp().crop_area.borrow_mut();
            println!("area: {:?}", area);
            //check if the area of the rectangle is bigger than 0
            let crop_mode_active = window.imp().crop_mode_active.clone();
            if *crop_mode_active.borrow() && area.get_start_x() != area.get_end_x() && area.get_start_y() != area.get_end_y(){
                println!("MUOIOOOOOOO");
                *crop_mode_active.borrow_mut() = false;
                
                let offset_x = window.imp().image_offset.borrow().get_x();
                let offset_y = window.imp().image_offset.borrow().get_y();

                let x_start_temp = min(area.get_start_x(), area.get_end_x()) - offset_x;
                let y_start_temp = min(area.get_start_y(), area.get_end_y()) - offset_y;
                let x_end_temp = max(area.get_start_x(), area.get_end_x()) - offset_x;
                let y_end_temp = max(area.get_start_y(), area.get_end_y()) - offset_y;
                
                let pixbuf = window.imp().pixbuf.clone().into_inner();
                let image = window.imp().image.clone();

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


                let cropped_pixbuf = Pixbuf::new(pixbuf.colorspace(), pixbuf.has_alpha(), pixbuf.bits_per_sample(), width as i32, height as i32).unwrap();
                pixbuf.copy_area(x_start as i32, y_start as i32, width as i32, height as i32, &cropped_pixbuf, 0, 0);
                let image = window.imp().image.clone();
                image.set_pixbuf(Some(&cropped_pixbuf));
                window.set_pixbuf(cropped_pixbuf.clone());
                let paintable = image.paintable();
                if let Some(pain) = paintable {
                    let image_offset = calculate_image_dimension(image.width(), image.height(), pain.intrinsic_aspect_ratio());
                    window.set_image_offset(image_offset);
                }
            }

            area.set_start_x(0);
            area.set_start_y(0);
            area.set_end_x(0);
            area.set_end_y(0);
            let drawing_a = window.imp().drawing_area.clone();
            drawing_a.queue_draw();
            window.set_crop_mode_active(false);
            
            window.imp().cropbar.hide();
            window.imp().menubar.show();
        });
        self.add_action(&confirm);
    }
    pub fn update_confirm_action_state(&self) {
        let crop_area = self.imp().crop_area.borrow();
        let is_invalid = is_crop_area_invalid(&crop_area);
    
        if let Some(action) = self.lookup_action("confirm") {
            if let Some(confirm_action) = action.downcast_ref::<gio::SimpleAction>() {
                confirm_action.set_enabled(!is_invalid);
            }
        }
    }
    /* "new_screen" action to add a screenshot to the window */
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

    
    /* Function to update shortcut for new screenshot action */
    pub fn update_shortcut(&self, values: &[&str]) {
        self.imp()
            .appl
            .borrow()
            .set_accels_for_action("win.new_screen", values);
    }

    /* "save_screen" action to save the current screenshot */
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

    /* "copy_screen" action to copy the current screenshot to the clipboard*/
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

        // Impostazione della funzione di disegno
        let window_1 = self.clone();
        drawing_area.set_draw_func(move |_, cr, _, _| {
            let crop_area = window_1.imp().crop_area.clone();
            cr.rectangle(crop_area.borrow().get_start_x() as f64, crop_area.borrow().get_start_y() as f64, crop_area.borrow().get_width() as f64, crop_area.borrow().get_height() as f64);
            cr.set_source_rgba(0.0, 0.0, 1.0, 0.5);
            let _ = cr.fill_preserve();
            cr.set_source_rgb(0.0, 0.0, 1.0);
            let _ =cr.stroke();
        });

        let window_2 = self.clone();
        crop.connect_activate(move |_, _| {
                window_2.set_crop_mode_active(true);
                window_2.imp().menubar.hide();
                window_2.imp().cropbar.show();
                // Resetta l'area di selezione
                window_2.set_crop_area(CropArea::new());
                drawing_area.queue_draw();
                window_2.update_confirm_action_state();
            },
        );

        let window_3 = self.clone();
        gesture_click_clone.connect_pressed(
            clone!(@strong drawing_area_clone, => move |_, _, x, y| {
                println!("PIXBUF: {}, {}", window_3.imp().pixbuf.clone().into_inner().width(), window_3.imp().pixbuf.clone().into_inner().height() );
                eprintln!("picture dimensions: {:?} {:?}", picture.width(), picture.height());
                eprintln!("drawing_area dimensions: {:?} {:?}", drawing_area_clone.width(), drawing_area_clone.height());
                let image_offset = window_3.imp().image_offset.clone();
                eprintln!("image_offset: {:?}", image_offset);
                eprintln!("x: {}, y: {}", x, y);
                let crop_mode_active = window_3.imp().crop_mode_active.clone();
                if *crop_mode_active.borrow() && x >= image_offset.borrow().get_x() as f64 && y >= image_offset.borrow().get_y() as f64 
                && x <= (picture.width() as i64 - image_offset.borrow().get_x()) as f64 && y <= (picture.height() as i64 - image_offset.borrow().get_y()) as f64{
                    print!("click");
                    
                    let crop_area = CropArea::new_with_params(x as i64, y as i64, 0, 0);
                    window_3.set_crop_area(crop_area);
                    println!("area: {:?}", window_3.imp().crop_area.borrow());
                }
            }),
        );
        // Gestione del rilascio del mouse
        let window_3 = self.clone();
        gesture_drag_clone.connect_drag_update(
            clone!(@strong drawing_area_clone => move |_, offset_x, offset_y| {
                //println!("drag update");
                {let image_offset = window_3.imp().image_offset.clone();
                let image = window_3.imp().image.clone();
                let mut area = window_3.imp().crop_area.borrow_mut();
                //println!("area: {:?}", area);
                let crop_mode_active = window_3.imp().crop_mode_active.clone();
                if *crop_mode_active.borrow() && area.get_start_x() >= image_offset.borrow().get_x() && area.get_start_y() >= image_offset.borrow().get_y()
                && area.get_start_x() <= (image.width() as i64-image_offset.borrow().get_x()) && area.get_start_y() <= (image.height() as i64 -image_offset.borrow().get_y()){
                    let end_x = min(max(area.get_start_x() + offset_x as i64, image_offset.borrow().get_x()), image.width() as i64 - image_offset.borrow().get_x());
                    area.set_end_x(end_x);
                    let end_y = min(max(area.get_start_y() + offset_y as i64, image_offset.borrow().get_y()), image.height() as i64 - image_offset.borrow().get_y());
                    area.set_end_y(end_y);
                    drawing_area_clone.queue_draw();
                }}

                window_3.update_confirm_action_state();
            }));
        //let window_4 = self.clone();
        /*gesture_drag_clone.connect_drag_end(move |_, _, _| {
            println!("drag end");
            let mut area = window_4.imp().crop_area.borrow_mut();
            println!("area: {:?}", area);
            //check if the area of the rectangle is bigger than 0
            let crop_mode_active = window_4.imp().crop_mode_active.clone();
            if *crop_mode_active.borrow() && area.get_start_x() != area.get_end_x() && area.get_start_y() != area.get_end_y(){
                println!("MUOIOOOOOOO");
                *crop_mode_active.borrow_mut() = false;
                
                let offset_x = window_4.imp().image_offset.borrow().get_x();
                let offset_y = window_4.imp().image_offset.borrow().get_y();

                let x_start_temp = min(area.get_start_x(), area.get_end_x()) - offset_x;
                let y_start_temp = min(area.get_start_y(), area.get_end_y()) - offset_y;
                let x_end_temp = max(area.get_start_x(), area.get_end_x()) - offset_x;
                let y_end_temp = max(area.get_start_y(), area.get_end_y()) - offset_y;
                
                let pixbuf = window_4.imp().pixbuf.clone().into_inner();
                let image = window_4.imp().image.clone();

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


                let cropped_pixbuf = Pixbuf::new(pixbuf.colorspace(), pixbuf.has_alpha(), pixbuf.bits_per_sample(), width as i32, height as i32).unwrap();
                pixbuf.copy_area(x_start as i32, y_start as i32, width as i32, height as i32, &cropped_pixbuf, 0, 0);
                let image = window_4.imp().image.clone();
                image.set_pixbuf(Some(&cropped_pixbuf));
                window_4.set_pixbuf(cropped_pixbuf.clone());
                let paintable = image.paintable();
                if let Some(pain) = paintable {
                    let image_offset = calculate_image_dimension(image.width(), image.height(), pain.intrinsic_aspect_ratio());
                    window_4.set_image_offset(image_offset);
                }
            }

            area.set_start_x(0);
            area.set_start_y(0);
            area.set_end_x(0);
            area.set_end_y(0);
            let drawing_a = window_4.imp().drawing_area.clone();
            drawing_a.queue_draw();
        });*/

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
        ImageOffset::new_with_params(x as i64, 0, aspect_ratio)
    } else {
        let y = (height as f64 - (width as f64 / aspect_ratio)) / 2.0;
        ImageOffset::new_with_params(0, y as i64, aspect_ratio)
    }
}


fn is_crop_area_invalid(crop_area: &CropArea) -> bool {
    crop_area.get_start_x() == crop_area.get_end_x() ||
    crop_area.get_start_y() == crop_area.get_end_y() ||
    (crop_area.get_start_x() == 0 && crop_area.get_start_y() == 0 && crop_area.get_end_x() == 0 && crop_area.get_end_y() == 0)
}