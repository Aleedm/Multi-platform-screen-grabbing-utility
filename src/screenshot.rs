use gdk_pixbuf::Pixbuf;
use gtk::gdk::{Display, self};
use gtk::gdk_pixbuf::{self, Colorspace};
use gtk::glib::{self, Cast};
use gtk::prelude::{DisplayExt, ListModelExt, MonitorExt};
use gtk4 as gtk;
use screenshots::{
    image::{ImageBuffer, Rgba},
    Screen,
};


pub fn screenshot() -> Pixbuf {
    let screens = Screen::all().unwrap();
    println!("screens: {:?}", screens);

    let display = Display::default().unwrap();
    let monitors = display.monitors();
    for i in 0..monitors.n_items() {
        // Ottieni l'elemento (Monitor) dall'elenco.
        let monitor = monitors.item(i).expect("Could not get monitor");

        // Converti l'elemento in Monitor.
        let monitor: gdk::Monitor = monitor.downcast().expect("Could not downcast to Monitor");

        // Stampa informazioni sul monitor.
        println!("Monitor {}: {:?}", i, monitor.geometry());

        // Prova ad ottenere il nome del monitor, se disponibile.
        let monitor_name = monitor.model().unwrap_or_else(|| "Unknown".into());
        println!("Monitor Name: {}", monitor_name);
    }

    let screen = screens[0].clone();

    let buffer = screen.capture().unwrap();
    let pixbuf = image_buffer_to_gdk_pixbuf(&buffer).unwrap();
    pixbuf
}

fn image_buffer_to_gdk_pixbuf(
    buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Result<Pixbuf, Box<dyn std::error::Error>> {
    let width = buffer.width() as i32;
    let height = buffer.height() as i32;
    let rowstride = width * 4; // 4 bytes per pixel (RGBA)

    let pixbuf = Pixbuf::from_bytes(
        &glib::Bytes::from(&buffer.as_flat_samples().as_slice()),
        Colorspace::Rgb,
        true, // has_alpha
        8,    // bits_per_sample
        width,
        height,
        rowstride,
    );

    Ok(pixbuf)
}

