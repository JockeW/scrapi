use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Error, Write},
};

use cli_table::{Cell, Style, Table};
use colored::Colorize;
use csv::Writer;
use inquire::Confirm;
use scraper::{ElementRef, Html, Node, Selector};
use serde_json::{json, to_writer};

use crate::{enums::Presentation, utils::get_all_scrape_names};

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

    let mut html: String = String::new();
    let html_result = reqwest::blocking::get(&url);
    match html_result {
        Ok(html_response) => {
            let html_text_result = html_response.text();
            match html_text_result {
                Ok(text) => html = text,
                Err(err) => {
                    println!("Failed getting data. Error: {}", err);
                    return;
                }
            }
        }
        Err(err) => {
            println!("Failed getting data. Error: {}", err);
            return;
        }
    }

    let document = Html::parse_document(&html);
    //println!("{}", document.html()); //TODO: Some message if response html is only a captcha.

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
        let file_type: &str;
        let file_type_opt = export.split('.').last();
        if let Some(type_of_file) = file_type_opt {
            file_type = type_of_file;
        } else {
            println!("File type missing for export file");
            return;
        }

        let title_to_write = match &title {
            Some(t) => t,
            None => "",
        };

        match file_type {
            "json" => {
                let file: File;
                let file_result = File::create(export);
                match file_result {
                    Ok(f) => file = f,
                    Err(err) => {
                        println!("Failed to create export file. Error: {}", err);
                        return;
                    }
                }

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

                let json_result = to_writer(&file, &data);
                match json_result {
                    Ok(()) => println!("JSON file created successfully."),
                    Err(err) => {
                        println!("Failed to create export file. Error: {}", err);
                        return;
                    }
                }
            }
            "csv" => {
                let file: File;
                let file_result = File::create(export);
                match file_result {
                    Ok(f) => file = f,
                    Err(err) => {
                        println!("Failed to create export file. Error: {}", err);
                        return;
                    }
                }

                let mut writer = Writer::from_writer(file);

                let mut write_results = Vec::new();
                let mut write_record_result = writer.write_record(&keys);
                write_results.push(write_record_result);

                for record in all_content {
                    write_record_result = writer.write_record(record);
                    write_results.push(write_record_result);
                }

                if write_results.iter().any(|result| result.is_err()) {
                    println!("Failed to create export file");
                    return;
                }

                let flush_result = writer.flush();
                if flush_result.is_err() {
                    println!("Failed to create export file");
                    return;
                }

                println!("CSV file created successfully.");
            }
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
                Ok(true) => {
                    let save_result = save_scrape(
                        &save, &url, selectors, keys, attributes, prefixes, suffixes, title,
                        present, export,
                    );

                    match save_result {
                        Ok(()) => println!("Scraoe saved successfully!"),
                        Err(err) => println!("Error: {}", err),
                    }
                }
                Ok(false) => println!("Skipped saving"),
                Err(err) => println!("Error with questionnaire. Error: {}", err),
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

    let table_display_result = table.display();
    match table_display_result {
        Ok(table_display) => println!("{}", table_display),
        Err(err) => println!("Failed printing data in table. Error: {}", err),
    }
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
) -> Result<(), Error> {
    println!("Saving scrape...");

    let mut file: File;
    let file_result = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open("scrapes.txt");

    match file_result {
        Ok(file_ok) => file = file_ok,
        Err(err) => {
            println!("Something went wrong when opening file.");
            return Result::Err(err);
        }
    }

    let scrape_names: Vec<String> = get_all_scrape_names();

    if scrape_names.contains(&name.to_lowercase()) {
        println!("There is already a scrape with that name: '{}'", name);
        return Result::Err(Error::new(std::io::ErrorKind::Other, "Name already taken"));
        //TODO: Prompt user with options for overwrite, entering a new name, or cancel
    } else if name.to_lowercase() == "combined" {
        println!("'combined' is a reserved word");
        return Result::Err(Error::new(
            std::io::ErrorKind::Other,
            "'combined' is a reserved word",
        ));
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

        let write_result = writeln!(
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
        );

        write_result
    }
}
