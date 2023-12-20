mod args;
mod utils;

use args::RScrapeArgs;
use clap::Parser;

pub mod commands;

fn main() {
    let args = RScrapeArgs::parse();

    match args.sub_command {
        args::RScrapeCommand::Scrape(cmd) => commands::scrape::scrape(
            cmd.url,
            cmd.selectors,
            cmd.keys,
            cmd.attributes,
            cmd.prefixes,
            cmd.suffixes,
            cmd.title,
            cmd.save,
            cmd.present,
        ),
        args::RScrapeCommand::Check(cmd) => commands::check::check(cmd.name),
        args::RScrapeCommand::Run(cmd) => commands::run::run(cmd.name),
        args::RScrapeCommand::Combine(cmd) => commands::combine::combine(cmd.name, cmd.scrapes),
        args::RScrapeCommand::Inspect(cmd) => commands::inspect::inspect(cmd.url, cmd.filter, cmd.search)
        
    }
}
