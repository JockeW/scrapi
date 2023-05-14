use clap::{Arg, ArgMatches, Command};
use scraper::{Html, Selector, element_ref::Text};
use std::fs::OpenOptions;

fn send_command() -> Command {
    Command::new("scrape")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .value_name("url")
                .help("URL to send the request to")
                .required(true),
        )
        .arg(
            Arg::new("selectors")
                .short('s')
                .long("selectors")
                .help("fghfgh")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("keys") //headers?
                .short('k')
                .long("keys")
                .help("qweqwe")
                .num_args(1..)
                .required(false),
        )
    //.get_matches_from(itr)
}

//Inspect source html and possibility to searh and filter...
fn inspect_command() -> Command {
    Command::new("inspect")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .value_name("url")
                .help("URL to page to inspect source on")
                .required(true),
        )
        .arg(
            Arg::new("search")
                .short('s')
                .long("search")
                .help("Search term")
                .required(false),
        )
        .arg(
            Arg::new("filter")
                .short('f')
                .long("filter")
                .help("Filter source on these values")
                .num_args(1..)
                .required(false),
        )
}

fn main() {
    let matches = Command::new("HTTP CLI")
        .version("1.0")
        .author("Joakim Wilhelmsson")
        .about("A command-line HTTP client")
        .arg(
            Arg::new("test")
                .help("Testing argument")
                .long("test")
                .value_name("test"),
        )
        .subcommand(send_command())
        .subcommand(inspect_command())
        .subcommand(
            Command::new("saved").arg(
                Arg::new("list")
                    .short('l')
                    .long("list")
                    .help("List saved requests")
                    .required(false),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("scrape", cmd)) => scrape(cmd),
        // Some(("push",   sub_c)) => {}, // push was used
        // Some(("commit", sub_c)) => {}, // commit was used
        _ => {} // Either no subcommand or one not tested for...
    };

    //let say_hello: Option<&str> = matches.get_one::<String>("arg").map(|s| s.as_str());
    if let Some(msg) = matches.get_one::<String>("test").map(|s| s.as_str()) {
        println!("Hello {msg}");
    }
}

struct Content {
    key: Option<String>,
    value: String,
}

fn scrape(args: &ArgMatches) {
    println!("SCRAPE SUB COMMAND");
    let url: String = args.get_one::<String>("url").unwrap().to_string();
    let html = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = Html::parse_document(&html);

    let selectors = args
        .get_many::<String>("selectors")
        .unwrap()
        .map(|s| s.as_str());

    let keys: Vec<&str> = args
        .get_many::<String>("keys")
        .unwrap()
        .map(|s| s.as_str())
        .collect();

    let mut contents: Vec<String> = Vec::new();

    for s in selectors {
        println!("SELECTOR: {}", s);
        let selector = Selector::parse(s).unwrap();
        //let element_ref =  document.select(&selector).collect();
        let content_vec: String = document
            .select(&selector)
            .flat_map(|x| x.text())
            .collect();

        contents.push(content_vec);
    }

    let mut all_content: Vec<Vec<(&str, &str)>> = Vec::new();

    for content_index in 0..contents.first().expect("NO CONTENT").len() {
        let mut chunk: Vec<(&str, &str)> = Vec::new();
        for (i, content) in contents.iter().enumerate() {
            let header = keys[i];
            let value = content;
            chunk.push((header, value));
        }

        all_content.push(chunk);
    }

    for chunk in all_content {
        //Print list or table. Just list for now
        for data in chunk {
            let header = data.0;
            let value = data.1;
            println!("{}: {}", header, value);
        }
        println!();
    }
}

// fn send_command(args: &ArgMatches) {
//     println!("SEND SUB COMMAND");
//     let url: String = args.get_one::<String>("url").unwrap().to_string();
//     let load = args.get_one::<String>("load");
//     if let Some(load) = load {
//         let load_str: &str = load;
//         //TODO: Load saved request and execute that
//     } else {
//         let response = reqwest::blocking::get(url).unwrap().text().unwrap();
//         let links = Dom::parse(&response).unwrap().to_json_pretty().unwrap();
//         //let test = serde_json::to_string_pretty(&response).unwrap();
//         println!("{}", response);
//     }

//     let save = args.get_one::<bool>("save");
//     if let Some(s) = save {
//         let shoud_save = *s;
//         if shoud_save {//TODO: Also only save if the request was successful
//             println!("SAVE THE REQUEST. TODO: ADD ARGS FOR SAVING, LIKE NAME");
//         }
//     }
// }

fn save_command(name: &str, method: &str, url: &str) {
    // let mut file = OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("commands.txt")
    //     .unwrap();

    // writeln!(file, "{} {} {}", name, method, url).unwrap();
}
