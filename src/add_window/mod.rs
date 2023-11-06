mod imp;
mod screenshot;
use gtk::{gio, glib, prelude::*};
use gtk4 as gtk;

glib::wrapper! {
    pub struct AddWindow(ObjectSubclass<imp::AddWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl AddWindow {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}