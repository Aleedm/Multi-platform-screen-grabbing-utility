use gdk_pixbuf::Pixbuf;
use gtk::gdk_pixbuf::{self, Colorspace};
use gtk::glib;
use gtk4 as gtk;
use screenshots::{
    image::{ImageBuffer, Rgba},
    Screen,
};

pub fn screenshot(index: usize) -> Pixbuf {
    let screens = Screen::all().unwrap();
    let screen = screens[index].clone();
    let buffer = screen.capture().unwrap();
    let pixbuf = image_buffer_to_gdk_pixbuf(&buffer).unwrap();
    pixbuf
}

fn image_buffer_to_gdk_pixbuf(
    buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Result<Pixbuf, Box<dyn std::error::Error>> {
    let width = buffer.width() as i32;
    let height = buffer.height() as i32;
    let rowstride = width * 4;

    let pixbuf = Pixbuf::from_bytes(
        &glib::Bytes::from(&buffer.as_flat_samples().as_slice()),
        Colorspace::Rgb,
        true,
        8,
        width,
        height,
        rowstride,
    );

    Ok(pixbuf)
}
