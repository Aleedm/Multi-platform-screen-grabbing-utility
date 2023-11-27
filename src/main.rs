use gtk4 as gtk;

mod main_window;
pub mod first_menu_bar;
pub mod settings_modal;
pub mod edit_menu_bar;
pub mod crop_menu_bar;
pub mod screenshot;
pub mod settings_manager;
pub mod utility;

use main_window::MainWindow;
use gtk::{gio, glib::{self, subclass::types::ObjectSubclassIsExt}, prelude::*};
fn main() -> glib::ExitCode {

    gio::resources_register_include!("compiled.gresource").unwrap();

    let application = gtk::Application::builder()
        .application_id("org.mpsgu")
        .build();

    application.connect_activate(move |app| {
        let app1 = app.clone();
        let win: MainWindow = MainWindow::new(app);
        win.set_application(app.clone());
        let shortcut = win.imp().settings_manager.clone().unwrap().get_screen_shortcut();
        win.update_shortcut(&[&shortcut.as_str()]);
        win.present();
        win.connect_close_request(move |_| {
            app1.quit();
            glib::Propagation::Proceed
        });
    });
    application.run()
}