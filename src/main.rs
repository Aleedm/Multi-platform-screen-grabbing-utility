use gtk4 as gtk;

mod main_window;
pub mod first_menu_bar;
pub mod edit_menu_bar;
pub mod screenshot;

use main_window::MainWindow;
use gtk::{gio, glib, prelude::*};
fn main() -> glib::ExitCode {

    gio::resources_register_include!("compiled.gresource").unwrap();

    let application = gtk::Application::builder()
        .application_id("org.mpsgu")
        .build();

    application.connect_activate(move |app| {
        let win: MainWindow = MainWindow::new(app);
        win.set_application(app.clone());
        win.update_shortcut(&["<Ctrl>a"]);
        win.present();
    });
    application.run()
}


/* fn set_new_screen_shortcut(app: &Application, keys:&[&str]){
    app.set_accels_for_action("win.new_screen", keys);
    
} */