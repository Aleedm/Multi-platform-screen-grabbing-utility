use crate::crop_menu_bar::CropMenuBar;
use crate::settings_manager::Settings;
use crate::utility::ImageOffset;
use crate::{first_menu_bar::FirstMenuBar, utility::CropArea};
use gtk::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::{
    glib::{self},
    prelude::WidgetExt,
    subclass::prelude::*,
};
use gtk4 as gtk;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, gtk::CompositeTemplate)]
#[template(resource = "/org/mpsgu/main_window.ui")]
pub struct MainWindow {
    #[template_child]
    pub menubar: TemplateChild<FirstMenuBar>,
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

    pub image_offset: RefCell<ImageOffset>,

    pub crop_area: RefCell<CropArea>,

    pub crop_mode_active: Rc<RefCell<bool>>,

    pub crop_mode: Rc<RefCell<i8>>, // 0 = crop, 1 = select

    pub side_selected: Rc<RefCell<i8>>,

    pub settings_manager: RefCell<Option<Settings>>,
    
    pub monitors: RefCell<Vec<String>>,
    pub current_monitor: RefCell<u32>
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            menubar: Default::default(),
            cropbar: Default::default(),
            image: Default::default(),
            appl: Default::default(),
            drawing_area: Default::default(),
            overlay: Default::default(),
            pixbuf: RefCell::new(Pixbuf::new(Colorspace::Rgb, true, 8, 10, 10).unwrap()),
            image_offset: RefCell::new(ImageOffset::new()),
            crop_area: RefCell::new(CropArea::new()),
            crop_mode_active: Rc::new(RefCell::new(false)),
            crop_mode: Rc::new(RefCell::new(-1)),
            side_selected: Rc::new(RefCell::new(-1)),
            settings_manager: RefCell::new(Settings::read_settings("config.json".to_string())),
            monitors: RefCell::new(Vec::new()),
            current_monitor: RefCell::new(0)
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        UtilityCallbacks::bind_template_callbacks(klass);
    }

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
        self.obj().delay_action_setup();
        self.obj().screen_action_setup();
        self.obj().save_action_setup();
        self.obj().copy_action_setup();
        self.obj().crop_action_setup();
        self.obj().setup_size_allocate();
        self.obj().exit_action_setup();
        self.obj().confirm_action_setup();
        self.obj().settings_setup();
        self.obj().setup_monitors();
    }
}

impl WidgetImpl for MainWindow {}
impl WindowImpl for MainWindow {}
impl ApplicationWindowImpl for MainWindow {}
