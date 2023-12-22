use std::{fmt, str::FromStr};

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Presentation {
    List,
    Table,
}

impl fmt::Display for Presentation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Presentation::List => write!(f, "list"),
            Presentation::Table => write!(f, "table"),
        }
    }
}

impl FromStr for Presentation {
    type Err = ();

    fn from_str(input: &str) -> Result<Presentation, Self::Err> {
        match input {
            "list" => Ok(Presentation::List),
            "table" => Ok(Presentation::Table),
            _ => Err(()),
        }
    }
}
