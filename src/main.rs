use clap::Parser;
use cli::Cli;
use error::{Error, ErrorKind};
use url::Url;
use util::process_url;

mod cli;
mod error;
mod gui;
mod util;

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let url: Option<Url> = if let Some(url) = args.url {
        Some(process_url(url.as_str())?)
    } else {
        None
    };

    if url.is_none() {
        return Err(Error::new(ErrorKind::UrlAbsent, "A URL is required."));
    }

    Ok(())
}
