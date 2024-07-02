use gtk::{gdk::Display, CssProvider};

pub mod browser_window;
pub mod url_dialog;

pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("../ui/style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a GDK display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
