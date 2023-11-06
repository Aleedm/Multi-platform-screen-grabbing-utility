use gtk4 as gtk;

mod add_window;

use add_window::AddWindow;
use gtk::{glib, prelude::*};
fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let win = AddWindow::new(app);
        win.present();
    });
    application.run()
}
