use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Lines, Write},
};

use inquire::Confirm;

use crate::utils::{get_combined_scrapes_for_scrape, get_scrape_name};

pub fn delete(name: String) {
    let file: File = OpenOptions::new().read(true).open("scrapes.txt").unwrap();

    let reader = BufReader::new(&file);

    let mut scrape_found = false;
    let mut lines_not_to_delete = Vec::new();
    let mut combined_scrapes_to_update: Vec<&str> = Vec::new();

    let result_lines: Lines<BufReader<&File>> = reader.lines();
    let lines = result_lines.map(|x| x.unwrap()).collect::<Vec<String>>();
    for line in lines.iter() {
        let line_parts = line.split(';').collect::<Vec<&str>>();

        let is_combined = line_parts[0].trim() == "combined";

        let scrape_name = get_scrape_name(line);

        if scrape_name == &name {
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

    if combined_scrapes_to_update.len() > 0 {
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
            .find(|&&x| get_scrape_name(x) == scrape_name)
            .is_some();

        if exists {
            continue;
        }
        lines_to_write.push(line);
    }

    File::create("scrapes.txt.temp").unwrap();

    let mut out_file: File = OpenOptions::new()
        .write(true)
        .open("scrapes.txt.temp")
        .unwrap();

    for line in lines_to_write {
        writeln!(out_file, "{}", line).unwrap();
    }

    drop(file);
    drop(out_file);

    fs::rename("scrapes.txt.temp", "scrapes.txt").unwrap();

    println!("Scrape '{}' has been deleted", name);
}

fn handle_combined(
    combined_scrapes_using_deleted_scrape: Vec<&str>,
    name: &str,
) -> Option<Vec<String>> {
    let mut updated_combined_scrapes: Vec<String> = Vec::new();

    let mut message = format!("The following combined scrapes are using this scrape: ");

    for combined in combined_scrapes_using_deleted_scrape {
        let combined_scrape_parts = combined.split(';').collect::<Vec<&str>>();

        message = format!("{}\n{}", message, combined_scrape_parts[1]);

        let combined_scrapes = combined_scrape_parts[2]
            .replace("[", "")
            .replace("]", "")
            .replace("\"", "")
            .split(',')
            .map(|s| s.trim().replace(",", ""))
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
            return None;
        }
    }
}
