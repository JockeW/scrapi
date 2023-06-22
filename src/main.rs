mod args;

use args::{Presentation, RScrapeArgs};
use clap::{Arg, ArgMatches, Command, Parser};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use colored::Colorize;
use inquire::{Confirm, Text};
use scraper::{ElementRef, Html, Node, Selector};
use std::fs::OpenOptions;
use std::io::{Write, Read, BufReader, BufRead};

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
        _ => {}
    }
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
        println!("SELECTOR: {}", s);
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

    match present {
        Some(Presentation::List) => {
            print_content_list(all_content, keys.iter().map(|k| k.as_str()).collect());
        }
        Some(Presentation::Table) => {
            print_content_table(all_content, keys.iter().map(|k| k.as_str()).collect());
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
                Ok(true) => save_scrape(&save, &url, selectors, keys, title, present),
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
    present: Option<Presentation>,
) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("scrapers.txt")
        .unwrap();

    let buff_reader = BufReader::new(file);

    //TODO: Check that the name is unique
    let lines: Vec<String> = buff_reader.lines().filter_map(|l| l.ok()).collect();
    let saved_scrape_names: Vec<&str> = lines.iter().map(|l| l.split('|').collect::<Vec<&str>>()[0].to_lowercase()).collect();

    if saved_scrape_names.contains(&name.to_lowercase()) {

    }

    //TODO: Maybe store scrape as JSON. Could possibly be easier to combine scrapers later, and read scrapers from the file etc. See scrapers.json file

    writeln!(
        file,
        "{} |{} |{:?} |{:?} |{:?} |{:?}",
        name, url, selectors, keys, title, present
    )
    .unwrap();
}
