use clap::{Args, Parser, Subcommand};
use std::fmt;

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

    // Update saved scrape. Same args as Scrape command except save arg.
    // Update(UpdateCommand)

    // Scrape and inspect. Args for url, search and filter
    // Inspect(InspectCommand)

    /// Run saved scrape
    Run(RunCommand)

    // Delete saved scrape
    // Delete(DeleteCommand)
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Presentation {
    List,
    Table,
}

impl fmt::Display for Presentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Presentation::List => write!(f, "List"),
            Presentation::Table => write!(f, "Table"),
        }
    }
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
    pub title: Option<String>,
    #[arg(long)]
    pub save: Option<String>,
    #[arg(short, long)]
    pub present: Option<Presentation>,
}

#[derive(Debug, Args)]
pub struct CheckCommand {
    #[arg(short, long, required = true)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct RunCommand {
    #[arg(short, long, required = true)]
    pub name: String,
}

// impl fmt::Display for ScrapeCommand {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         //TODO: Fix selectors and keys. Should not be printed as array!

//         let selectors = self.selectors.iter().map(|val| format!("{}", val)).collect::<Vec<String>>().join(" ");
//         // let mut selectors = String::new();
//         // for selector in &self.selectors {
//         //     let sel = format!("{} ", selector);
//         //     selectors.push_str(selector);
//         // }

//         //let together: Vec<&str> = self.selectors.iter().map(|s| s.as_str()).collect();

//         let title;
//         match &self.title {
//             Some(t) => title = format!(" --title=\"{}\"",t.as_str()),
//             None => title = "".to_string(),
//         }

//         // let save;
//         // match &self.save {
//         //     Some(s) => save = format!(" --save\"{}\"",s.as_str()),
//         //     None => save = "".to_string(),
//         // }

//         let present;
//         match &self.present {
//             Some(p) => {
//                 if *p == Presentation::List {
//                     present = " --present list";
//                 } else {
//                     present = " --present table";
//                 }
//             },
//             None => present = "",
//         }

//         write!(
//             f,
//             "scrape --url {} --selectors {} --keys {:?}{}{}",
//             self.url, selectors, self.keys, title, present
//         )
//     }
// }
