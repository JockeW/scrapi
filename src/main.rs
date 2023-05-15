use clap::{Arg, ArgMatches, Command};
use colored::Colorize;
use scraper::{element_ref::Text, Html, Selector, Node};
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
                .required(true),
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

    let selectors = args
        .get_many::<String>("selectors")
        .unwrap()
        .map(|s| s.as_str());

    let keys: Vec<&str> = args
        .get_many::<String>("keys")
        .unwrap()
        .map(|s| s.as_str())
        .collect();

    if keys.len() != selectors.len() {
        println!("{}: Keys needs to be as many as selectors", "error".bold().color("red"));
        return;
    }

    let url: String = args.get_one::<String>("url").unwrap().to_string();
    let html = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = Html::parse_document(&html);

    let mut contents: Vec<Vec<String>> = Vec::new();

    for s in selectors {
        println!("SELECTOR: {}", s);
        let selector = Selector::parse(s).unwrap();
        let element_ref: Vec<_> = document.select(&selector).collect();

        let mut content_vec: Vec<String> = Vec::new();

        for element in element_ref {
            let outer_text: Vec<&str> = element
                .children()
                .filter_map(|node| match node.value() {
                    Node::Text(text) => Some(&text[..]),
                    _ => None,
                })
                .collect();

            //println!("{:?}", outer_text);
            //TODO: Maybe add to get text of child nodes as well. (element.children())

            let element_text: String = outer_text
                .join("");

            content_vec.push(element_text);
        }

        contents.push(content_vec);
    }

    println!("SELECTORS CONTENT: {:?}", contents);

    let mut all_content: Vec<Vec<(&str, &str)>> = Vec::new();

    for content_index in 0..contents.first().expect("NO CONTENT").len() {
        let mut chunk: Vec<(&str, &str)> = Vec::new();
        for (i, content) in contents.iter().enumerate() {
            let header = keys[i];
            let value = content[content_index].trim();
            chunk.push((header, value));
        }

        all_content.push(chunk);
    }

    println!("CONTENT WITH KEYS: {:?}", all_content);

    println!();
    for chunk in all_content {
        //TODO: Print list or table. Just list for now
        for data in chunk {
            let header = data.0;
            let value = data.1;
            println!("{}: {}", header.bold(), value);
        }
        println!();
    }
}

fn save_command(name: &str, method: &str, url: &str) {
    // let mut file = OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("commands.txt")
    //     .unwrap();

    // writeln!(file, "{} {} {}", name, method, url).unwrap();
}
