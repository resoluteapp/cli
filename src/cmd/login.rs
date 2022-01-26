use rocket::config::{Environment, LoggingLevel};
use rocket::http::RawStr;
use rocket::routes;
use rocket::{get, Config};

pub fn run() {
    if webbrowser::open(AUTH_URL).is_err() {
        println!(
            "Failed to open {0} in your web browser. Please go to it manually: {0}",
            AUTH_URL
        );
    }

    let config = Config::build(Environment::Development)
        .log_level(LoggingLevel::Off)
        .finalize()
        .expect("Failed to finalize server");
    rocket::custom(config)
        .mount("/", routes![successful_obtain, failed_obtain])
        .launch();
}

#[get("/?<code>")]
fn successful_obtain(code: &RawStr) -> &'static str {
    println!("Code: {}", code);
    "All set, just close this tab and return to the terminal"
}

#[get("/?<error>", rank = 2)]
fn failed_obtain(error: &RawStr) -> String {
    println!("{}", error);
    format!("Error: {}, please return to the terminal.", error)
}

const AUTH_URL: &str = "https://useresolute.com/api/oauth/authorize?client_id=6f5b1c7280f2b0555e0b089eb50de061&scope=reminders:view,reminders:create";
