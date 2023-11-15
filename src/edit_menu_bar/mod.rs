mod imp;
use gtk4 as gtk;

use gtk::glib;

glib::wrapper! {
    pub struct EditMenuBar(ObjectSubclass<imp::EditMenuBar>)
        @extends gtk::Widget;
}

#[gtk::template_callbacks]
impl EditMenuBar {
}