use clap::{Arg, Command};
use reqwest::Client;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

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
        .subcommand(
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
                ),
        )
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
        Some(("send", sub_c)) => send_command(), // clone was used
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

fn send_command() {
    println!("SEND SUB COMMAND");
}

fn save_command(name: &str, method: &str, url: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("commands.txt")
        .unwrap();

    writeln!(file, "{} {} {}", name, method, url).unwrap();
}
