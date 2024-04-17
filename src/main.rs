use std::env;

use circle_button::circle_button::CircleButtonStyle;
// [] start and create the (pk, sk)
// [] choose a device to store the sk
// [] create the password (symmetric key) and encrypt the pk and sk stored on the USB
// [] decrypt the sk and check if the format is correct (PEM)
use iced::{
    alignment, font, theme,
    widget::{button, column, container, row, text, Button, Container, Row, Text},
    window::Position,
    Application, Command, Element, Length, Settings, Size, Theme,
};

use iced_aw::floating_element::Anchor;
use iced_aw::{helpers::floating_element, BOOTSTRAP_FONT};
use login::{login, unlock_wallet};
use step::step::{Step, Steps};
use utils::{generate_key_pair, get_keys, is_pk_key_created};

use crate::utils::utils::{pad16, pad32};
use data_struct::account::account::Account;

mod circle_button;
mod data_struct;
mod enums;
mod login;
mod step;
mod utils;

fn main() -> iced::Result {
    env::set_var("RUST_BACKTRACE", "1");
    let settings: iced::Settings<()> = iced::Settings {
        window: iced::window::Settings {
            icon: iced::window::icon::from_file(format!(
                "{}/resources/icon.png",
                env!("CARGO_MANIFEST_DIR")
            ))
            .ok(),
            position: Position::Centered,
            size: Size::new(800., 600.),
            min_size: Some(Size::new(475., 500.)),
            ..iced::window::Settings::default()
        },
        id: Some("PassVault".to_string()),
        ..Default::default()
    };
    ModalExample::run(settings)
}

#[derive(Clone, Debug)]
enum Message {
    PasswordChanged(String),
    ConfirmPasswordChanged(String),
    SavePassword,
    #[allow(dead_code)]
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
    Start,
    UnlockWallet,
    AddAccount,
    None,
}

#[derive(Debug)]
enum ModalExample {
    Loading,
    Loaded(State),
}

#[derive(Default, Debug)]
pub struct State {
    theme: Theme,
    password: String,
    confirm_password: String,
    step: Steps,
    public_key: Option<String>,
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for ModalExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ModalExample, Command<Message>) {
        (
            ModalExample::Loading,
            Command::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("PassVault")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self {
            ModalExample::Loading => {
                if let Message::Loaded(_) = message {
                    *self = match is_pk_key_created() {
                        true => ModalExample::Loaded(State {
                            step: Steps::UnlockWallet,
                            ..Default::default()
                        }),
                        false => ModalExample::Loaded(State::default()),
                    }
                }
            }
            ModalExample::Loaded(state) => match message {
                Message::PasswordChanged(password) => state.password = password,
                Message::ConfirmPasswordChanged(password) => state.confirm_password = password,
                Message::SavePassword => {
                    if state.password != state.confirm_password {
                        println!("Password not match!")
                    } else if state.password.len() < 8
                        || state.password.chars().all(char::is_alphanumeric)
                    {
                        println!("Weak password!")
                    } else {
                        generate_key_pair(&state.password);
                        state.step = Steps::PasswordManager
                    }
                }
                Message::UnlockWallet => {
                    //TODO: check if the password is correct -> try decrypt
                    state.public_key = match get_keys(&state.password) {
                        Ok((pk, _)) => Some(pk),
                        Err(_) => None,
                    };
                    if state.public_key.is_none() {
                        println!("Wrong password!");
                    } else {
                        state.step = Steps::PasswordManager;
                    }
                }
                Message::Start => state.step = Steps::Login,
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self {
            ModalExample::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            ModalExample::Loaded(state) => view_logic(state),
        }
    }

    fn theme(&self) -> Theme {
        match self {
            ModalExample::Loading => Theme::Light,
            ModalExample::Loaded(state) => state.theme.clone(),
        }
    }
}

fn view_logic(state: &State) -> Element<'static, Message> {
    let advance_button: Row<'_, Message> = match state.step {
        Steps::Welcome => row![button(text("Start").size(30))
            .padding(8)
            .on_press(Message::Start)],
        Steps::Login => row![],
        Steps::SecretKeyLocation => row![],
        Steps::PasswordManager => row![],
        Steps::UnlockWallet => row![],
    };
    let content = match state.step {
        Steps::Login => login(&state),
        Steps::Welcome => welcome(),
        Steps::SecretKeyLocation => sk_location(state),
        Steps::PasswordManager => password_manager(state),
        Steps::UnlockWallet => unlock_wallet(state),
    };
    Container::new(column![
        content,
        container(advance_button)
            .padding(10)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

fn welcome() -> Element<'static, Message> {
    Container::new(
        column![
            text("Welcome to PassVault!").size(50),
            text("The place where your password are secure.").size(26)
        ]
        .align_items(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Shrink)
    .center_y()
    .center_x()
    .into()
}

fn sk_location(state: &State) -> Element<'static, Message> {
    Container::new(column![
        text("Warning!").size(50),
        text("Now it's time to decide the location of the secret key which allow to decrypt your passwords.").size(26)
    ].align_items(iced::Alignment::Center))
            .width(Length::Fill)
            .height(Length::Shrink)
            .center_y()
            .center_x()
            .into()
}

fn password_manager(state: &State) -> Element<'static, Message> {
    let account = Account::new(String::from("Windows"), String::from("value"), String::from("password"));
    let content = floating_element(
        Container::new(column![
            row![text("Your keys!").size(50)].align_items(iced::Alignment::Start),
            row![account_widget(account)].align_items(iced::Alignment::Start)
        ])
        .width(Length::Fill)
        .height(Length::Fill),
        Button::new(
            Text::new("ADD")
                .font(BOOTSTRAP_FONT)
                .size(20)
                .line_height(1.0)
                .shaping(text::Shaping::Advanced),
        )
        .on_press(Message::AddAccount)
        .padding(5)
        .style(theme::Button::Custom(Box::new(CircleButtonStyle::new(
            theme::Button::Primary,
        )))),
    )
    .anchor(Anchor::SouthEast)
    .offset(20.0)
    .hide(false);

    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .center_x()
        .center_y()
        .into()
}
fn account_widget(account: Account) -> Element<'static, Message> {
    Container::new(row![
        column![
            row![text(account.get_host())],
            row![text(account.get_username())]
        ],
        column![
            Button::new(Text::new("Delete").font(BOOTSTRAP_FONT)
            .size(20)
            .line_height(1.0)
            .shaping(text::Shaping::Advanced)).on_press(Message::AddAccount)
        .padding(5)
        .style(theme::Button::Custom(Box::new(CircleButtonStyle::new(
            theme::Button::Secondary,
        ))))
        ]
    ]).into()
}
