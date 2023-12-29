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
        String::from("Iced Test")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Messages::OnInput(data) => self.text_input = data,
            Messages::OnPressed => self.result_text = String::from("Sad UwU Noises"),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let my_text_input =
            TextInput::new("Enter Text", &self.text_input).on_input(|data| Messages::OnInput(data));

        let my_button: Button<'_, Messages> =
            Button::new("Placeholder Text").on_press(Messages::OnPressed);

        let my_result_text = text(format!("{}", &self.result_text));

        container(column!(my_text_input, my_button, my_result_text))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
