use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Lines, Write},
};

use inquire::Confirm;

use crate::utils::{get_combined_scrapes_for_scrape, get_scrape_name};

pub fn delete(name: String) {
    let file: File;
    let file_result = OpenOptions::new().read(true).open(".data/scrapes.txt");

    match file_result {
        Ok(file_ok) => file = file_ok,
        Err(_err) => {
            println!("There are no saved scrapes. The scrapes file does not exist.");
            return;
        }
    }

    let reader = BufReader::new(&file);

    let mut scrape_found = false;
    let mut lines_not_to_delete = Vec::new();
    let mut combined_scrapes_to_update: Vec<String> = Vec::new();

    let result_lines: Lines<BufReader<&File>> = reader.lines();
    let mut lines: Vec<String> = Vec::new();
    for line_result in result_lines {
        match line_result {
            Ok(line) => lines.push(line),
            Err(err) => {
                println!("Something went wrong. Error: {}", err);
                return;
            }
        }
    }

    for line in lines.iter() {
        let line_parts = line.split(';').collect::<Vec<&str>>();

        let is_combined = line_parts[0].trim() == "combined";

        let scrape_name = get_scrape_name(line);

        if scrape_name == name {
            scrape_found = true;

            if !is_combined {
                combined_scrapes_to_update = get_combined_scrapes_for_scrape(&name);
            }
        } else {
            lines_not_to_delete.push(line);
        }
    }

    if !scrape_found {
        println!("Scrape '{}' was not found", name);
        return;
    }

    let mut lines_to_write: Vec<&String> = Vec::new();
    let mut updated_combined_scrapes: Vec<String> = Vec::new();

    if !combined_scrapes_to_update.is_empty() {
        let update_combined_option = handle_combined(combined_scrapes_to_update, &name);

        match update_combined_option {
            Some(mut combined_scrapes) => {
                updated_combined_scrapes.append(&mut combined_scrapes);

                for combined_scrape in updated_combined_scrapes.iter() {
                    lines_to_write.push(combined_scrape);
                }
            }
            None => return,
        }
    }

    for line in lines_not_to_delete {
        let scrape_name = get_scrape_name(line);
        let exists = lines_to_write
            .iter()
            .any(|&x| get_scrape_name(x) == scrape_name);

        if exists {
            continue;
        }
        lines_to_write.push(line);
    }

    let file_result = File::create(".data/scrapes.txt.temp");
    if let Err(err) = file_result {
        println!("Something went wrong. Error: {}", err);
        return;
    }

    let mut out_file: File;
    let out_file_result = OpenOptions::new()
        .write(true)
        .open(".data/scrapes.txt.temp");

    match out_file_result {
        Ok(file) => out_file = file,
        Err(err) => {
            println!("Something went wrong. Error: {}", err);
            return;
        }
    }

    for line in lines_to_write {
        let write_result = writeln!(out_file, "{}", line);
        if let Err(err) = write_result {
            println!("Something went wrong. Error: {}", err);
            return;
        }
    }

    drop(file);
    drop(out_file);

    let rename_result = fs::rename(".data/scrapes.txt.temp", ".data/scrapes.txt");
    if let Err(err) = rename_result {
        println!("Something went wrong. Error: {}", err);
        return;
    }

    println!("Scrape '{}' has been deleted", name);
}

fn handle_combined(
    combined_scrapes_using_deleted_scrape: Vec<String>,
    name: &str,
) -> Option<Vec<String>> {
    let mut updated_combined_scrapes: Vec<String> = Vec::new();

    let mut message = "The following combined scrapes are using this scrape: ".to_string();

    for combined in combined_scrapes_using_deleted_scrape {
        let combined_scrape_parts = combined.split(';').collect::<Vec<&str>>();

        message = format!("{}\n{}", message, combined_scrape_parts[1]);

        let combined_scrapes = combined_scrape_parts[2]
            .replace(['[', ']', '\"'], "")
            .split(',')
            .map(|s| s.trim().replace(',', ""))
            .collect::<Vec<String>>();

        let mut new_combined_scrapes = Vec::new();
        for scrape in combined_scrapes {
            if scrape == name {
                continue;
            }

            new_combined_scrapes.push(scrape.clone());
        }

        let new_combined_to_write = format!(
            "combined;{};{:?}",
            combined_scrape_parts[1], new_combined_scrapes
        );

        updated_combined_scrapes.push(new_combined_to_write);
    }

    message = format!("{}\nAre you sure you want to delete the scrape?", message);

    let answer = Confirm::new(&message)
        .with_default(false)
        .with_help_message(
            "All arguments will be saved so the scrape can be reused with 'run' command.",
        )
        .prompt();

    match answer {
        Ok(true) => Some(updated_combined_scrapes),
        Ok(false) => {
            println!("The scrape was NOT deleted");
            None
        }
        Err(_) => {
            println!("Error with questionnaire, try again later");
            None
        }
    }
}
