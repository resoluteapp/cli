#![feature(proc_macro_hygiene, decl_macro)]

use cmd::{list, login};
use reqwest::blocking::Client;

mod api;
mod cli;
mod cmd;
mod conf;

fn main() {
    let matches = cli::setup();
    match matches.subcommand() {
        Some(("login", _)) => {
            login::run();
        }
        Some(("list", _)) => {
            let client = Client::new();
            list::run(client);
        }
        _ => unreachable!(),
    }
}
