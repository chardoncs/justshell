use gtk::{prelude::*, Application, ApplicationWindow};
use url::Url;
use webkit2gtk::{UserContentManager, WebContext, WebView, WebViewExt, WebViewExtManual};

pub fn new_browser_window(app: &Application, url: &Url) -> ApplicationWindow {
    let ctx = WebContext::default().unwrap_or_else(|| WebContext::new());

    let webview = WebView::new_with_context_and_user_content_manager(&ctx, &UserContentManager::new());

    let window = ApplicationWindow::builder()
        .application(app)
        .maximized(true)
        .title("JustShell")
        .build();

    window.present();

    webview.load_uri(url.to_string().as_str());

    window
}
