use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Lines, Write},
};

use inquire::Confirm;

pub fn delete(name: String) {
    //TODO: If not a combined scrape, check if any combined is using the scrape that will be removed. Prompt user for confirmation before continuing removal
    let file: File = OpenOptions::new().read(true).open("scrapes.txt").unwrap();

    let reader = BufReader::new(&file);

    let mut scrape_found = false;
    let mut is_combined = false;
    let mut lines_to_write = Vec::new();

    let result_lines: Lines<BufReader<&File>> = reader.lines();
    let lines = result_lines.map(|x| x.unwrap()).collect::<Vec<String>>();
    for line in lines.iter() {
        //let line = line.unwrap();
        let line_parts = line.split(';').collect::<Vec<&str>>();

        let scrape_name = if is_combined {
            line_parts[1]
        } else {
            line_parts[0]
        };

        if scrape_name == &name {
            scrape_found = true;
            is_combined = line_parts[0].trim() == "combined";
            if !is_combined {
                //TODO: Handle it here instead?
                // let updated_combined = handle_combined(&lines, name.clone());
            }
        } else {
            lines_to_write.push(line);
        }
    }

    if !scrape_found {
        println!("Scrape '{}' was not found", name);
        return;
    }

    if !is_combined {
        let combined_scrapes_using_deleted_scrape = lines
            .iter()
            .filter(|&l| {
                l.split(';').collect::<Vec<&str>>()[0] == "combined"
                    && l.split(';').collect::<Vec<&str>>()[2]
                        .replace("[", "")
                        .replace("]", "")
                        .replace("\"", "")
                        .split(',')
                        .map(|s| s.trim().replace(",", ""))
                        .collect::<Vec<String>>()
                        .contains(&name)
            })
            .map(|x| x.as_str())
            .collect::<Vec<&str>>();

        let combined_scrape_names = combined_scrapes_using_deleted_scrape
            .iter()
            .map(|x| x.split(';').collect::<Vec<&str>>()[1])
            .collect::<Vec<&str>>();

        if combined_scrapes_using_deleted_scrape.len() > 0 {
            lines_to_write = lines_to_write
                .iter()
                .filter(|&&x| {
                    x.split(';').collect::<Vec<&str>>()[0] == "combined"
                        && combined_scrape_names
                            .iter()
                            .any(|&c| c == x.split(';').collect::<Vec<&str>>()[1])
                            == false
                })
                .map(|x| *x)
                .collect::<Vec<&String>>();

            let combined_scrapes =
                handle_combined(combined_scrapes_using_deleted_scrape, name.clone());

            match combined_scrapes {
                Some(scrapes) => {
                    for scrape in scrapes {
                        let test = scrape.clone();
                        lines_to_write.push(&test);
                    }
                }
                None => return,
            }
        }
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
    name: String,
) -> Option<Vec<String>> {
    let mut updated_combined_scrapes: Vec<String> = Vec::new();

    // let combined_scrapes_using_deleted_scrape = lines
    //     .iter()
    //     .filter(|&l| {
    //         l.split(';').collect::<Vec<&str>>()[0] == "combined"
    //             && l.split(';').collect::<Vec<&str>>()[2]
    //                 .replace("[", "")
    //                 .replace("]", "")
    //                 .replace("\"", "")
    //                 .split(',')
    //                 .map(|s| s.trim().replace(",", ""))
    //                 .collect::<Vec<String>>()
    //                 .contains(&name)
    //     })
    //     .map(|x| x.as_str())
    //     .collect::<Vec<&str>>();

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
            return None;
        }
        Err(_) => {
            println!("Error with questionnaire, try again later");
            return None;
        }
    }
}
