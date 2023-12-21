use crate::{commands::scrape, enums::Presentation, utils::get_saved_scrape, structs::Scrape};

pub fn run(name: String) {
    let scrape_data = get_saved_scrape(&name);

    match scrape_data {
        Some(scrapes) => {
            //let data: Vec<&str> = data_str.split(";").collect();

            for scrape in scrapes {
                run_scrape(scrape);
            }

            // match data[0] {
            //     "combined" => {
            //         let scrapes = data[2][1..data[2].len() - 1]
            //             .split(", ")
            //             .collect::<Vec<&str>>()
            //             .iter()
            //             .map(|&s| s.trim().replace("\"", "").to_string())
            //             .collect::<Vec<String>>();

            //         run_combined_scrapes(scrapes);
            //     }
            //     _ => run_scrape(data),
            // }
        }
        None => println!("Scrape {} not found!", &name),
    }
}

fn run_scrape(scrape: Scrape) {
    // let url = data[1];
    // let selectors = data[2][1..data[2].len() - 1]
    //     .split(", ")
    //     .collect::<Vec<&str>>()
    //     .iter()
    //     .map(|&s| s.trim().replace("\"", "").to_string())
    //     .collect::<Vec<String>>();

    // let keys = data[3][1..data[3].len() - 1]
    //     .split(", ")
    //     .collect::<Vec<&str>>()
    //     .iter()
    //     .map(|&s| s.trim().replace("\"", "").to_string())
    //     .collect::<Vec<String>>();

    // let attributes: Option<Vec<String>> = if data[4].len() > 2 {
    //     Some(
    //         data[4][1..data[4].len() - 1]
    //             .split(", ")
    //             .collect::<Vec<&str>>()
    //             .iter()
    //             .map(|&s| s.trim().replace("\"", "").to_string())
    //             .collect::<Vec<String>>(),
    //     )
    // } else {
    //     None
    // };

    // let prefixes: Option<Vec<String>> = if data[5].len() > 2 {
    //     Some(
    //         data[5][1..data[5].len() - 1]
    //             .split(", ")
    //             .collect::<Vec<&str>>()
    //             .iter()
    //             .map(|&s| s.trim().replace("\"", "").to_string())
    //             .collect::<Vec<String>>(),
    //     )
    // } else {
    //     None
    // };

    // let suffixes: Option<Vec<String>> = if data[6].len() > 2 {
    //     Some(
    //         data[6][1..data[6].len() - 1]
    //             .split(", ")
    //             .collect::<Vec<&str>>()
    //             .iter()
    //             .map(|&s| s.trim().replace("\"", "").to_string())
    //             .collect::<Vec<String>>(),
    //     )
    // } else {
    //     None
    // };

    // let title: Option<String> = if data[7].len() > 0 {
    //     Some(data[7].to_string())
    // } else {
    //     None
    // };

    // let presentation = if data[8].len() > 0 {
    //     if data[8].to_lowercase() == "table" {
    //         Some(Presentation::Table)
    //     } else {
    //         Some(Presentation::List)
    //     }
    // } else {
    //     None
    // };

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

// fn run_combined_scrapes(scrapes: Vec<Scrape>) {
//     let mut saved_scrapes: Vec<Vec<&str>> = Vec::new();

//     for scrape in &scrapes {
//         let saved_scrape = get_saved_scrape(scrape);

//         match saved_scrape {
//             Some(data_str) => {
//                 let data: Vec<&str> = data_str.split(";").collect();
//                 saved_scrapes.push(data);
//             }
//             None => {
//                 println!("Scrape was not found: '{}'", scrape);
//             }
//         }
//     }

//     if saved_scrapes.len() == scrapes.len() {
//         for scrape in saved_scrapes {
//             run_scrape(scrape);
//         }
//     }
// }
