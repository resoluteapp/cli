mod cli;

fn main() {
    let matches = cli::setup();
    match matches.subcommand() {
        Some(("login", login_matches)) => {
            println!("Login command executed");
        }
        _ => unreachable!(),
    }
}
