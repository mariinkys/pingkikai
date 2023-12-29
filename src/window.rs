use crate::check_site;
use iced::widget::{column, container, text, Button, TextInput};
use iced::{executor, Application, Command, Length, Theme};

pub struct State {
    text_input: String,
    result_text: String,
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
        let my_text_input =
            TextInput::new("Enter a site. Ej: https://mariinkys.dev", &self.text_input)
                .on_input(Messages::OnInput)
                .padding(15);

        let my_button: Button<'_, Messages> = Button::new("Check Site")
            .on_press(Messages::OnPressed)
            .padding(15);

        let my_result_text = text(&self.result_text.to_string());

        container(
            column!(my_text_input, my_button, my_result_text)
                .align_items(iced::Alignment::Center)
                .padding(50)
                .spacing(25),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
