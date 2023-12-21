use crate::{utils::get_saved_scrape, structs::Scrape};

pub fn check(name: String) {
    let scrape_data = get_saved_scrape(&name);

    match scrape_data {
        Some(scrapes) => print_scrape_info(scrapes),
        None => println!("Scrape {} not found!", &name),
    }
}

fn print_scrape_info(scrapes: Vec<Scrape>) {

    for scrape in scrapes {
        
    }

    fn print(scrape: Scrape) {
        println!("Name: {}", scrape.name);
        println!("Url: {}", scrape.url);
        println!("Selectors: {:?}", scrape.selectors);
        println!("Keys: {:?}", scrape.keys);
        println!("Attributes: {:?}", scrape.attributes);
        println!("Prefixes: {:?}", scrape.prefixes);
        println!("Suffixes: {:?}", scrape.suffixes);
        println!("Title: {:?}", scrape.title);
        println!("Present: {:?}", scrape.presentation);

        let selectors: String = data[2].replace("[", "").replace("]", "").replace(",", "");
        let keys: String = data[3].replace("[", "").replace("]", "").replace(",", "");

        let attributes_string = data[4].replace("[", "").replace("]", "").replace(",", "");
        let attributes = if attributes_string.len() > 0 {
            format!(" --attributes {}", attributes_string)
        } else {
            "".to_string()
        };

        let prefixes_string = data[5].replace("[", "").replace("]", "").replace(",", "");
        let prefixes = if prefixes_string.len() > 0 {
            format!(" --prefixes {}", prefixes_string)
        } else {
            "".to_string()
        };

        let suffixes_string = data[6].replace("[", "").replace("]", "").replace(",", "");
        let suffixes = if suffixes_string.len() > 0 {
            format!(" --suffixes {}", suffixes_string)
        } else {
            "".to_string()
        };

        let title: String = if data[7].len() > 0 {
            format!(" --title \"{}\"", data[7])
        } else {
            "".to_string()
        };

        let presentation: String = if data[8].len() > 0 {
            format!(" --present \"{}\"", data[8])
        } else {
            "".to_string()
        };

        println!(
            "Full command: {}",
            format!(
                "scrape --url \"{}\" --selectors {} --keys {}{}{}{}{}{}",
                scrape.url, selectors, keys, attributes, prefixes, suffixes, title, presentation
            )
        );
    }
}
