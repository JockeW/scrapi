use crate::{
    enums::Presentation,
    structs::Scrape,
};

pub fn get_scrape_name(scrape: &str) -> &str {
    let scrape_parts = scrape.split(';').collect::<Vec<&str>>();

    let scrape_name = if scrape_parts[0] == "combined" {
        scrape_parts[1]
    } else {
        scrape_parts[0]
    };

    scrape_name
}

pub fn get_combined_scrapes_for_scrape(scrape_name: &String) -> Vec<&'static str> {
    let file_content = include_str!("../scrapes.txt");

    let lines: Vec<&str> = file_content.trim().split('\n').collect();
    let saved_scrapes: Vec<&str> = lines.iter().map(|l| l.trim()).collect();

    return saved_scrapes
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
                    .contains(scrape_name)
        })
        .map(|&x| x)
        .collect::<Vec<&str>>();
}

pub fn get_all_scrape_names() -> Vec<String> {
    let file_content = include_str!("../scrapes.txt");

    let lines: Vec<&str> = file_content.trim().split('\n').collect();
    let saved_scrapes: Vec<&str> = lines.iter().map(|l| l.trim()).collect();

    let mut scrape_names: Vec<String> = Vec::new();

    for scrape in saved_scrapes {
        if scrape.split(';').collect::<Vec<&str>>()[0] == "combined" {
            scrape_names.push(
                scrape
                    .split(';')
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()[1]
                    .clone(),
            )
        } else {
            scrape_names.push(
                scrape
                    .split(';')
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()[0]
                    .clone(),
            )
        }
    }

    return scrape_names;
}

// pub fn get_saved_scrapes(include_combined: bool) -> (Vec<Scrape>, Vec<CombinedScrape>) {
//     let mut scrapes = Vec::new();
//     let mut combined_scrapes = Vec::new();

//     let file_content = include_str!("../scrapes.txt");

//     let lines: Vec<&str> = file_content.trim().split('\n').collect();
//     let saved_scrapes: Vec<&str> = lines.iter().map(|l| l.trim()).collect();

//     for scrape_str in saved_scrapes {
//         let scrape_parts: Vec<&str> = scrape_str.split(';').map(|x| x.trim()).collect();

//         if include_combined && scrape_parts[0] == "combined" {
//             let name = scrape_parts[1];
//             let scrapes_in_combined = scrape_parts[2]
//                 .replace("[", "")
//                 .replace("]", "")
//                 .replace("\"", "")
//                 .split(',')
//                 .map(|s| s.trim().replace(",", ""))
//                 .map(|s| get_scrape_from_str(&s))
//                 .collect::<Vec<Scrape>>();

//             let combined_scrape = CombinedScrape {
//                 name: name.to_string(),
//                 scrapes: scrapes_in_combined,
//             };

//             combined_scrapes.push(combined_scrape);
//         } else if scrape_parts[0] != "combined" {
//             let scrape = get_scrape_from_str(scrape_str);
//             scrapes.push(scrape);
//         }
//     }

//     (scrapes, combined_scrapes)
// }

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

    let title: Option<String> = if data[7].trim().len() > 0 {
        Some(data[7].clone().trim().to_string())
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

    let export: Option<String> = if data[9].trim().len() > 0 {
        Some(data[9].clone().trim().to_string())
    } else {
        None
    };

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
        export,
    }
}
