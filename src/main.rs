extern crate chrono;
use chrono::*;

fn main() {
    let local: DateTime<Local> = Local::now();
    println!("{}", local);
}