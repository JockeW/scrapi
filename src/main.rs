mod args;
mod utils;

use args::{Presentation, RScrapeArgs};
use clap::Parser;
use scraper::Html;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use utils::get_saved_scrape;

pub mod commands;

fn main() {
    let args = RScrapeArgs::parse();

    match args.sub_command {
        args::RScrapeCommand::Scrape(cmd) => commands::scrape::scrape(
            cmd.url,
            cmd.selectors,
            cmd.keys,
            cmd.attributes,
            cmd.title,
            cmd.save,
            cmd.present,
        ),
        args::RScrapeCommand::Check(cmd) => commands::check::check(cmd.name),
        args::RScrapeCommand::Run(cmd) => run(cmd.name),
        args::RScrapeCommand::Combine(cmd) => combine(cmd.name, cmd.scrapes),
        args::RScrapeCommand::Html(cmd) => html(cmd.url), //TODO: Have as export option for Scrape and Run commands
    }
}

fn html(url: String) {
    let html = reqwest::blocking::get(&url).unwrap().text().unwrap();
    let document = Html::parse_document(&html);

    let data = document.html();

    fs::write("foo.html", data).expect("Unable to write file");
}

fn combine(name: String, scrapes: Vec<String>) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open("scrapes.txt")
        .unwrap();

    let buff_reader = BufReader::new(&file);

    let mut lines: Vec<String> = Vec::new();
    for line in buff_reader.lines() {
        match line {
            Ok(l) => lines.push(l),
            Err(e) => println!("ERROR: {}", e),
        }
    }

    let saved_scrapes: Vec<&str> = lines.iter().map(|l| l.as_str()).collect();

    let scrape_names: Vec<&str> = saved_scrapes
        .iter()
        .map(|s| s.split(';').collect::<Vec<&str>>()[0])
        .collect(); //TODO: Change scrape_names to contain names of combined scrapes instead of just "combined"

    if scrape_names.contains(&name.to_lowercase().as_str()) {
        println!("There is already a scrape with that name: '{}'", name);
        //TODO: Prompt user with options for entering a new name or cancel
    } else if name.to_lowercase() == "combined" {
        println!("'combined' is a reserved word");
    } else {
        let mut all_scrapes_exists = true;
        for scrape in &scrapes {
            if !scrape_names.contains(&scrape.as_str()) {
                println!("There is no saved scrape: '{}'", scrape);
                all_scrapes_exists = false;
            }
        }

        if all_scrapes_exists {
            writeln!(file, "combined;{};{:?}", name, scrapes).unwrap();
        }
    }
}

fn run(name: String) {
    let scrape_data = get_saved_scrape(&name);

    match scrape_data {
        Some(data_str) => {
            let data: Vec<&str> = data_str.split(";").collect();

            match data[0] {
                "combined" => {
                    let scrapes = data[2][1..data[2].len() - 1]
                        .split(", ")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|&s| s.trim().replace("\"", "").to_string())
                        .collect::<Vec<String>>();

                    run_combined_scrapes(scrapes);
                }
                _ => run_scrape(data),
            }
        }
        None => println!("Scrape {} not found!", &name),
    }
}

fn run_scrape(data: Vec<&str>) {
    let url = data[1];
    let selectors = data[2][1..data[2].len() - 1]
        .split(", ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.trim().replace("\"", "").to_string())
        .collect::<Vec<String>>();

    let keys = data[3][1..data[3].len() - 1]
        .split(", ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.trim().replace("\"", "").to_string())
        .collect::<Vec<String>>();

    let attributes: Option<Vec<String>> = if data[4].len() > 2 {
        Some(
            data[4][1..data[4].len() - 1]
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|&s| s.trim().replace("\"", "").to_string())
                .collect::<Vec<String>>(),
        )
    } else {
        None
    };

    let title: Option<String> = if data[5].len() > 0 {
        Some(data[5].to_string())
    } else {
        None
    };

    let presentation = if data[6].len() > 0 {
        if data[6].to_lowercase() == "table" {
            Some(Presentation::Table)
        } else {
            Some(Presentation::List)
        }
    } else {
        None
    };

    commands::scrape::scrape(
        url.to_string(),
        selectors,
        keys,
        attributes,
        title,
        None,
        presentation,
    );
}

fn run_combined_scrapes(scrapes: Vec<String>) {
    let mut saved_scrapes: Vec<Vec<&str>> = Vec::new();

    for scrape in &scrapes {
        let saved_scrape = get_saved_scrape(scrape);

        match saved_scrape {
            Some(data_str) => {
                let data: Vec<&str> = data_str.split(";").collect();
                saved_scrapes.push(data);
            }
            None => {
                println!("Scrape was not found: '{}'", scrape);
            }
        }
    }

    if saved_scrapes.len() == scrapes.len() {
        for scrape in saved_scrapes {
            run_scrape(scrape);
        }
    }
}
