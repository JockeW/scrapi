use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

pub fn delete(name: String) {
    let file: File = OpenOptions::new().read(true).open("scrapes.txt").unwrap();

    let reader = BufReader::new(&file);

    let mut scrape_found = false;
    let mut lines_to_write = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(&name) {
            //TODO: Fix this to check only against the name instead of whole line
            scrape_found = true;
        } else {
            lines_to_write.push(line);
        }
    }

    if !scrape_found {
        println!("Scrape '{}' was not found", name);
        return;
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
