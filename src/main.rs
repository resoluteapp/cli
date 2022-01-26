#![feature(proc_macro_hygiene, decl_macro)]

use cmd::login;

mod cli;
mod cmd;

fn main() {
    let matches = cli::setup();
    match matches.subcommand() {
        Some(("login", login_matches)) => {
            login::run();
        }
        _ => unreachable!(),
    }
}
