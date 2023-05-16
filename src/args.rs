use clap::{Args, Subcommand, Parser};


#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RScrapeArgs {
    #[clap(subcommand)]
    pub subCommand: RScrapeCommand
}

#[derive(Debug, Subcommand)]
pub enum RScrapeCommand {
    /// Scrape a website and present the data
    Scrape(ScrapeCommand),
    // Inspect saved scrapes
    //Inspect(InspectCommand)
}

#[derive(Debug, Args)]
pub struct ScrapeCommand {
    pub url: String,
    pub selectors: Vec<String>,
    pub keys: Vec<String>,
    pub title: Option<String>
}
