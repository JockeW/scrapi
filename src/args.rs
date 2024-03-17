use crate::enums::Presentation;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RScrapeArgs {
    #[clap(subcommand)]
    pub sub_command: RScrapeCommand,
}

#[derive(Debug, Subcommand)]
pub enum RScrapeCommand {
    /// Scrape a website and present the data
    Scrape(ScrapeCommand),

    /// Check command, to check saved scrape and get all saved data presented nicely
    Check(CheckCommand),

    /// List the name of all saved scrapes
    List(ListCommand),
    /// Combine saved scrapes
    Combine(CombineCommand),

    // Update saved scrape. Same args as Scrape command except save arg.
    // Update(UpdateCommand)

    // Scrape and inspect. Args for url, search and filter
    //Inspect(InspectCommand),
    /// Run saved scrape
    Run(RunCommand),

    /// Delete saved scrape. OBS!! HANDLE SAVED COMBINED SCRAPES THAT MIGHT BE USING THIS SCRAPE
    Delete(DeleteCommand)
}

#[derive(Debug, Args)]
pub struct CombineCommand {
    #[arg(short, long, required = true)]
    pub name: String,
    #[arg(short, long, num_args(1..), required = true)]
    pub scrapes: Vec<String>,
}

#[derive(Debug, Args)]
pub struct ScrapeCommand {
    #[arg(short, long, required = true)]
    pub url: String,
    #[arg(short, long, num_args(1..), required = true)]
    pub selectors: Vec<String>,
    #[arg(short, long, num_args(1..), required = true)]
    pub keys: Vec<String>,
    #[arg(short, long, num_args(0..), required = false, help = "Attribute to get for specific selector. One attribute per selector. Format: <selector_index>:<attribute>")]
    pub attributes: Option<Vec<String>>,
    #[arg(long, num_args(0..), required = false, help = "Prefixes to add for specific selector values. One prefix per selector. Format: <selector_index>:<prefix>")]
    pub prefixes: Option<Vec<String>>,
    #[arg(long, num_args(0..), required = false, help = "Suffixes to add for specific selector values. One suffix per selector. Format: <selector_index>:<suffix>")]
    pub suffixes: Option<Vec<String>>,
    #[arg(short, long, required = false)]
    pub title: Option<String>,
    #[arg(long, required = false)]
    pub save: Option<String>,
    #[arg(long, required = false)]
    pub present: Option<Presentation>,
}

#[derive(Debug, Args)]
pub struct CheckCommand {
    #[arg(required = true)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct DeleteCommand {
    #[arg(required = true)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct ListCommand {}

#[derive(Debug, Args)]
pub struct RunCommand {
    #[arg(required = true)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct InspectCommand {
    #[arg(short, long, required = true)]
    pub url: String,
    #[arg(short, long, required = false)]
    pub filter: Option<String>,
    #[arg(short, long, required = false)]
    pub search: Option<String>,
}
