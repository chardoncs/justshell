use gtk::{prelude::*, gio::{self, ActionEntry}, glib::{self, prelude::*, property::{PropertyGet, PropertySet}, Object}, subclass::prelude::*, Application};

mod imp;

glib::wrapper! {
    pub struct UrlDialog(ObjectSubclass<imp::UrlDialog>)
        @extends gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible,
                    gtk::Buildable, gtk::ConstraintTarget, gtk::Native,
                    gtk::Root, gtk::ShortcutManager;
}

impl UrlDialog {
    pub fn new(app: &Application) -> Self {
        Object::builder()
            .property("application", app)
            .build()
    }
}
