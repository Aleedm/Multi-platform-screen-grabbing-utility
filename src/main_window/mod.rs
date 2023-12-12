mod imp;
use arboard::{Clipboard, ImageData};
use gtk::cairo;
use gtk::glib::{clone, Propagation, VariantType};
use gtk::{
    gdk, gdk_pixbuf::Pixbuf, gio, glib, prelude::*, subclass::prelude::ObjectSubclassIsExt,
    FileChooserAction, FileChooserDialog, GestureClick, GestureDrag, ResponseType,
};
use gtk4 as gtk;
use screenshots::image::EncodableLayout;
use std::{
    borrow::Cow,
    cmp::{max, min},
    path::PathBuf,
    thread,
    time::Duration,
};

use crate::screenshot::screenshot;
use crate::settings_modal::SettingsModal;
use crate::utility::{CropArea, ImageOffset};

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
    pub fn settings_setup(&self) {
        let show_setting = gio::SimpleAction::new("show_setting", None);
        let window = self.clone();
        let app = window.imp().appl.clone().into_inner();
        let settings: SettingsModal = SettingsModal::new(&app);
        settings.set_application(Some(&app.clone()));
        let settings_manager = self.imp().settings_manager.clone().expect("Settings not available");
        settings.set_settings_manager(settings_manager);
        show_setting.connect_activate(move |_, _| {
            settings.set_transient_for(Some(&window));
            settings.set_modal(true);
            settings.focus();
            settings.present();
            // Utilizza glib::Cast per eseguire un cast sicuro
            if let Ok(dialog) = settings.clone().dynamic_cast::<gtk::ApplicationWindow>() {
                dialog.connect_close_request(|dialog| {
                    dialog.hide();
                    Propagation::Stop
                });
            }
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

    pub fn set_side_selected(&self, new_side_selected: i8) {
        let imp = self.imp();
        *imp.side_selected.borrow_mut() = new_side_selected;
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
            //check if the area of the rectangle is bigger than 0
            let crop_mode_active = window.imp().crop_mode_active.clone();
            if *crop_mode_active.borrow()
                && area.get_start_x() != area.get_end_x()
                && area.get_start_y() != area.get_end_y()
            {
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

                let x_start = ((x_start_temp as f64 / width_image as f64) * width_pixbuf as f64)
                    .round() as i64;
                let y_start = ((y_start_temp as f64 / height_image as f64) * height_pixbuf as f64)
                    .round() as i64;
                let x_end =
                    ((x_end_temp as f64 / width_image as f64) * width_pixbuf as f64).round() as i64;
                let y_end = ((y_end_temp as f64 / height_image as f64) * height_pixbuf as f64)
                    .round() as i64;

                let width = x_end - x_start;
                let height = y_end - y_start;

                let cropped_pixbuf = Pixbuf::new(
                    pixbuf.colorspace(),
                    pixbuf.has_alpha(),
                    pixbuf.bits_per_sample(),
                    width as i32,
                    height as i32,
                )
                .unwrap();
                pixbuf.copy_area(
                    x_start as i32,
                    y_start as i32,
                    width as i32,
                    height as i32,
                    &cropped_pixbuf,
                    0,
                    0,
                );
                let image = window.imp().image.clone();
                image.set_pixbuf(Some(&cropped_pixbuf));
                window.set_pixbuf(cropped_pixbuf.clone());
                let paintable = image.paintable();
                if let Some(pain) = paintable {
                    let image_offset = calculate_image_offset(
                        image.width(),
                        image.height(),
                        pain.intrinsic_aspect_ratio(),
                    );
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
            window.hide();
            while glib::MainContext::default().iteration(false) {}
            if delay > 0 {
                let sleep_duration = Duration::from_secs(delay);
                thread::sleep(sleep_duration);
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
                let image_offset = calculate_image_offset(
                    image_clone.width(),
                    image_clone.height(),
                    pain.intrinsic_aspect_ratio(),
                );
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
                &[],
            );
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

        //let picture = self.imp().image.clone();

        let drawing_area = self.imp().drawing_area.clone();
        let drawing_area_clone = drawing_area.clone();
        let gesture_click = GestureClick::new();
        let gesture_click_clone = gesture_click.clone();
        let gesture_drag = GestureDrag::new();
        let gesture_drag_clone = gesture_drag.clone();
        let cursor_controller = gtk::EventControllerMotion::new();
        let cursor_controller_clone = cursor_controller.clone();

        gesture_drag.set_button(gdk::BUTTON_PRIMARY);
        gesture_click.set_button(gdk::BUTTON_PRIMARY);
        drawing_area.add_controller(gesture_drag);
        drawing_area.add_controller(gesture_click);
        drawing_area.add_controller(cursor_controller);

        // Impostazione della funzione di disegno
        let window_1 = self.clone();
        drawing_area.set_draw_func(move |_, cr, width, height| {
            let crop_mode_active = window_1.imp().crop_mode_active.clone();
            if *crop_mode_active.borrow() {
                cr.set_source_rgba(0.3, 0.3, 0.3, 0.8); // Colore grigio
                cr.rectangle(0.0, 0.0, width as f64, height as f64);
                let _ = cr.fill();

                let crop_area = window_1.imp().crop_area.clone();
                let side_selected = window_1.imp().side_selected.clone();
                let width;
                let height;
                if *side_selected.borrow() == -1 {
                    width = crop_area.borrow().get_end_x() - crop_area.borrow().get_start_x();
                    height = crop_area.borrow().get_end_y() - crop_area.borrow().get_start_y();
                    cr.rectangle(
                        crop_area.borrow().get_start_x() as f64,
                        crop_area.borrow().get_start_y() as f64,
                        width as f64,
                        height as f64,
                    );
                    cr.set_source_rgb(1.0, 1.0, 1.0);
                    let _ = cr.stroke();
                    cr.rectangle(
                        crop_area.borrow().get_start_x() as f64,
                        crop_area.borrow().get_start_y() as f64,
                        width as f64,
                        height as f64,
                    );
                    cr.set_operator(cairo::Operator::Clear);
                    let _ = cr.fill();
                } else {
                    let start_x = crop_area.borrow().get_new_start_x();
                    let start_y = crop_area.borrow().get_new_start_y();
                    width =
                        crop_area.borrow().get_new_end_x() - crop_area.borrow().get_new_start_x();
                    height =
                        crop_area.borrow().get_new_end_y() - crop_area.borrow().get_new_start_y();
                    cr.rectangle(start_x as f64, start_y as f64, width as f64, height as f64);
                    cr.set_source_rgb(1.0, 1.0, 1.0);
                    let _ = cr.stroke();
                    cr.rectangle(start_x as f64, start_y as f64, width as f64, height as f64);
                    cr.set_operator(cairo::Operator::Clear);
                    let _ = cr.fill();
                }
            }
        });

        let window_2 = self.clone();
        crop.connect_activate(move |_, _| {
            window_2.set_crop_mode_active(true);
            window_2.imp().menubar.hide();
            window_2.imp().cropbar.show();
            // Resetta l'area di selezione
            let image_offset = window_2.imp().image_offset.clone();
            let image = window_2.imp().image.clone();
            let crop_area = calculate_crop_area(
                image_offset.borrow().get_x(),
                image_offset.borrow().get_y(),
                image.width() as i64,
                image.height() as i64,
            );
            window_2.set_crop_area(crop_area);
            drawing_area.queue_draw();
            window_2.update_confirm_action_state();
        });
        let window_3 = self.clone();
        cursor_controller_clone.connect_motion(move |_, x, y| {
            let drawing_area_clone = window_3.imp().drawing_area.clone();
            let crop_area = window_3.imp().crop_area.clone();
            let x_start = crop_area.borrow().get_start_x();
            let y_start = crop_area.borrow().get_start_y();
            let x_end = crop_area.borrow().get_end_x();
            let y_end = crop_area.borrow().get_end_y();
            let side_selected = window_3.imp().side_selected.borrow().clone();

            // Controlla se il mouse è vicino a un bordo del rettangolo
            let cursor_name;
            if side_selected == -1 {
                cursor_name = "grab";
            } else {
                cursor_name = "grabbing";
            }
            set_cursor(
                x,
                x_start,
                x_end,
                y,
                y_start,
                y_end,
                drawing_area_clone,
                cursor_name,
            );
        });

        let window_4 = self.clone();
        gesture_click_clone.connect_pressed(move |_, _, x, y| {
            let drawing_area_clone = window_4.imp().drawing_area.clone();
            let crop_area = window_4.imp().crop_area.clone();
            let x_start = crop_area.borrow().get_start_x();
            let y_start = crop_area.borrow().get_start_y();
            let x_end = crop_area.borrow().get_end_x();
            let y_end = crop_area.borrow().get_end_y();

            // controlla se il mouse è vicino ad uno dei bordi e aggiorna la side selected
            if (x as i64 - x_start).abs() < 10 {
                window_4.set_side_selected(0); // 0 = left
            } else if (x as i64 - x_end).abs() < 10 {
                window_4.set_side_selected(1); // 1 = right
            } else if (y as i64 - y_start).abs() < 10 {
                window_4.set_side_selected(2); // 2 = top
            } else if (y as i64 - y_end).abs() < 10 {
                window_4.set_side_selected(3); // 3 = bottom
            } else {
                window_4.set_side_selected(-1);
            }
            let cursor_name = "grabbing";
            set_cursor(
                x,
                x_start,
                x_end,
                y,
                y_start,
                y_end,
                drawing_area_clone,
                cursor_name,
            );
            if (x as i64 - x_start).abs() < 10
                || (x as i64 - x_end).abs() < 10
                || (y as i64 - y_start).abs() < 10
                || (y as i64 - y_end).abs() < 10
            {
                crop_area.borrow_mut().set_new_start_x(x_start);
                crop_area.borrow_mut().set_new_start_y(y_start);
                crop_area.borrow_mut().set_new_end_x(x_end);
                crop_area.borrow_mut().set_new_end_y(y_end);
            }
        });

        let window_5 = self.clone();
        gesture_drag_clone.connect_drag_update(
            clone!(@strong drawing_area_clone => move |_, offset_x, offset_y| {
                //println!("offset x: {}, offset y: {}", offset_x, offset_y);
                    //println!("drag update");
                {
                    let image_offset = window_5.imp().image_offset.clone();
                    let image = window_5.imp().image.clone();
                    let mut area = window_5.imp().crop_area.borrow_mut();
                    //println!("area: {:?}", area);
                    let crop_mode_active = window_5.imp().crop_mode_active.clone();
                    // if *crop_mode_active.borrow() && area.get_start_x() >= image_offset.borrow().get_x() && area.get_start_y() >= image_offset.borrow().get_y()
                    // && area.get_start_x() <= (image.width() as i64-image_offset.borrow().get_x()) && area.get_start_y() <= (image.height() as i64 -image_offset.borrow().get_y()){
                    //     let end_x = min(max(area.get_start_x() + offset_x as i64, image_offset.borrow().get_x()), image.width() as i64 - image_offset.borrow().get_x());
                    //     area.set_end_x(end_x);
                    //     let end_y = min(max(area.get_start_y() + offset_y as i64, image_offset.borrow().get_y()), image.height() as i64 - image_offset.borrow().get_y());
                    //     area.set_end_y(end_y);
                    //     drawing_area_clone.queue_draw();
                    // }
                    let side_selected = window_5.imp().side_selected.clone();
                    if *crop_mode_active.borrow() && area.get_start_x() >= image_offset.borrow().get_x() && area.get_start_y() >= image_offset.borrow().get_y()
                    && area.get_start_x() <= (image.width() as i64-image_offset.borrow().get_x()) && area.get_start_y() <= (image.height() as i64 -image_offset.borrow().get_y()){
                        if *side_selected.borrow() == 0 { //left
                            let start_x = min(max(area.get_start_x() + offset_x as i64, image_offset.borrow().get_x()), image.width() as i64 - image_offset.borrow().get_x());
                            area.set_new_start_x(start_x);
                            drawing_area_clone.queue_draw();
                        }else if *side_selected.borrow() == 1{
                            let end_x = max(min(area.get_end_x() + offset_x as i64, image.width() as i64 - image_offset.borrow().get_x()), image_offset.borrow().get_x());
                            area.set_new_end_x(end_x);
                            drawing_area_clone.queue_draw();
                        }else if *side_selected.borrow() == 2{
                            let start_y = min(max(area.get_start_y() + offset_y as i64, image_offset.borrow().get_y()), image.height() as i64 - image_offset.borrow().get_y());
                            area.set_new_start_y(start_y);
                            drawing_area_clone.queue_draw();
                        }else if *side_selected.borrow() == 3{
                            let end_y = max(min(area.get_end_y() + offset_y as i64, image.height() as i64 - image_offset.borrow().get_y()), image_offset.borrow().get_y());
                            area.set_new_end_y(end_y);
                            drawing_area_clone.queue_draw();
                        }
                    }
                }


                    window_5.update_confirm_action_state();
            }),
        );

        let window_6 = self.clone();
        gesture_drag_clone.connect_drag_end(move |_, x, y| {
            let drawing_area_clone = window_6.imp().drawing_area.clone();
            let crop_area = window_6.imp().crop_area.clone();
            // let old_x_start = crop_area.borrow().get_start_x();
            // let old_y_start = crop_area.borrow().get_start_y();
            // let old_x_end = crop_area.borrow().get_end_x();
            // let old_y_end = crop_area.borrow().get_end_y();
            let x_start = crop_area.borrow().get_new_start_x();
            let y_start = crop_area.borrow().get_new_start_y();
            let x_end = crop_area.borrow().get_new_end_x();
            let y_end = crop_area.borrow().get_new_end_y();
            let new_crop_area = CropArea::new_with_params(x_start, y_start, x_end, y_end);
            window_6.set_crop_area(new_crop_area);

            // Controlla se il mouse è vicino a un bordo del rettangolo
            let cursor_name = "grab";
            set_cursor(
                x,
                x_start,
                x_end,
                y,
                y_start,
                y_end,
                drawing_area_clone,
                cursor_name,
            );
            window_6.set_side_selected(-1);
        });

        /*let window_3 = self.clone();
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
        let window_4 = self.clone();
        gesture_drag_clone.connect_drag_end(move |_, _, _| {
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
                let image_offset =
                    calculate_image_offset(width, height, pain.intrinsic_aspect_ratio());
                // let crop_area = calculate_crop_area(
                //     image_offset.get_x(),
                //     image_offset.get_y(),
                //     image.width() as i64,
                //     image.height() as i64,
                // );
                window.set_image_offset(image_offset);
                //window.set_crop_area(crop_area);
            }
        });
    }
}

fn set_cursor(
    x: f64,
    x_start: i64,
    x_end: i64,
    y: f64,
    y_start: i64,
    y_end: i64,
    drawing_area_clone: gtk::DrawingArea,
    name: &str,
) {
    if (x as i64 - x_start).abs() < 10
        || (x as i64 - x_end).abs() < 10
        || (y as i64 - y_start).abs() < 10
        || (y as i64 - y_end).abs() < 10
    {
        let cursor = gdk::Cursor::from_name(name, None);
        drawing_area_clone.set_cursor(Some(&cursor.unwrap()));
    } else {
        // Ripristina il cursore normale
        drawing_area_clone.set_cursor(None);
    }
}

// fn reset_cursor(drawing_area_clone: gtk::DrawingArea) {
//     let cursor = gdk::Cursor::from_name("default", None);
//     drawing_area_clone.set_cursor(Some(&cursor.unwrap()));
// }

fn calculate_image_offset(width: i32, height: i32, aspect_ratio: f64) -> ImageOffset {
    let x = (width as f64 - (height as f64 * aspect_ratio)) / 2.0;
    if x > 0.0 {
        ImageOffset::new_with_params(x as i64, 0, aspect_ratio)
    } else {
        let y = (height as f64 - (width as f64 / aspect_ratio)) / 2.0;
        ImageOffset::new_with_params(0, y as i64, aspect_ratio)
    }
}
fn calculate_crop_area(
    x_offset: i64,
    y_offset: i64,
    width_image: i64,
    height_image: i64,
) -> CropArea {
    let x_start = x_offset;
    let y_start = y_offset;
    let x_end = width_image - x_offset;
    let y_end = height_image - y_offset;
    CropArea::new_with_params(x_start, y_start, x_end, y_end)
}

fn is_crop_area_invalid(crop_area: &CropArea) -> bool {
    crop_area.get_start_x() == crop_area.get_end_x()
        || crop_area.get_start_y() == crop_area.get_end_y()
        || (crop_area.get_start_x() == 0
            && crop_area.get_start_y() == 0
            && crop_area.get_end_x() == 0
            && crop_area.get_end_y() == 0)
}
