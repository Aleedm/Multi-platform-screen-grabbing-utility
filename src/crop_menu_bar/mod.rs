mod imp;
use gtk4 as gtk;

use gtk::glib;

glib::wrapper! {
    pub struct CropMenuBar(ObjectSubclass<imp::CropMenuBar>)
        @extends gtk::Widget;
}

#[gtk::template_callbacks]
impl CropMenuBar {
}