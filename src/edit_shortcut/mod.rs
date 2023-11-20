mod imp;
use gtk4 as gtk;

use gtk::glib;

glib::wrapper! {
    pub struct EditShortCut(ObjectSubclass<imp::EditShortCut>)
        @extends gtk::Widget;
}

#[gtk::template_callbacks]
impl EditShortCut {
}