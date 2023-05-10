use scraper::{Html, Selector};
use clap::{Arg, Command, ArgMatches};
use std::fs::OpenOptions;

pub fn get_send_command() -> Command {
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
            Arg::new("keys")//headers?
                .short('k')
                .long("keys")
                .help("qweqwe")
                .num_args(1..)
                .required(false),
        )
        //.get_matches_from(itr)
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
        .subcommand(get_send_command())
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

fn scrape(args: &ArgMatches) {
    println!("SCRAPE SUB COMMAND");
    let url: String = args.get_one::<String>("url").unwrap().to_string();
    //selectors (String)
    let mut selectors = args
        .get_many::<String>("selectors")
        .unwrap()
        .map(|s| s.as_str());

    let test = selectors.next().unwrap();

    //

    let html = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = Html::parse_document(&html);
    let selector = Selector::parse("h3.news-list__headline").unwrap();
    //let select = document.select(&selector);
    let content = document.select(&selector).map(|x| x.inner_html());
    //println!("Select: {:?}", test);
    for element in content {
        println!("Content: {}", element);
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
