use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;

#[macro_use]
extern crate nickel;
use nickel::Nickel;

extern crate chrono;
use chrono::{DateTime, Local};

extern crate clap;
use clap::{Arg, Command};

fn formatted_time_entry() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn record_entry_in_log(filename: &String, bytes: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(filename)?;
    file.write_all(bytes)?;
    Ok(())
}

fn log_time(filename: &String) -> io::Result<String> {
    let entry = formatted_time_entry();
    {
        let bytes = entry.as_bytes();

        record_entry_in_log(filename, &bytes)?;
    }
    Ok(entry)
}

fn do_log_time(logfile_path: &String, auth_token: &Option<String>) -> String {
    match log_time(logfile_path) {
        Ok(entry) => format!("Entry Logged: {}", entry),
        Err(e) => format!("Error: {}", e),
    }
}

fn main() {
    let matches = Command::new("simple-log")
        .version("v0.0.1")
        .arg(
            Arg::new("LOG FILE")
                .short('l')
                .long("logfile")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("AUTH TOKEN")
                .short('t')
                .long("token")
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    let logfile_path = matches.get_one::<String>("LOG FILE").unwrap().to_string();
    let auth_token = match matches.get_one::<String>("AUTH TOKEN") {
        Some(str) => Some(str.to_string()),
        None => None,
    };

    let mut server = Nickel::new();
    server.utilize(router! {
        get "**" => |_req, _res| {
            do_log_time(&logfile_path, &auth_token)
        }
    });

    server
        .listen("127.0.0.1:6767")
        .expect("Failed to start server");
}
