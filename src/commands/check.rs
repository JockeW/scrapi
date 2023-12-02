use crate::utils::get_saved_scrape;

pub fn check(name: String) {
    let scrape_data = get_saved_scrape(&name);

    match scrape_data {
        Some(data) => print_scrape_info(data),
        None => println!("Scrape {} not found!", &name),
    }
}

fn print_scrape_info(data_str: &str) {
    let data: Vec<&str> = data_str.split(";").collect();

    if data[0] == "combined" {
        let scrape_names: Vec<String> = data[2]
            .replace("[", "")
            .replace("]", "")
            .replace(",", "")
            .replace("\"", "")
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        for name in scrape_names {
            let scrape = get_saved_scrape(&name);
            if let Some(scrape) = scrape {
                print(scrape.split(";").collect());
                println!();
            }
        }
    } else {
        print(data);
    }

    fn print(data: Vec<&str>) {
        println!("Name: {}", data[0]);
        println!("Url: {}", data[1]);
        println!("Selectors: {}", data[2]);
        println!("Keys: {}", data[3]);
        println!("Attributes: {}", data[4]);
        println!("Title: {}", data[5]);
        println!("Present: {}", data[6]);

        let selectors: String = data[2].replace("[", "").replace("]", "").replace(",", "");
        let keys: String = data[3].replace("[", "").replace("]", "").replace(",", "");

        let attributes_string = data[4].replace("[", "").replace("]", "").replace(",", "");
        let attributes = if attributes_string.len() > 0 {
            format!(" --attributes {}", attributes_string)
        } else {
            "".to_string()
        };

        let title: String = if data[5].len() > 0 {
            format!(" --title \"{}\"", data[5])
        } else {
            "".to_string()
        };

        let presentation: String = if data[6].len() > 0 {
            format!(" --present \"{}\"", data[6])
        } else {
            "".to_string()
        };

        println!(
            "Full command: {}",
            format!(
                "scrape --url \"{}\" --selectors {} --keys {}{}{}{}",
                data[1], selectors, keys, attributes, title, presentation
            )
        );
    }
}
