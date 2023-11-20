use crate::first_menu_bar::FirstMenuBar;
use crate::crop_menu_bar::CropMenuBar;
use crate::edit_shortcut::EditShortCut;
use gtk::{
    glib::{self},
    subclass::prelude::*, prelude::WidgetExt,
};
use gtk4 as gtk;
use std::cell::RefCell;
use gtk::gdk_pixbuf::{Pixbuf, Colorspace};
use crate::utility::ImageOffset;

/// The private struct, which can hold widgets and other data.
#[derive(Debug, gtk::CompositeTemplate)]
#[template(resource = "/org/mpsgu/main_window.ui")]
pub struct MainWindow {
    // The #[template_child] attribute tells the CompositeTemplate macro
    // that a field is meant to be a child within the template.
    #[template_child]
    pub menubar: TemplateChild<FirstMenuBar>,
    #[template_child]
    pub prova: TemplateChild<EditShortCut>,
    #[template_child]
    pub cropbar: TemplateChild<CropMenuBar>,
    #[template_child]
    pub image: TemplateChild<gtk::Picture>,
    #[template_child]
    pub overlay: TemplateChild<gtk::Overlay>,
    #[template_child]
    pub drawing_area: TemplateChild<gtk::DrawingArea>,

    pub appl: RefCell<gtk::Application>,

    pub pixbuf: RefCell<Pixbuf>,

    pub image_offset: RefCell<ImageOffset>
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            prova: Default::default(),
            menubar: Default::default(),
            cropbar: Default::default(),
            image: Default::default(),
            appl: Default::default(),
            drawing_area: Default::default(),
            overlay: Default::default(),
            pixbuf: RefCell::new(Pixbuf::new(Colorspace::Rgb, true, 8, 10, 10).unwrap()),
            image_offset: RefCell::new(ImageOffset { x: 0, y: 0, aspect_ratio: 0.0 })
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    // Within class_init() you must set the template.
    // The CompositeTemplate derive macro provides a convenience function
    // bind_template() to set the template and bind all children at once.
    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        UtilityCallbacks::bind_template_callbacks(klass);
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

struct UtilityCallbacks {}

#[gtk::template_callbacks(functions)]
impl UtilityCallbacks {}

impl ObjectImpl for MainWindow {
    fn constructed(&self) {
        self.parent_constructed();
        self.cropbar.hide();
        self.prova.hide();
        self.obj().delay_action_setup();
        self.obj().screen_action_setup();
        self.obj().save_action_setup();
        self.obj().copy_action_setup();
        self.obj().crop_action_setup();
        self.obj().setup_size_allocate();
        self.obj().exit_action_setup();
        self.obj().confirm_action_setup();
    }
}

impl WidgetImpl for MainWindow {}
impl WindowImpl for MainWindow {}
impl ApplicationWindowImpl for MainWindow {}
