use clap::{Args, Parser, Subcommand};
use std::fmt;
use std::str::FromStr;

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

    // List all saved scrapes
    // List(ListCommand)
    /// Combine saved scrapes
    Combine(CombineCommand),

    // Update saved scrape. Same args as Scrape command except save arg.
    // Update(UpdateCommand)

    // Scrape and inspect. Args for url, search and filter
    // Inspect(InspectCommand)
    /// Run saved scrape
    Run(RunCommand),
    // Delete saved scrape. OBS!! HANDLE SAVED COMBINED SCRAPES THAT MIGHT BE USING THIS SCRAPE
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

impl FromStr for Presentation {
    type Err = ();

    fn from_str(input: &str) -> Result<Presentation, Self::Err> {
        match input {
            "List" => Ok(Presentation::List),
            "Table" => Ok(Presentation::Table),
            _ => Err(()),
        }
    }
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
pub struct RunCommand {
    #[arg(required = true)]
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
