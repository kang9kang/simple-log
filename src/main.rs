extern crate chrono;

use std::io::prelude::*;
use std::fs::File;
use std::io;
use chrono::*;

fn log_time(filename: &'static str) -> io::Result<()> {
    let local: DateTime<Local> = Local::now();
    let formatted_time = local.format("%Y").to_string();
    let bytes = formatted_time.as_bytes();
    let mut f = File::create(filename)?;
    f.write_all(bytes)?;
    Ok(())
}

fn main() {
    match log_time("log.txt") {
        Ok(..) => println!("File created!"),
        Err(..) => println!("Error: could not create file.")
    }
}