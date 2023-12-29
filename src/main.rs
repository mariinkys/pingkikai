mod window;
use iced::{Application, Settings};
use window::State;

fn main() {
    load_create_config();

    State::run(Settings::default()).expect("Pingkikai");
}

pub fn check_site(url: String) -> bool {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .expect("Failed to create reqwest client");

    match client.get(url).send() {
        Ok(response) => response.status().is_success(),
        Err(_err) => false,
    }
}

//TODO: Retrieve config file
fn load_create_config() {
    match std::env::consts::OS {
        "linux" => println!("Running on Linux"),
        "macos" => println!("Running on macOS"),
        "windows" => println!("Running on Windows"),
        _ => {
            println!("Running on an unknown operating system");
            std::process::exit(1);
        }
    }
}

//TODO: Save config file
