mod args;

use args::{Presentation, RScrapeArgs};
use clap::Parser;
use cli_table::{Cell, Style, Table};
use colored::Colorize;
use inquire::Confirm;
use scraper::{ElementRef, Html, Node, Selector};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn main() {
    let args = RScrapeArgs::parse();

    match args.sub_command {
        args::RScrapeCommand::Scrape(cmd) => scrape(
            cmd.url,
            cmd.selectors,
            cmd.keys,
            cmd.attributes,
            cmd.title,
            cmd.save,
            cmd.present,
        ),
        args::RScrapeCommand::Check(cmd) => check(cmd.name),
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

fn get_saved_scrape(name: &str) -> Option<&str> {
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

    scrape(
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

fn check(name: String) {
    let scrape_data = get_saved_scrape(&name);

    match scrape_data {
        Some(data) => print_scrape_info(data),
        None => println!("Scrape {} not found!", &name),
    }
}

fn print_scrape_info(data_str: &str) {
    let data: Vec<&str> = data_str.split(";").collect();

    if data[0] == "combined" {
        let scrape_names: Vec<String> = data[2]
            .replace("[", "")
            .replace("]", "")
            .replace(",", "")
            .replace("\"", "")
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        for name in scrape_names {
            let scrape = get_saved_scrape(&name);
            if let Some(scrape) = scrape {
                print(scrape.split(";").collect());
                println!();
            }
        }
    } else {
        print(data);
    }

    fn print(data: Vec<&str>) {
        println!("Name: {}", data[0]);
        println!("Url: {}", data[1]);
        println!("Selectors: {}", data[2]);
        println!("Keys: {}", data[3]);
        println!("Attributes: {}", data[4]);
        println!("Title: {}", data[5]);
        println!("Present: {}", data[6]);

        let selectors: String = data[2].replace("[", "").replace("]", "").replace(",", "");
        let keys: String = data[3].replace("[", "").replace("]", "").replace(",", "");

        let attributes_string = data[4].replace("[", "").replace("]", "").replace(",", "");
        let attributes = if attributes_string.len() > 0 {
            format!(" --attributes {}", attributes_string)
        } else {
            "".to_string()
        };

        let title: String = if data[5].len() > 0 {
            format!(" --title \"{}\"", data[5])
        } else {
            "".to_string()
        };

        let presentation: String = if data[6].len() > 0 {
            format!(" --present \"{}\"", data[6])
        } else {
            "".to_string()
        };

        println!(
            "Full command: {}",
            format!(
                "scrape --url \"{}\" --selectors {} --keys {}{}{}{}",
                data[1], selectors, keys, attributes, title, presentation
            )
        );
    }
}

fn scrape(
    url: String,
    selectors: Vec<String>,
    keys: Vec<String>,
    attributes: Option<Vec<String>>,
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

    let mut parsed_attributes: Vec<(usize, &str)> = Vec::new();

    if let Some(attributes) = attributes.as_ref() {
        for attr in attributes {
            let attr_parts: Vec<&str> = attr.split(":").collect();
            if attr_parts.len() != 2 {
                println!("{}: Invalid attribute", "error".bold().color("red"));
                return;
            }

            let selector_index: usize = attr_parts
                .first()
                .unwrap()
                .parse()
                .expect("Attribute argument needs correct format");

            let attribute = *attr_parts.last().unwrap();

            parsed_attributes.push((selector_index, attribute));
        }
    }

    let html = reqwest::blocking::get(&url).unwrap().text().unwrap();
    let document = Html::parse_document(&html);
    //println!("{}", document.html());//TODO: Some message if response html is only a captcha

    let mut contents: Vec<Vec<String>> = Vec::new();

    for (index, s) in selectors.iter().enumerate() {
        let selector = Selector::parse(&s).expect("Not a valid selector");
        let element_ref: Vec<ElementRef> = document.select(&selector).collect();

        if element_ref.len() > 0 {
            let mut content_vec: Vec<String> = Vec::new();

            for element in element_ref {
                let mut full_text = String::from("");

                // If there is an attribute specified for current selector, get the value of that attribute
                // instead of checking element.children

                if parsed_attributes.len() > 0 {
                    let attributes = parsed_attributes
                        .iter()
                        .filter(|&a| a.0 == index)
                        .collect::<Vec<&(usize, &str)>>();

                    if attributes.len() > 0 {
                        if attributes.len() > 1 {
                            println!("Only one attribute per selector.");
                            return;
                        }

                        for attribute in attributes {
                            let attribute_value = element
                                .value()
                                .attr(attribute.1)
                                .expect("Attribute not found");

                            full_text = attribute_value.to_string();
                            content_vec.push(full_text);
                        }

                        continue;
                    }
                }

                for node in element.children() {
                    match node.value() {
                        Node::Text(text) => {
                            full_text = format!("{} {}", full_text, text.trim());
                        }
                        Node::Element(_el) => {
                            let element_ref = ElementRef::wrap(node).unwrap();
                            let element_text = element_ref.text().collect::<Vec<&str>>();

                            let mut text_to_append = String::new();
                            for text in element_text {
                                text_to_append = format!("{}{}", text_to_append, text);
                            }

                            full_text = format!("{}{}", full_text, text_to_append);
                        }
                        _ => (),
                    }
                }
                content_vec.push(full_text);
            }

            contents.push(content_vec);
        } else {
            println!("No elements found for selector: {}", s);
        }
    }

    if contents.len() == 0 {
        return;
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
        None => (),
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
                Ok(true) => save_scrape(&save, &url, selectors, keys, attributes, title, present),
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
    attributes: Option<Vec<String>>,
    title: Option<String>,
    presentation: Option<Presentation>,
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
        //TODO: Prompt user with options for overwrite, entering a new name, or cancel
    } else if name.to_lowercase() == "combined" {
        println!("'combined' is a reserved word");
    } else {
        let title_to_write = match title {
            Some(t) => t,
            None => "".to_string(),
        };

        let attributes_to_write = match attributes {
            Some(t) => t,
            None => Vec::new(),
        };

        let presentation_to_write = match presentation {
            Some(p) => p.to_string().to_lowercase(),
            None => "".to_string(),
        };

        writeln!(
            file,
            "{};{};{:?};{:?};{:?};{};{}",
            name, url, selectors, keys, attributes_to_write, title_to_write, presentation_to_write
        )
        .unwrap();
    }
}
