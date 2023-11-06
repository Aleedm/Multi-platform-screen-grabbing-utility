use gtk::{glib, prelude::*, subclass::prelude::*};
use crate::add_window::ss::screenshot;
use gtk4 as gtk;
/// The private struct, which can hold widgets and other data.
#[derive(Debug, Default, gtk::CompositeTemplate)]
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
    pub image: TemplateChild<gtk::Image>
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
impl UtilityCallbacks {

} 

impl ObjectImpl for AddWindow {
    fn constructed(&self) {
        self.parent_constructed();
        
        let image_clone = self.image.clone();

        // Connect to "clicked" signal of `button`
        self.add_ss.connect_clicked(move |_| {
            eprintln!("Clicked!");
            image_clone.set_from_pixbuf(Some(&screenshot()));
        });
    }

    
}

impl WidgetImpl for AddWindow {}
impl WindowImpl for AddWindow {}
impl ApplicationWindowImpl for AddWindow {}