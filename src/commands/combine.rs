use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub fn combine(name: String, scrapes: Vec<String>) {
    let mut file: File;
    let file_result = OpenOptions::new()
        .append(true)
        .read(true)
        .open(".data/scrapes.txt");

    match file_result {
        Ok(file_ok) => file = file_ok,
        Err(_err) => {
            println!("There are no saved scrapes");
            return;
        }
    }

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
        .filter(|&&s| s.split(';').collect::<Vec<&str>>()[0] != "combined")
        .map(|s| s.split(';').collect::<Vec<&str>>()[0])
        .collect();

    let combined_scrape_names: Vec<&str> = saved_scrapes
        .iter()
        .filter(|&&s| s.split(';').collect::<Vec<&str>>()[0] == "combined")
        .map(|s| s.split(';').collect::<Vec<&str>>()[1])
        .collect();

    if scrape_names.contains(&name.to_lowercase().as_str())
        || combined_scrape_names.contains(&name.to_lowercase().as_str())
    {
        println!("There is already a scrape with that name: '{}'", name);
        //TODO: Prompt user with options for entering a new name or cancel
    } else if name.to_lowercase() == "combined" {
        println!("'combined' is a reserved word");
    } else {
        for scrape in &scrapes {
            if !scrape_names.contains(&scrape.as_str()) {
                println!("There is no saved scrape: '{}'", scrape);
                return;
            } else if combined_scrape_names.contains(&scrape.as_str()) {
                println!(
                    "You can't combine with other combined scrapes: '{}'",
                    scrape
                );
                return;
            }
        }

        let write_result = writeln!(file, "combined;{};{:?}", name, scrapes);
        match write_result {
            Ok(_) => println!("Combine successful"),
            Err(err) => println!("Something went wrong when writing to file. Error: {}", err),
        }
    }
}
