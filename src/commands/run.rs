use crate::{commands::scrape, utils::get_saved_scrape};

pub fn run(name: String) {
    let scrape_data = get_saved_scrape(&name);

    match scrape_data {
        Some(scrapes) => {
            for scrape in scrapes {
                scrape::scrape(
                    scrape.url,
                    scrape.selectors,
                    scrape.keys,
                    scrape.attributes,
                    scrape.prefixes,
                    scrape.suffixes,
                    scrape.title,
                    None,
                    scrape.presentation,
                );
            }
        }
        None => println!("Scrape {} not found!", &name),
    }
}
