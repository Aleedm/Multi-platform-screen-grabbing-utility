use gdk_pixbuf::Pixbuf;
use gtk::gdk_pixbuf::{self, Colorspace};
use gtk4 as gtk;
use screenshots::{
    image::{ImageBuffer, Rgba},
    Screen,
};

pub fn screenshot() -> Pixbuf {
    let screens = Screen::all().unwrap();

    let screen = screens[0].clone();

    println!("capturer: {:?}", screen);
    let buffer = screen.capture().unwrap();
    print!("before image_buffer");
    let pixbuf = image_buffer_to_gdk_pixbuf(&buffer).unwrap();
    print!("after");
    pixbuf
}

fn image_buffer_to_gdk_pixbuf(
    buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Result<Pixbuf, Box<dyn std::error::Error>> {
    let width = buffer.width() as i32;
    let height = buffer.height() as i32;
    let rowstride = width * 4; // 4 canali RGBA per pixel.

    // Clona i dati dell'immagine perché `from_mut_slice` si aspetta un prestito mutabile.
    let mut image_data = buffer.clone().into_raw();

    // Utilizza `from_mut_slice` per creare un Pixbuf da dati di pixel esistenti.
    // Assicurati che la vita dei dati dell'immagine sia sufficiente fino a quando il Pixbuf non viene distrutto.
    let pixbuf = Pixbuf::from_mut_slice(
        image_data.as_mut_slice(),
        Colorspace::Rgb,
        true, // RGBA usa l'alpha.
        8,    // 8 bit per canale.
        width,
        height,
        rowstride,
    );

    Ok(pixbuf)
}