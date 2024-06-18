use gtk::{prelude::*, Application, ApplicationWindow};

pub fn new_browser_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .maximized(true)
        .title("JustShell")
        .build();

    window.present();
    window
}
