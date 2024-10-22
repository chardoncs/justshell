use clap::Parser;
use cli::Cli;
use error::{Error, ErrorKind};
use gtk::{gio::{self, ApplicationFlags}, glib, prelude::*, Application};
use gui::{browser_window::new_browser_window, load_css, url_dialog::UrlDialog};
use url::Url;
use glib::closure_local;

mod cli;
mod error;
mod gui;

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let url: Option<Url> = if let Some(url) = args.url {
        Some(Url::parse(url.as_str())
            .or(Err(Error::new(ErrorKind::UrlProcessing, "Invalid URL")))?)
    } else {
        None
    };

    gio::resources_register_include!("justshell-ui.gresource")
        .expect("Failed to register resources");

    let app = Application::builder()
        .application_id("com.chardoncs.justshell")
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_startup(|_| load_css());

    app.connect_activate(move |app| {
        if let Some(url) = url.as_ref() {
            new_browser_window(app, url);
        } else {
            let dialog = UrlDialog::new(app);
            dialog.present();

            let app_ref = app.clone();

            dialog.connect_closure("proceed", false, closure_local!(move |dialog: UrlDialog, url: String| {
                new_browser_window(&app_ref, &Url::parse(url.as_str()).unwrap());
                dialog.close();
            }));
        }
    });

    app.connect_open(|app, _, _| {
        app.activate();
    });

    if app.run().value() != 0 {
        Err(Error::new(ErrorKind::Gui, "Window exited with error"))?;
    }

    Ok(())
}
