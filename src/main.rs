mod api;
mod cli;
mod cmd;
mod conf;

fn main() {
    let matches = cli::setup().get_matches();
    match matches.subcommand() {
        Some(("list", _)) => cmd::list::run(),
        Some(("create", args)) => cmd::create::run(args),
        Some(("delete", _)) => cmd::delete::run(),
        Some(("completion", args)) => cmd::completion::run(args),
        _ => unreachable!(),
    }
}
