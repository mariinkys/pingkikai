use iced::widget::{column, container, row, text, Button, Row, Rule, Scrollable, TextInput};
use iced::{alignment, executor, Application, Color, Command, Length, Theme};

pub struct State {
    url_input: String,
    result: bool,
    theme: Theme,
    checked_site: bool,
    saved_sites: Vec<Site>,
}

#[derive(Debug, Clone)]
pub enum Messages {
    OnUrlInput(String),
    CheckSitePressed(String),
    AddSitePressed,
    DeleteSite(i64),
    SavedCheckSitePressed(i64),
}

impl Application for State {
    type Message = Messages;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            State {
                url_input: String::from(""),
                result: false,
                checked_site: false,
                theme: system_theme_mode(),
                saved_sites: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pingkikai")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Messages::OnUrlInput(data) => self.url_input = data,
            Messages::CheckSitePressed(data) => {
                self.checked_site = true;
                self.result = check_site(data);
            }
            Messages::AddSitePressed => {
                if !self.url_input.is_empty() {
                    let new_id: i64 = (self.saved_sites.len() + 1) as i64;
                    let new_status = check_site(self.url_input.to_string());

                    self.saved_sites.push(Site {
                        id: new_id,
                        url: (self.url_input.to_string()),
                        status: new_status,
                    });
                }
            }
            Messages::DeleteSite(id) => {
                if let Some(index) = self.saved_sites.iter().position(|site| site.id == id) {
                    self.saved_sites.remove(index);
                }
            }
            Messages::SavedCheckSitePressed(id) => {
                if let Some(index) = self.saved_sites.iter().position(|site| site.id == id) {
                    let site = &mut self.saved_sites[index];
                    site.status = check_site(site.url.to_string());
                }
            }
        }
        Command::none()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let url_input = TextInput::new("Enter a site. Ej: https://mariinkys.dev", &self.url_input)
            .on_input(Messages::OnUrlInput)
            .size(20)
            .padding(10);

        let check_site_button: Button<'_, Messages> = Button::new("Check Site")
            .on_press(Messages::CheckSitePressed(self.url_input.to_string()))
            .padding(10);

        let result_text = if self.result && self.checked_site {
            text("Website Is Up")
                .style(Color::from([0.0, 0.7, 0.0]))
                .size(20)
        } else if !self.result && self.checked_site {
            text("Website Is Down")
                .style(Color::from([0.8, 0.0, 0.0]))
                .size(20)
        } else {
            text("").size(20)
        };

        let add_site_button: Button<'_, Messages> = Button::new("+")
            .on_press(Messages::AddSitePressed)
            .padding(10);

        let sites: Vec<Row<'_, Messages>> = self
            .saved_sites
            .iter()
            .enumerate()
            .map(|(_i, site)| -> Row<'_, Messages> {
                let url_text = text(site.url.to_string()).width(Length::Fill).size(20);
                let status_text = if site.status {
                    text("Up")
                        .width(Length::Fill)
                        .size(20)
                        .style(Color::from([0.0, 0.7, 0.0]))
                        .horizontal_alignment(alignment::Horizontal::Center)
                } else {
                    text("Down")
                        .width(Length::Fill)
                        .size(20)
                        .style(Color::from([0.8, 0.0, 0.0]))
                        .horizontal_alignment(alignment::Horizontal::Center)
                };
                let delete_button = Button::new("Delete")
                    .on_press(Messages::DeleteSite(site.id))
                    .padding(10);
                let check_button = Button::new("Check")
                    .on_press(Messages::SavedCheckSitePressed(site.id))
                    .padding(10);

                row!(url_text, status_text, delete_button, check_button)
                    .align_items(iced::Alignment::Center)
                    .width(Length::Fill)
                    .spacing(25)
            })
            .collect();

        let sites_col = sites.into_iter().fold(column!().spacing(25), |col, site| {
            col.push(site).push(Rule::horizontal(10.0))
        });
        let sites_scrollable = Scrollable::new(sites_col).height(Length::Fill);

        let input_row = row![url_input, check_site_button, add_site_button]
            .spacing(25)
            .align_items(iced::Alignment::Center);

        container(
            column!(input_row, result_text, sites_scrollable)
                .align_items(iced::Alignment::Center)
                .padding(50)
                .spacing(25),
        )
        .width(Length::Fill)
        .center_x()
        .into()
    }
}

struct Site {
    id: i64,
    url: String,
    status: bool,
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
