mod args;

use args::{Presentation, RScrapeArgs};
use clap::Parser;
use cli_table::{Cell, Style, Table};
use colored::Colorize;
use inquire::Confirm;
use scraper::{ElementRef, Html, Node, Selector};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let args = RScrapeArgs::parse();

    match args.sub_command {
        args::RScrapeCommand::Scrape(cmd) => scrape(
            cmd.url,
            cmd.selectors,
            cmd.keys,
            cmd.title,
            cmd.save,
            cmd.present,
        ),
        args::RScrapeCommand::Check(cmd) => check(cmd.name),
        args::RScrapeCommand::Run(cmd) => run(cmd.name),
        args::RScrapeCommand::Combine(cmd) => combine(cmd.name, cmd.scrapes),
        _ => {}
    }
}

fn get_saved_scrape(name: &str) -> Option<&'static str> {
    let file_content = include_str!("../scrapes.txt");

    for line in file_content.trim().split('\n') {
        let scrape_name = if line.split(';').collect::<Vec<&str>>()[0] == "combined" {
            line.split(';').collect::<Vec<&str>>()[1]
        } else {
            line.split(';').collect::<Vec<&str>>()[0]
        };

        if scrape_name == name {
            return Some(line);
        }
    }

    None
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

    let title: Option<String> = if data[4].len() > 0 {
        Some(data[4].to_string())
    } else {
        None
    };

    let presentation = if data[5].to_lowercase() == "table" {
        Some(Presentation::Table)
    } else {
        Some(Presentation::List)
    };

    scrape(url.to_string(), selectors, keys, title, None, presentation);
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

fn check(name: String) {
    let scrape_data = get_saved_scrape(&name);

    match scrape_data {
        Some(data) => print_scrape_info(data),
        None => println!("Scrape {} not found!", &name),
    }
}

fn print_scrape_info(data_str: &str) {
    let data: Vec<&str> = data_str.split(";").collect();

    //TODO: Handle print for combined scrape

    println!("Name: {}", data[0]);
    println!("Url: {}", data[1]);
    println!("Selectors: {}", data[2]);
    println!("Keys: {}", data[3]);

    println!("Title: {}", data[4]);
    println!("Present: {}", data[5]);

    let selectors: String = data[2].replace("[", "").replace("]", "").replace(",", "");
    let keys: String = data[3].replace("[", "").replace("]", "").replace(",", "");
    let title: String = if data[4].len() > 0 {
        format!(" --title \"{}\"", data[4])
    } else {
        "".to_string()
    };

    println!(
        "Full command: {}",
        format!(
            "scrape --url {} --selectors {} --keys {}{} --present {}",
            data[1], selectors, keys, title, data[5]
        )
    );
}

fn scrape(
    url: String,
    selectors: Vec<String>,
    keys: Vec<String>,
    title: Option<String>,
    save: Option<String>,
    present: Option<Presentation>,
) {
    if keys.len() != selectors.len() {
        println!(
            "{}: Keys needs to be as many as selectors",
            "error".bold().color("red")
        );
        return;
    }

    let html = reqwest::blocking::get(&url).unwrap().text().unwrap();
    let document = Html::parse_document(&html);

    let mut contents: Vec<Vec<String>> = Vec::new();

    for s in &selectors {
        let selector = Selector::parse(&s).expect("Not a valid selector");
        let element_ref: Vec<ElementRef> = document.select(&selector).collect();

        let mut content_vec: Vec<String> = Vec::new();

        for element in element_ref {
            let outer_text: Vec<&str> = element
                .children()
                .filter_map(|node| match node.value() {
                    Node::Text(text) => Some(&text[..]),
                    _ => None,
                })
                .collect();

            //println!("{:?}", outer_text);
            //TODO: Maybe add to get text of child nodes as well. (element.children())

            let element_text: String = outer_text.join("");

            content_vec.push(element_text);
        }

        contents.push(content_vec);
    }

    let mut all_content: Vec<Vec<&str>> = Vec::new();

    for content_index in 0..contents.first().expect("NO CONTENT").len() {
        let mut chunk: Vec<&str> = Vec::new();
        for content in contents.iter() {
            let value = content[content_index].trim();
            chunk.push(value);
        }

        all_content.push(chunk);
    }

    println!();

    if let Some(title) = &title {
        println!("{}", title.bold());
        println!();
    }

    let mut selected_presentation = Presentation::List;

    match present {
        Some(Presentation::List) => {
            print_content_list(all_content, keys.iter().map(|k| k.as_str()).collect());
            selected_presentation = Presentation::List;
        }
        Some(Presentation::Table) => {
            print_content_table(all_content, keys.iter().map(|k| k.as_str()).collect());
            selected_presentation = Presentation::Table;
        }
        None => {
            //Printing list as default. TODO: Maybe prompt and ask for list or table in this case instead.
            print_content_list(all_content, keys.iter().map(|k| k.as_str()).collect());
        }
    }

    if let Some(save) = save {
        if !save.is_empty() {
            let answer = Confirm::new("Are you sure you want to save this scrape?")
                .with_default(false)
                .with_help_message(
                    "All arguments will be saved so the scrape can be reused with 'run' command.",
                )
                .prompt();

            match answer {
                Ok(true) => save_scrape(&save, &url, selectors, keys, title, selected_presentation),
                Ok(false) => println!("Skipped saving"),
                Err(_) => println!("Error with questionnaire, try again later"),
            }
        }
    }
}

fn print_content_list(content: Vec<Vec<&str>>, keys: Vec<&str>) {
    for chunk in content {
        for (i, data) in chunk.iter().enumerate() {
            let header = keys[i];
            let value = data;
            println!("{}: {}", header.bold(), value);
        }
        println!();
    }
}

fn print_content_table(content: Vec<Vec<&str>>, keys: Vec<&str>) {
    let table = content
        .table()
        .title(keys.iter().map(|k| k.cell().bold(true)))
        .bold(true);

    let table_display = table.display().unwrap();

    println!("{}", table_display);
}

fn save_scrape(
    name: &str,
    url: &str,
    selectors: Vec<String>,
    keys: Vec<String>,
    title: Option<String>,
    present: Presentation,
) {
    println!("Saving scrape...");
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
    println!("LINES: {:?}", lines);
    let saved_scrapes: Vec<&str> = lines.iter().map(|l| l.as_str()).collect();

    let scrape_names: Vec<&str> = saved_scrapes
        .iter()
        .map(|s| s.split(';').collect::<Vec<&str>>()[0])
        .collect(); //TODO: Change scrape_names to contain names of combined scrapes instead of just "combined"

    println!("SCRAPE NAMES: {:?}", scrape_names);

    if scrape_names.contains(&name.to_lowercase().as_str()) {
        println!("There is already a scrape with that name: '{}'", name);
        //TODO: Prompt user with options for entering a new name or cancel
    } else if name.to_lowercase() == "combined" {
        println!("'combined' is a reserved word");
    } else {
        let title_to_write = match title {
            Some(t) => t,
            None => "".to_string(),
        };

        writeln!(
            file,
            "{};{};{:?};{:?};{};{}",
            name,
            url,
            selectors,
            keys,
            title_to_write,
            present.to_string().to_lowercase()
        )
        .unwrap();
    }
}
