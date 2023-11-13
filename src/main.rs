use gtk4 as gtk;

mod main_window;
pub mod first_menu_bar;
pub mod screenshot;

use main_window::MainWindow;
use gtk::{gio, glib, prelude::*};
fn main() -> glib::ExitCode {

    gio::resources_register_include!("compiled.gresource").unwrap();

    let application = gtk::Application::builder()
        .application_id("org.mpsgu")
        .build();


    //let app_clone = Rc::new(RefCell::new(application));
    //let app_clone_for_activate = Rc::clone(&app_clone);    

    application.set_accels_for_action("win.new_screen", &["<Ctrl>a"]);
    application.connect_activate(|app| {
        let win: MainWindow = MainWindow::new(app);
        win.present();
    });
    application.run()
}


/* fn set_new_screen_shortcut(app: &Application, keys:&[&str]){
    app.set_accels_for_action("win.new_screen", keys);
    
} */