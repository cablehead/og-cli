use std::io::{self, Read};
use std::process;
use opengraph_rs::scraper::{extract, Opts};
use serde_json;

fn main() {
    let mut input = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error reading from stdin: {}", err);
        process::exit(1);
    }

    let mut reader = input.as_bytes();
    let options = Opts::default();

    match extract(&mut reader, options) {
        Ok(object) => {
            if let Ok(json) = serde_json::to_string(&object) {
                println!("{}", json);
            } else {
                eprintln!("Error serializing object to JSON");
                process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("Error during extraction: {}", err);
            process::exit(1);
        }
    }
}