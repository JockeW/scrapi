use crate::{structs::Scrape, utils::get_saved_scrape};

pub fn check(name: String) {
    let scrape_data = get_saved_scrape(&name);

    match scrape_data {
        Some(scrapes) => print_scrape_info(scrapes),
        None => println!("Scrape {} not found!", &name),
    }
}

fn print_scrape_info(scrapes: Vec<Scrape>) {
    for scrape in scrapes {
        let attributes = if let Some(attributes) = scrape.attributes {
            attributes
        } else {
            Vec::new()
        };
        let prefixes = if let Some(prefixes) = scrape.prefixes {
            prefixes
        } else {
            Vec::new()
        };
        let suffixes = if let Some(suffixes) = scrape.suffixes {
            suffixes
        } else {
            Vec::new()
        };
        let title = if let Some(title) = scrape.title {
            title
        } else {
            "".to_string()
        };
        let presentation = if let Some(presentation) = scrape.presentation {
            presentation.to_string()
        } else {
            "".to_string()
        };
        let export = if let Some(export) = scrape.export {
            export
        } else {
            "".to_string()
        };
        println!("Name: {}", scrape.name);
        println!("Url: {}", scrape.url);
        println!("Selectors: {:?}", scrape.selectors);
        println!("Keys: {:?}", scrape.keys);
        println!("Attributes: {:?}", attributes);
        println!("Prefixes: {:?}", prefixes);
        println!("Suffixes: {:?}", suffixes);
        println!("Title: {:?}", title);
        println!("Present: {:?}", presentation);
        println!("Export: {:?}", export);

        let selectors_full_command: String = scrape
            .selectors
            .iter()
            .map(|s| format!("\"{}\"", s))
            .reduce(|acc, e| acc + " " + &e)
            .unwrap();

        let keys_full_command: String = scrape
            .keys
            .iter()
            .map(|s| format!("\"{}\"", s))
            .reduce(|acc, e| acc + " " + &e)
            .unwrap();

        let attributes_full_command = if !attributes.is_empty() {
            let attributes_string = attributes
                .iter()
                .map(|s| format!("\"{}\"", s))
                .reduce(|acc, e| acc + " " + &e)
                .unwrap();

            format!(" --attributes {}", attributes_string)
        } else {
            "".to_string()
        };

        let prefixes_full_command = if !prefixes.is_empty() {
            let prefixes_string = prefixes
                .iter()
                .map(|s| format!("\"{}\"", s))
                .reduce(|acc, e| acc + " " + &e)
                .unwrap();
            format!(" --prefixes {}", prefixes_string)
        } else {
            "".to_string()
        };

        let suffixes_full_command = if !suffixes.is_empty() {
            let suffixes_string = suffixes
                .iter()
                .map(|s| format!("\"{}\"", s))
                .reduce(|acc, e| acc + " " + &e)
                .unwrap();

            format!(" --suffixes {}", suffixes_string)
        } else {
            "".to_string()
        };

        let title_full_command: String = if !title.is_empty() {
            format!(" --title \"{}\"", title)
        } else {
            "".to_string()
        };

        let presentation_full_command: String = if !presentation.is_empty() {
            format!(" --present \"{}\"", presentation)
        } else {
            "".to_string()
        };

        let export_full_command: String = if !export.is_empty() {
            format!(" --export \"{}\"", export)
        } else {
            "".to_string()
        };

        println!(
            "Full command: scrape --url \"{}\" --selectors {} --keys {}{}{}{}{}{}{}",
            scrape.url,
            selectors_full_command,
            keys_full_command,
            attributes_full_command,
            prefixes_full_command,
            suffixes_full_command,
            title_full_command,
            presentation_full_command,
            export_full_command
        );
        println!();
    }
}
