use std::env;

use circle_button::circle_button::CircleButtonStyle;
use custom_widget::{card::card::Card as PersonalCard, image_button::{self, image_button::image_button}};
use enums::Modal;
// [] start and create the (pk, sk)
// [] choose a device to store the sk
// [] create the password (symmetric key) and encrypt the pk and sk stored on the USB
// [] decrypt the sk and check if the format is correct (PEM)
use iced::{
    alignment::{self, Horizontal}, font, theme, widget::{button, column, container, row, scrollable, shader::wgpu::naga::proc::index, text, Button, Column, Container, Row, Text, TextInput}, window::Position, Alignment, Application, Color, Command, Element, Font, Length, Padding, Settings, Size, Theme
};

use iced_aw::{floating_element::Anchor, modal, Card};
use iced_aw::{helpers::floating_element, BOOTSTRAP_FONT};
use login::{login, unlock_wallet};
use serde::de;
use step::step::{Step, Steps};
use utils::{generate_key_pair, get_keys, is_pk_key_created};

use crate::utils::{decrypt_data, utils::{pad16, pad32}};
use data_structure::account::account::{deserialize_accounts, serialize_accounts, Account};

mod circle_button;
mod data_structure;
mod enums;
mod login;
mod step;
mod utils;
mod custom_widget;

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
            min_size: Some(Size::new(600., 600.)),
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
    SaveAccount,
    DeleteAccount(usize),
    ShowPassword(usize),
    HidePassword,
    EditAccount(usize),
    CopyPassword(usize),
    CloseAddModal,
    UsernameChange(String),
    HostChange(String),
    SaveEdit,
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
    symm: String,
    accounts: Vec<Account>,
    public_key: Option<String>,
    show_password: Option<usize>,
    modal: Option<Modal>,
    host_name: String,
    username: String,
    edit_index: Option<usize>
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
                            accounts: Vec::new(),
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
                        && state.password.chars().all(char::is_alphanumeric)
                    {
                        println!("Weak password!")
                    } else {
                        generate_key_pair(&state.password);
                        state.symm = state.password.clone();
                        state.password.clear();
                        state.confirm_password.clear();
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
                        let accounts = deserialize_accounts(&state.password).unwrap();
                        state.accounts = accounts;
                        state.symm = state.password.clone();
                        state.password.clear();
                        state.step = Steps::PasswordManager;
                    }
                },
                Message::DeleteAccount(index) => {
                    state.accounts.remove(index);

                    let _ = serialize_accounts(&state.accounts, &state.symm);
                    println!("Delete account at index: {}", index);
                },
                Message::Start => state.step = Steps::Login,
                Message::ShowPassword(index) => {
                    state.show_password = Some(index);
                    println!("Show password at index: {}", index);
                },
                Message::HidePassword => {
                    state.show_password = None;
                    println!("Hide password");
                },
                Message::CopyPassword(index) => {
                    println!("Copy account at index: {}", index);
                },
                Message::EditAccount(index) => {

                    let account_to_edit = state.accounts.get(index).unwrap();

                    state.host_name = account_to_edit.get_host().clone();
                    state.username = account_to_edit.get_username().clone();
                    state.password = account_to_edit.get_key().clone();
                    state.edit_index = Some(index);

                    state.modal = Some(Modal::EDIT);
                    println!("Edit account at index: {}", index);
                },
                Message::SaveEdit => { 

                    println!("{}", state.confirm_password);

                    if state.password.len() < 8 && state.password.chars().all(char::is_alphanumeric) {
                        println!("Weak password!");
                        return Command::none();
                    } else if state.password != state.confirm_password {
                        println!("Password not match!");
                        return Command::none();
                    }

                    state.accounts[state.edit_index.unwrap()].set_host(state.host_name.clone());
                    state.accounts[state.edit_index.unwrap()].set_username(state.username.clone());
                    state.accounts[state.edit_index.unwrap()].set_key(state.password.clone());

                    let _ = serialize_accounts(&state.accounts, &state.symm);

                    state.edit_index = None;
                    state.password.clear();
                    state.host_name.clear();
                    state.username.clear();
                    state.confirm_password.clear();
                    
                    state.modal = None;
                },
                Message::AddAccount => {
                    state.modal = Some(Modal::ADD);
                    println!("Add account");
                },
                Message::SaveAccount => {
                    //TODO: Checks, encrypt and update json 

                    if state.password.len() < 8 && state.password.chars().all(char::is_alphanumeric) {
                        println!("Weak password!");
                        return Command::none();
                    } else if state.password != state.confirm_password {
                        println!("Password not match!");
                        return Command::none();
                    }

                    let new_account = Account::new(state.host_name.clone(), state.username.clone(), state.password.clone());

                    state.password.clear();
                    state.host_name.clear();
                    state.username.clear();
                    state.confirm_password.clear();
                    
                    state.accounts.push(new_account);

                    let _ = serialize_accounts(&state.accounts, &state.symm);
                    
                    state.modal = None;
                },
                Message::CloseAddModal => {

                    state.password.clear();
                    state.host_name.clear();
                    state.username.clear();
                    state.confirm_password.clear();
                    
                    state.modal = None;
                },
                Message::HostChange(host) => {
                    state.host_name = host;
                },
                Message::UsernameChange(username) => {
                    state.username = username;
                }
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

    let mut account_list: Column<'static, Message> = Column::new();
    for (index, account) in state.accounts.iter().enumerate() {
        account_list = account_list.push(
            row![account_widget(account.clone(), index, &state)].padding(2.)
        );
    }

    let main_content = if state.accounts.len() > 4 {
        Container::new(scrollable( column![
            row![text("Your keys!").size(50)].align_items(iced::Alignment::Start),
            row![account_list].align_items(iced::Alignment::Start)
        ]
        .align_items(iced::Alignment::Center)
        .width(Length::Fill)))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
    } else {
        Container::new(column![
            row![text("Your keys!").size(50)].align_items(iced::Alignment::Start),
            row![account_list].align_items(iced::Alignment::Start)
        ]
        .align_items(iced::Alignment::Center)
        .width(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
    };

    let modal_overlay: Option<Element<'static, Message>> =  match &state.modal {

        Some(m) => {
            match m {
                Modal::ADD =>  add_modal_view(&state),
                Modal::EDIT => edit_modal_view(&state),
            }
        },
        None => None,
    };
    
    let content = floating_element(
       main_content,
        Button::new(
                container(
                    Text::new("New Item +")
                .font(Font {
                    weight: font::Weight::Bold,
                    ..BOOTSTRAP_FONT
                })
                .size(18)
                .line_height(1.0)
                .shaping(text::Shaping::Advanced)
                ).padding(8)
        )
        .on_press(Message::AddAccount)
        .padding(5)
        .style(theme::Button::Custom(Box::new(CircleButtonStyle::new(
            theme::Button::Primary,
        )))),
    )
    .anchor(Anchor::South)
    .offset(10.0)
    .hide(false);

    modal(Container::new(content)
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(10)
    , modal_overlay).backdrop(Message::CloseAddModal).on_esc(Message::CloseAddModal).align_y(alignment::Vertical::Center).into()
}

fn account_widget(account: Account, index: usize, state: &State) -> Element<'static, Message> {
    let (switch_visibility, visibility_content) = if state.show_password == Some(index) {
       (image_button("visibility_off", Message::HidePassword), account.get_key())
    } else {
        (image_button("visibility_on", Message::ShowPassword(index)), account.get_username())
    };
    let delete_button = image_button("delete", Message::DeleteAccount(index));
    let edit_button = image_button("edit", Message::EditAccount(index));
    let copy_button = image_button("copy", Message::CopyPassword(index));

    let button_column = column![
            row![delete_button, edit_button, switch_visibility, copy_button].align_items(Alignment::Center)
        ].width(Length::FillPortion(1)).align_items(iced::Alignment::End);

    let account_column = column![
        row![text(account.get_host())
            .font(Font {
                weight: font::Weight::Semibold,
                ..BOOTSTRAP_FONT
            })
            .size(24)],
        row![text(visibility_content).size(22)]
    ].width(Length::FillPortion(2));
    Container::new(row![
        account_column,
        button_column
    
    ].align_items(Alignment::Center))
    .padding(Padding::new(20.))
    .width(600.)
    .max_width(800.)
    .style(iced::theme::Container::Custom(Box::new(PersonalCard::new(iced::Background::Color(Color::from_rgb(0.97, 0.97, 0.97)))))).into()
}

fn add_modal_view(state: &State) -> Option<Element<'static, Message>> {
    Some(
        Card::new(
            Text::new("New Item").size(20).font(Font{weight: font::Weight::Bold, ..BOOTSTRAP_FONT}),
            add_modal_body(&state),
        )
        .foot(
            Row::new()
                .spacing(10)
                .padding(5)
                .width(Length::Fill)
                .push(
                    Button::new(
                        Text::new("Cancel")
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .width(Length::Fill)
                    .on_press(Message::CloseAddModal),
                )
                .push(
                    Button::new(
                        Text::new("Ok").horizontal_alignment(Horizontal::Center),
                    )
                    .width(Length::Fill)
                    .on_press(Message::SaveAccount),
                ),
        )
        .max_width(500.0)
        //.width(Length::Shrink)
        .on_close(Message::CloseAddModal).into()
    )
}


fn add_modal_body(state: &State) -> Element<'static, Message> {
        Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(
                    TextInput::new("Host", &state.host_name)
                        .on_input(Message::HostChange)
                        .padding(10)
                )
                .push(
                    TextInput::new("Username", &state.username)
                        .on_input(Message::UsernameChange)
                        .padding(10)
                )
                .push(
                    TextInput::new("Password", &state.password)
                        .on_input(Message::PasswordChanged)
                        .padding(10)
                        .secure(true),
                )
                .push(
                    TextInput::new("Password Confirm", &state.confirm_password)
                        .on_input(Message::ConfirmPasswordChanged)
                        .padding(10)
                        .secure(true),
                )
        )
        .center_y()
        .center_x()
        .into()
}


fn edit_modal_view(state: &State) -> Option<Element<'static, Message>> {
    Some(
        Card::new(
            Text::new("Edit Account").size(20).font(Font{weight: font::Weight::Bold, ..BOOTSTRAP_FONT}),
            edit_modal_body(&state),
        )
        .foot(
            Row::new()
                .spacing(10)
                .padding(5)
                .width(Length::Fill)
                .push(
                    Button::new(
                        Text::new("Cancel")
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .width(Length::Fill)
                    .on_press(Message::CloseAddModal),
                )
                .push(
                    Button::new(
                        Text::new("Ok").horizontal_alignment(Horizontal::Center),
                    )
                    .width(Length::Fill)
                    .on_press(Message::SaveEdit),
                ),
        )
        .max_width(500.0)
        //.width(Length::Shrink)
        .on_close(Message::CloseAddModal).into()
    )
}

fn edit_modal_body(state: &State) -> Element<'static, Message> {
    Container::new(
        Column::new()
            .align_items(Alignment::Center)
            .max_width(600)
            .padding(20)
            .spacing(16)
            .push(
                TextInput::new("Host", &state.host_name)
                    .on_input(Message::HostChange)
                    .padding(10)
            )
            .push(
                TextInput::new("Username", &state.username)
                    .on_input(Message::UsernameChange)
                    .padding(10)
            )
            .push(
                TextInput::new("Password", &state.password)
                    .on_input(Message::PasswordChanged)
                    .padding(10)
                    .secure(true),
            )
            .push(
                TextInput::new("Password Confirm", &state.confirm_password)
                    .on_input(Message::ConfirmPasswordChanged)
                    .padding(10)
                    .secure(true),
            )
    )
    .center_y()
    .center_x()
    .into()
}
