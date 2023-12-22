use crate::{enums::Presentation, structs::Scrape};

pub fn get_saved_scrape(name: &str) -> Option<Vec<Scrape>> {
    let file_content = include_str!("../scrapes.txt");

    let lines: Vec<&str> = file_content.trim().split('\n').collect();
    let saved_scrape = lines
        .iter()
        .find(|&&s| {
            let line_parts = s.split(';').collect::<Vec<&str>>();
            if line_parts[0] == "combined" {
                line_parts[1] == name
            } else {
                line_parts[0] == name
            }
        })
        .map(|&s| s);

    if let Some(scrape) = saved_scrape {
        let mut scrapes: Vec<Scrape> = Vec::new();

        if scrape.split(';').collect::<Vec<&str>>()[0] == "combined" {
            let scrapes_in_combined = scrape.split(';').collect::<Vec<&str>>()[2]
                .replace("[", "")
                .replace("]", "")
                .replace("\"", "")
                .split(',')
                .map(|s| s.trim().replace(",", "").to_string())
                .collect::<Vec<String>>();

            for sc in scrapes_in_combined {
                let saved_scrape = lines
                    .iter()
                    .find(|&&s| s.split(';').collect::<Vec<&str>>()[0] == sc)
                    .map(|&s| s);

                if let Some(s) = saved_scrape {
                    scrapes.push(get_scrape_from_str(s));
                }
            }
        } else {
            scrapes.push(get_scrape_from_str(scrape));
        }

        return Some(scrapes);
    } else {
        return None;
    }
}

fn get_scrape_from_str(data_str: &str) -> Scrape {
    let data: Vec<String> = data_str.split(";").map(|s| s.to_owned()).collect();
    let name = &data[0];
    let url = &data[1];
    let selectors = data[2][1..data[2].len() - 1]
        .split(", ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.trim().replace("\"", ""))
        .collect::<Vec<String>>();

    let keys = data[3][1..data[3].len() - 1]
        .split(", ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.trim().replace("\"", ""))
        .collect::<Vec<String>>();

    let attributes: Option<Vec<String>> = if data[4].len() > 2 {
        Some(
            data[4][1..data[4].len() - 1]
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|&s| s.trim().replace("\"", ""))
                .collect::<Vec<String>>(),
        )
    } else {
        None
    };

    let prefixes: Option<Vec<String>> = if data[5].len() > 2 {
        Some(
            data[5][1..data[5].len() - 1]
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|&s| s.trim().replace("\"", ""))
                .collect::<Vec<String>>(),
        )
    } else {
        None
    };

    let suffixes: Option<Vec<String>> = if data[6].len() > 2 {
        Some(
            data[6][1..data[6].len() - 1]
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|&s| s.trim().replace("\"", ""))
                .collect::<Vec<String>>(),
        )
    } else {
        None
    };

    let title: Option<String> = if data[7].len() > 0 {
        Some(data[7].clone())
    } else {
        None
    };

    let presentation = if data[8].len() > 0 {
        if data[8].trim() == "table" {
            Some(Presentation::Table)
        } else {
            Some(Presentation::List)
        }
    } else {
        None
    };

    // let export: Option<String> = if data[9].len() > 0 {
    //     Some(data[9].clone())
    // } else {
    //     None
    // };

    Scrape {
        name: name.to_string(),
        url: url.to_string(),
        selectors,
        keys,
        attributes,
        prefixes,
        suffixes,
        title,
        presentation,
        export: None, //TODO: Update when using export
    }
}
