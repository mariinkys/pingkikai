mod window;
use iced::{Application, Settings};
use window::State;

fn main() {
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
