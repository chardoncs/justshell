use clap::Parser;
use cli::Cli;
use error::{Error, ErrorKind};
use gtk::{gio::ApplicationFlags, prelude::{ApplicationExt, ApplicationExtManual}, Application};
use gui::browser_window::new_browser_window;
use url::Url;

mod cli;
mod error;
mod gui;

const APP_ID: &'static str = "com.chardoncs.justshell";

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let url: Option<Url> = if let Some(url) = args.url {
        Some(Url::parse(url.as_str())
            .or(Err(Error::new(ErrorKind::UrlProcessing, "Invalid URL")))?)
    } else {
        None
    };

    let app = Application::builder()
        .application_id(APP_ID)
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_activate(move |app| {
        if let Some(url) = url.as_ref() {
            let _ = new_browser_window(app, url);
        }
    });

    app.connect_open(move |app, _, _| {
        app.activate();
    });

    let e = app.run();

    if e.value() != 0 {
        Err(Error::new(ErrorKind::Gui, "GTK exited unexpectedly."))?;
    }

    Ok(())
}
