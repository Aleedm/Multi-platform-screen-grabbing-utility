use std::{cell::RefCell, time::Duration};

use crate::add_window::screenshot::screenshot;
use gtk::{
    glib::{self},
    prelude::*,
    subclass::prelude::*,
};
use gtk4 as gtk;
use std::thread;
/// The private struct, which can hold widgets and other data.
#[derive(Debug, gtk::CompositeTemplate)]
#[template(file = "add_window.ui")]
pub struct AddWindow {
    // The #[template_child] attribute tells the CompositeTemplate macro
    // that a field is meant to be a child within the template.
    #[template_child]
    pub delay_opt: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub shortcut: TemplateChild<gtk::Button>,
    #[template_child]
    pub add_ss: TemplateChild<gtk::Button>,
    #[template_child]
    pub image: TemplateChild<gtk::Image>,
    pub delay: RefCell<u64>,
}

impl Default for AddWindow {
    fn default() -> Self {
        Self {
            delay_opt: Default::default(),
            shortcut: Default::default(),
            add_ss: Default::default(),
            image: Default::default(),
            delay: RefCell::new(2),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for AddWindow {
    const NAME: &'static str = "AddWindow";
    type Type = super::AddWindow;
    type ParentType = gtk::ApplicationWindow;

    // Within class_init() you must set the template.
    // The CompositeTemplate derive macro provides a convenience function
    // bind_template() to set the template and bind all children at once.
    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        //UtilityCallbacks::bind_template_callbacks(klass);
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

struct UtilityCallbacks {}

#[gtk::template_callbacks(functions)]
impl UtilityCallbacks {}

impl ObjectImpl for AddWindow {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().setup_menu();
        let window_clone = self.obj().clone();
        let image_clone = self.image.clone();
        //self.obj().update_timer(0);

        // Connect to "clicked" signal of `button`
        self.add_ss.connect_clicked(move |_| {
            let delay = window_clone.get_delay();
            println!("delay: {}", delay);
            eprintln!("Clicked!");
            window_clone.hide();
            while glib::MainContext::default().iteration(false) {}
            //TODO timer
            eprintln!("waiting {:?} seconds", delay);
            let five_seconds = Duration::from_secs(delay);
            thread::sleep(five_seconds);
            eprintln!("waited {:?} seconds", delay);
            //image_clone.set_from_pixbuf(Some(&screenshot()));
            screenshot();
            image_clone.set_from_file(Some("./target/prova.png"));
            window_clone.show();
            window_clone.present();
        });
    }
}

impl WidgetImpl for AddWindow {}
impl WindowImpl for AddWindow {}
impl ApplicationWindowImpl for AddWindow {}
