use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(help = "Target URL. The URL dialog launches if it is absent")]
    pub url: Option<String>,
}
