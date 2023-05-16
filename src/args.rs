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
    #[arg(short, long, required = true)]
    pub url: String,
    #[arg(short, long, num_args(1..), required = true)]
    pub selectors: Vec<String>,
    #[arg(short, long, num_args(1..), required = true)]
    pub keys: Vec<String>,
    #[arg(short, long)]
    pub title: Option<String>
}
