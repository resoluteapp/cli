#![feature(proc_macro_hygiene, decl_macro)]

use cmd::{create, list};
use reqwest::blocking::Client;

mod api;
mod cli;
mod cmd;
mod conf;

fn main() {
    let matches = cli::setup();
    match matches.subcommand() {
        Some(("list", _)) => {
            let client = Client::new();
            list::run(client);
        }
        Some(("create", matches)) => {
            let client = Client::new();
            create::run(matches, client);
        }
        _ => unreachable!(),
    }
}
