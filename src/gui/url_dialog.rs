use gtk::{gdk::Key, gio, glib::{self, clone, Object}, prelude::*, subclass::prelude::*, Application, EventControllerKey, ShortcutController};
use url::Url;

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

    fn setup_callbacks(&self) {
        self.imp()
            .cancel_button
            .connect_clicked(clone!(@weak self as dialog => move |_| {
                dialog.close();
            }));

        self.imp()
            .ok_button
            .connect_clicked(clone!(@weak self as dialog => move |_| {
                dialog.emit_by_name::<()>("proceed", &[&dialog.imp().entry.text().trim().to_string()]);
            }));

        self.imp()
            .entry
            .connect_text_notify(clone!(@weak self as dialog => move |entry| {
                dialog.imp().ok_button.set_sensitive(
                    Url::parse(entry.text().trim()).is_ok()
                );
            }));
    }
}
