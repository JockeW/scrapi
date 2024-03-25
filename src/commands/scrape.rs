use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

use cli_table::{Cell, Style, Table};
use colored::Colorize;
use inquire::Confirm;
use scraper::{ElementRef, Html, Node, Selector};
use serde_json::{json, to_writer};

use crate::enums::Presentation;

pub fn scrape(
    url: String,
    selectors: Vec<String>,
    keys: Vec<String>,
    attributes: Option<Vec<String>>,
    prefixes: Option<Vec<String>>,
    suffixes: Option<Vec<String>>,
    title: Option<String>,
    save: Option<String>,
    present: Option<Presentation>,
    export: Option<String>,
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

            if parsed_attributes.iter().any(|a| a.0 == selector_index) {
                println!("You can only have one attribute per seletor");
                return;
            }

            let attribute = *attr_parts.last().unwrap();

            parsed_attributes.push((selector_index, attribute));
        }
    }

    let mut parsed_prefixes: Vec<(usize, &str)> = Vec::new();

    if let Some(prefixes) = prefixes.as_ref() {
        for prefix in prefixes {
            let prefix_parts = prefix.split_once(":").expect("Invalid prefix");

            let selector_index: usize = prefix_parts
                .0
                .parse()
                .expect("Prefixes argument needs correct format");

            if parsed_prefixes.iter().any(|a| a.0 == selector_index) {
                println!("You can only have one prefix per seletor");
                return;
            }

            let prefix_value = prefix_parts.1;

            parsed_prefixes.push((selector_index, prefix_value));
        }
    }

    let mut parsed_suffixes: Vec<(usize, &str)> = Vec::new();

    if let Some(suffixes) = suffixes.as_ref() {
        for suffix in suffixes {
            let suffix_parts = suffix.split_once(":").expect("Invalid suffix");

            let selector_index: usize = suffix_parts
                .0
                .parse()
                .expect("Suffixes argument needs correct format");

            if parsed_suffixes.iter().any(|a| a.0 == selector_index) {
                println!("You can only have one suffix per seletor");
                return;
            }

            let suffix_value = suffix_parts.1;

            parsed_suffixes.push((selector_index, suffix_value));
        }
    }

    let html = reqwest::blocking::get(&url).unwrap().text().unwrap();
    let document = Html::parse_document(&html);
    //println!("{}", document.html());//TODO: Some message if response html is only a captcha. Or does it work with WebDriver? For example Ticketmaster site.

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
                        let attribute_value = element
                            .value()
                            .attr(attributes.first().unwrap().1)
                            .expect("Attribute not found");

                        full_text = attribute_value.to_string();
                    }
                }

                if full_text.is_empty() {
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
                }

                //Add prefix and suffix
                if parsed_prefixes.len() > 0 {
                    let prefixes = parsed_prefixes
                        .iter()
                        .filter(|&a| a.0 == index)
                        .collect::<Vec<&(usize, &str)>>();

                    if prefixes.len() > 0 {
                        full_text = format!("{}{}", prefixes.first().unwrap().1, full_text);
                    }
                }
                if parsed_suffixes.len() > 0 {
                    let suffixes = parsed_suffixes
                        .iter()
                        .filter(|&a| a.0 == index)
                        .collect::<Vec<&(usize, &str)>>();

                    if suffixes.len() > 0 {
                        full_text = format!("{}{}", full_text, suffixes.first().unwrap().1);
                    }
                }

                content_vec.push(full_text);
            }

            contents.push(content_vec);
        } else {
            println!("No elements found for selector: {}", s);
            return;
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
            print_content_list(&all_content, keys.iter().map(|k| k.as_str()).collect());
        }
        Some(Presentation::Table) => {
            print_content_table(&all_content, keys.iter().map(|k| k.as_str()).collect());
        }
        None => (),
    }

    if let Some(export) = &export {
        let file_type = export.split('.').last().unwrap();

        match file_type {
            "json" => {
                let file = File::create(export).unwrap();

                let title_to_write = match &title {
                    Some(t) => t,
                    None => "",
                };

                let mut results: Vec<HashMap<&String, &str>> = Vec::new();

                for content in all_content {
                    let mut hash_map = HashMap::new();
                    for (i, data_str) in content.iter().enumerate() {
                        let key = &keys[i];
                        let value = *data_str;
                        hash_map.insert(key, value);
                    }

                    results.push(hash_map);
                }

                let data = json!({
                    "title": title_to_write,
                    "results": results
                });

                to_writer(&file, &data).unwrap();

                println!("JSON file created successfully.");
            }
            "csv" => (),
            _ => {
                println!("The supported file types are '.json' and '.csv'");
                return;
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
                Ok(true) => save_scrape(
                    &save, &url, selectors, keys, attributes, prefixes, suffixes, title, present,
                    export,
                ),
                Ok(false) => println!("Skipped saving"),
                Err(_) => println!("Error with questionnaire, try again later"),
            }
        }
    }
}

fn print_content_list(content: &Vec<Vec<&str>>, keys: Vec<&str>) {
    for chunk in content {
        for (i, data) in chunk.iter().enumerate() {
            let header = keys[i];
            let value = data;
            println!("{}: {}", header.bold(), value);
        }
        println!();
    }
}

fn print_content_table(content: &Vec<Vec<&str>>, keys: Vec<&str>) {
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
    prefixes: Option<Vec<String>>,
    suffixes: Option<Vec<String>>,
    title: Option<String>,
    presentation: Option<Presentation>,
    export: Option<String>,
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

        let prefixes_to_write = match prefixes {
            Some(t) => t,
            None => Vec::new(),
        };

        let suffixes_to_write = match suffixes {
            Some(t) => t,
            None => Vec::new(),
        };

        let presentation_to_write = match presentation {
            Some(p) => p.to_string().to_lowercase(),
            None => "".to_string(),
        };

        let export_to_write = match export {
            Some(t) => t,
            None => "".to_string(),
        };

        writeln!(
            file,
            "{};{};{:?};{:?};{:?};{:?};{:?};{};{};{}",
            name,
            url,
            selectors,
            keys,
            attributes_to_write,
            prefixes_to_write,
            suffixes_to_write,
            title_to_write,
            presentation_to_write,
            export_to_write
        )
        .unwrap();
    }
}
