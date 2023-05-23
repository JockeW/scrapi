mod args;

use args::{Presentation, RScrapeArgs};
use clap::{Arg, ArgMatches, Command, Parser};
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use colored::Colorize;
use inquire::Confirm;
use scraper::{element_ref::Text, ElementRef, Html, Node, Selector};
use std::fs::OpenOptions;
use std::io::Write;

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

    let html = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = Html::parse_document(&html);

    let mut contents: Vec<Vec<String>> = Vec::new();

    for s in selectors {
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

    let mut all_content: Vec<Vec<(&str, &str)>> = Vec::new();

    for content_index in 0..contents.first().expect("NO CONTENT").len() {
        let mut chunk: Vec<(&str, &str)> = Vec::new();
        for (i, content) in contents.iter().enumerate() {
            let header = keys[i].as_str();
            let value = content[content_index].trim();
            chunk.push((header, value));
        }

        all_content.push(chunk);
    }

    println!("CONTENT WITH KEYS: {:?}", all_content);

    println!();

    if let Some(title) = title {
        println!("{}", title.bold());
        println!();
    }

    match present {
        Some(Presentation::List) => {
            for chunk in all_content {
                for data in chunk {
                    let header = data.0;
                    let value = data.1;
                    println!("{}: {}", header.bold(), value);
                }
                println!();
            }
        }
        Some(Presentation::Table) => {
            //TODO: Print table
            let table = vec![
                vec!["Tom".cell(), 10.cell().justify(Justify::Right)],
                vec!["Jerry".cell(), 15.cell().justify(Justify::Right)],
                vec!["Scooby Doo".cell(), 20.cell().justify(Justify::Right)],
            ]
            .table()
            .title(vec![
                "Name".cell().bold(true),
                "Age (in years)".cell().bold(true),
            ])
            .bold(true);

            let table_display = table.display().unwrap();

            println!("{}", table_display);
        }
        None => {
            //Printing list as default. TODO: Maybe prompt and ask for list or table in this case instead.
            for chunk in all_content {
                for data in chunk {
                    let header = data.0;
                    let value = data.1;
                    println!("{}: {}", header.bold(), value);
                }
                println!();
            }
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
                Ok(true) => println!("Scrape is saved!"),
                Ok(false) => println!("Skipped saving"),
                Err(_) => println!("Error with questionnaire, try again later"),
            }
        }
    }
}

fn save_scrape(name: &str, url: String, selectors: Vec<String>, keys: Vec<String>, title: String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("scrapers.txt")
        .unwrap();

    //TODO: Maybe store scrape as JSON. Could possibly be easier to combine scrapers later, and read scrapers from the file etc. See scrapers.json file

    writeln!(
        file,
        "{} {} {:?} {:?} {}",
        name, url, selectors, keys, title
    )
    .unwrap();
}
