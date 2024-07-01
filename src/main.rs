use clap::Parser;
use cli::Cli;
use error::{Error, ErrorKind};
use gtk::{prelude::{ApplicationExt, ApplicationExtManual}, Application};
use gui::browser_window::new_browser_window;
use url::Url;
use util::process_url;

mod cli;
mod error;
mod gui;
mod util;

const APP_ID: &'static str = "com.chardoncs.justshell";

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let url: Option<Url> = if let Some(url) = args.url {
        Some(process_url(url.as_str())?)
    } else {
        None
    };

    if let Some(url) = url {
        let app = Application::builder()
            .application_id(APP_ID)
            .build();

        app.connect_activate(move |app| {
            let _ = new_browser_window(app, &url);
        });

        let e = app.run();

        if e.value() != 0 {
            Err(Error::new(ErrorKind::Gui, "GTK exited unexpectedly."))?;
        }

    } else {
        return Err(Error::new(ErrorKind::UrlAbsent, "A URL is required."));
    }

    Ok(())
}
