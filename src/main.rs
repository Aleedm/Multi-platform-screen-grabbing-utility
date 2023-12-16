use gtk4 as gtk;

pub mod crop_menu_bar;
pub mod edit_menu_bar;
pub mod first_menu_bar;
mod main_window;
pub mod screenshot;
pub mod settings_manager;
pub mod settings_modal;
pub mod utility;

use gtk::{
    gdk, gio,
    glib::{self, subclass::types::ObjectSubclassIsExt},
    prelude::*,
    CssProvider, style_context_add_provider_for_display,
};
use main_window::MainWindow;
fn main() -> glib::ExitCode {

    gtk::init().expect("Unable to start GTK");

    gio::resources_register_include!("compiled.gresource").unwrap();

    let application = gtk::Application::builder()
        .application_id("org.mpsgu")
        .build();

    application.connect_activate(move |app| {
        let app1 = app.clone();
        let win: MainWindow = MainWindow::new(app);
        load_css();
        win.set_application(app.clone());
        let shortcut = win
            .imp()
            .settings_manager
            .borrow()
            .clone()
            .unwrap()
            .get_screen_shortcut();
        win.update_shortcut(&[&shortcut.as_str()]);
        win.present();
        win.connect_close_request(move |_| {
            app1.quit();
            glib::Propagation::Proceed
        });
    });
    application.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/org/mpsgu/css/style.css");

    if let Some(display) = gdk::Display::default() {
        style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
