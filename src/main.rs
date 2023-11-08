use gtk4 as gtk;

mod main_window;
pub mod first_menu_bar;
pub mod screenshot;

use main_window::MainWindow;
use gtk::{glib, prelude::*};
fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

        application.set_accels_for_action("win.new_screen", &["<Ctrl>a"]);

    application.connect_activate(|app| {
        let win = MainWindow::new(app);
        win.present();
    });
    application.run()
}
