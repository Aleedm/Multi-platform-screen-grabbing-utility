
use crate::first_menu_bar::FirstMenuBar;
use gtk::{
    glib::{self},
    subclass::prelude::*,
};


use gtk4 as gtk;

/// The private struct, which can hold widgets and other data.
#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "main_window.ui")]
pub struct MainWindow {
    // The #[template_child] attribute tells the CompositeTemplate macro
    // that a field is meant to be a child within the template.
    #[template_child]
    pub menubar: TemplateChild<FirstMenuBar>,
    #[template_child]
    pub add_ss: TemplateChild<gtk::Button>,
    #[template_child]
    pub image: TemplateChild<gtk::Image>
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
        self.obj().delay_action_setup();
        self.obj().screen_action_setup()
        //let window_clone = self.obj().clone();
        //let menu_clone = self.menubar.clone();
        //let image_clone = self.image.clone();
        //self.obj().update_timer(0);

        /*  Connect to "clicked" signal of `button`
        self.add_ss.connect_clicked(move |_| {
            let delay = menu_clone.get_delay();
            println!("delay: {}", delay);
            eprintln!("Clicked!");
            window_clone.hide();
           // while glib::MainContext::default().iteration(false) {}
            //TODO timer
            eprintln!("waiting {:?} seconds", delay);
            if delay > 0 {
                let sleep_duration = Duration::from_secs(delay);
                thread::sleep(sleep_duration);
                eprintln!("waited {:?} seconds", sleep_duration);
            }
            //image_clone.set_from_pixbuf(Some(&screenshot()));
            screenshot();
            image_clone.set_from_file(Some("./target/prova.png"));
            window_clone.show();
            window_clone.present();
        }); */
    }
}

impl WidgetImpl for MainWindow {}
impl WindowImpl for MainWindow {}
impl ApplicationWindowImpl for MainWindow {}
