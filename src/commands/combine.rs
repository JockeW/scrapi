use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

pub fn combine(name: String, scrapes: Vec<String>) {
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