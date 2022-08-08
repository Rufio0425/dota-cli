#![allow(unused)]
use clap::Parser;

#[derive(Parser,Default,Debug)]
struct Cli {
    request: String
}

mod dota_apis;

fn main() {
    let cli = Cli::parse();
    match cli.request.as_str() {
        "get-heros" => dota_apis::get_all_heros(),
        _ => {
            println!("Improper request");
        }
    }
}