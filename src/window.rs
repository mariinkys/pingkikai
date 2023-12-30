use iced::widget::{column, container, row, text, Button, TextInput};
use iced::{executor, Application, Command, Length, Theme};

pub struct State {
    text_input: String,
    result_text: String,
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Messages {
    OnInput(String),
    OnPressed,
}

impl Application for State {
    type Message = Messages;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            State {
                text_input: String::from(""),
                result_text: String::from(""),
                theme: system_theme_mode(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pingkikai")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Messages::OnInput(data) => self.text_input = data,
            Messages::OnPressed => {
                let result: bool = check_site(self.text_input.clone());
                if result {
                    self.result_text = String::from("Website is Up")
                } else {
                    self.result_text = String::from("Website is Down")
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let url_input = TextInput::new("Enter a site. Ej: https://mariinkys.dev", &self.text_input)
            .on_input(Messages::OnInput)
            .padding(15);

        let check_site_button: Button<'_, Messages> = Button::new("Check Site")
            .on_press(Messages::OnPressed)
            .padding(15);

        let result_text = text(&self.result_text.to_string());

        let input_row = row![url_input, check_site_button].spacing(25);

        container(
            column!(input_row, result_text)
                .align_items(iced::Alignment::Center)
                .padding(50)
                .spacing(25),
        )
        .width(Length::Fill)
        .center_x()
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn check_site(url: String) -> bool {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .expect("Failed to create reqwest client");

    match client.get(url).send() {
        Ok(response) => response.status().is_success(),
        Err(_err) => false,
    }
}

fn system_theme_mode() -> Theme {
    match dark_light::detect() {
        dark_light::Mode::Light | dark_light::Mode::Default => Theme::Light,
        dark_light::Mode::Dark => Theme::Dark,
    }
}
