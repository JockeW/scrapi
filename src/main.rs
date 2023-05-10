use serde_json::{Result, Value};
use html5ever::{parse_document};
use html_parser::Dom;
use clap::{Arg, Command, ArgMatches};
use reqwest::Client;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub fn get_send_command() -> Command {
    Command::new("send")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .value_name("url")
                .help("URL to send the request to")
                .required(true),
        )
        .arg(
            Arg::new("load")
                .short('l')
                .long("load")
                .value_name("request_name")
                .help("Load a stored request and send that")
                .required(false),
        )
        .arg(
            Arg::new("save")
                .short('s')
                .long("save")
                .help("Save the request to reuse it later")
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
        Some(("send", cmd)) => send_command(cmd),
        // Some(("push",   sub_c)) => {}, // push was used
        // Some(("commit", sub_c)) => {}, // commit was used
        _ => {} // Either no subcommand or one not tested for...
    };

    //let say_hello: Option<&str> = matches.get_one::<String>("arg").map(|s| s.as_str());
    if let Some(msg) = matches.get_one::<String>("test").map(|s| s.as_str()) {
        println!("Hello {msg}");
    }

    // let client = Client::new();

    // let method: &str = *matches.get_one("method").unwrap();
    // let url: &str = *matches.get_one("url").unwrap();
    //let output: PathBuf = matches.get_one("output").unwrap().map(|s: &str| PathBuf::from(s));

    //let response = reqwest::blocking::get(url).unwrap().text().unwrap();

    // if let Some(output) = output {
    //     let mut file = File::create(output).unwrap();
    //     response.copy_to(&mut file).unwrap();
    // } else {
    //     let body = response.text().unwrap();
    //     println!("{}", body);
    // }

    // if let Some(name) = *matches.get_one("save").unwrap() {
    //     save_command(name, method, url);
    // }
}

fn send_command(args: &ArgMatches) {
    println!("SEND SUB COMMAND");
    let url: String = args.get_one::<String>("url").unwrap().to_string();
    let load = args.get_one::<String>("load");
    if let Some(load) = load {
        let load_str: &str = load;
        //TODO: Load saved request and execute that
    } else {
        let response = reqwest::blocking::get(url).unwrap().text().unwrap();
        let links = Dom::parse(&response).unwrap().to_json_pretty().unwrap();
        //let test = serde_json::to_string_pretty(&response).unwrap();
        println!("{}", response);
    }

    let save = args.get_one::<bool>("save"); 
    if let Some(s) = save {
        let shoud_save = *s;
        if shoud_save {//TODO: Also only save if the request was successful
            println!("SAVE THE REQUEST. TODO: ADD ARGS FOR SAVING, LIKE NAME");
        }
    }
}

fn save_command(name: &str, method: &str, url: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("commands.txt")
        .unwrap();

    writeln!(file, "{} {} {}", name, method, url).unwrap();
}
