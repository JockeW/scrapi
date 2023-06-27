mod args;

use args::{Presentation, RScrapeArgs};
use clap::{Arg, ArgMatches, Command, Parser};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use colored::Colorize;
use inquire::{Confirm, Text};
use scraper::{ElementRef, Html, Node, Selector};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read, Write};

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
    println!("SAVING SCRAPE");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open("scrapers.txt")
        .unwrap();

    println!("TEST 1");

    let buff_reader = BufReader::new(file);
    println!("TEST 2");

    //TODO: Check that the name is unique
    let mut lines: Vec<String> = Vec::new();
    for line in buff_reader.lines() {
        match line {
            Ok(l) => lines.push(l),
            Err(e) => println!("ERROR: {}", e)
        }
    }
    println!("LINES: {:?}", lines);
    let saved_scrapes: Vec<&str> = lines.iter().map(|l| l.as_str()).collect();

    let scrape_names: Vec<&str> = saved_scrapes
        .iter()
        .map(|s| s.split('|').collect::<Vec<&str>>()[0])
        .collect();

    println!("SCRAPE NAMES: {:?}", scrape_names);

    if scrape_names.contains(&name.to_lowercase().as_str()) {}

    //TODO: Maybe store scrape as JSON. Could possibly be easier to combine scrapers later, and read scrapers from the file etc. See scrapers.json file

    // writeln!(
    //     file,
    //     "{}|{}|{:?}|{:?}|{:?}|{:?}",
    //     name, url, selectors, keys, title, present
    // )
    // .unwrap();
}
