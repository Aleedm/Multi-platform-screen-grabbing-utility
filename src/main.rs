use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use gtk4 as gtk;
use screenshots::Screen;

fn screenshot() {
    let screens = Screen::all().unwrap();

    let screen = screens[0].clone();

    println!("capturer: {:?}", screen);
    let image = screen.capture().unwrap();
    image.save(format!("target/prova.png")).unwrap();
}

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
            screenshot();
        });
        window.set_child(Some(&button));

        window.present();
    });

    application.run()
}
