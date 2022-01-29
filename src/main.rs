use cmd::{create, delete, list};

mod api;
mod cli;
mod cmd;
mod conf;

fn main() {
    let matches = cli::setup();
    match matches.subcommand() {
        Some(("list", _)) => list::run(),
        Some(("create", matches)) => create::run(matches),
        Some(("delete", _)) => delete::run(),
        _ => unreachable!(),
    }
}
