use gtk::{prelude::*, Application, ApplicationWindow};
use url::Url;
use webkitgtk::{prelude::*, WebView};

pub fn new_browser_window(app: &Application, url: &Url) {
    let webview = WebView::builder()
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .maximized(true)
        .title("JustShell")
        .child(&webview)
        .build();

    let window_ref = window.clone();

    webview.connect_title_notify(move |webview| {
        window_ref.set_title(if let Some(ref title_gstr) = webview.title() {
            Some(title_gstr.as_str())
        } else {
            Some("(no title) - JustShell")
        });
    });

    let window_ref = window.clone();

    webview.connect_close(move |_| {
        window_ref.close();
    });

    webview.load_uri(url.as_str());

    window.present();
}
