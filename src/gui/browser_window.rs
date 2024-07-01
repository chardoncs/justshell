use gtk::{prelude::*, Application, ApplicationWindow};
use url::Url;

pub fn new_browser_window(app: &Application, url: &Url) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .maximized(true)
        .title("JustShell")
        .build();

    window.present();
    window
}
