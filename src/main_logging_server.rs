#[macro_use]
extern crate nickel;
extern crate chrono;

use chrono::{DateTime, Local};
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;

use nickel::Nickel;

fn formatted_time_entry() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn record_entry_in_log(filename: &str, bytes: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(filename)?;
    file.write_all(bytes)?;
    Ok(())
}

fn log_time(filename: &'static str) -> io::Result<String> {
    let entry = formatted_time_entry();
    let bytes = entry.as_bytes();

    record_entry_in_log(filename, &bytes)?;
    Ok(entry)
}

fn do_log_time() -> String {
    match log_time("log.txt") {
        Ok(entry) => format!("Entry Logged: {}", entry),
        Err(e) => format!("Error: {}", e)
    }
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            do_log_time()
        }
    });

    match server.listen("127.0.0.1:6767") {
        Ok(_) => println!("Server started successfully."),
        Err(e) => eprintln!("Failed to start server: {}", e),
    }
}
