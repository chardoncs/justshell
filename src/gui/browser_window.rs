use gtk::{prelude::*, Application, ApplicationWindow};
use url::Url;
use webkit6::{prelude::*, WebView};

pub fn new_browser_window(app: &Application, url: &Url) -> ApplicationWindow {
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
        window_ref.set_title(if let Some(ref title_gstr) = webview.title() { Some(title_gstr.as_str()) } else { None });
    });

    let window_ref2 = window.clone();

    webview.connect_close(move |_| {
        window_ref2.close();
    });

    webview.load_uri(url.as_str());

    window.present();
    window
}
