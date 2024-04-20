use std::{path::PathBuf, str::FromStr};

use enums::{step::step::Step, Modal};
use iced::{alignment::Horizontal, font, widget::{container, text}, window::Position, Application, Command, Element, Length, Size, Theme
};

use utils::{check_decryption_key, generate_key_pair, is_key_created, select_path};

use data_structure::account::account::{deserialize_accounts, serialize_accounts, Account};
use view::view_logic;

use crate::utils::utils::create_passvault_files;

mod data_structure;
mod enums;
mod view;
mod utils;
mod custom_widget;

fn main() -> iced::Result {
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
    #[allow(dead_code)]
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
    Start,
    AddAccount,
    PasswordCreated,
    Login,
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
    SelectPath,
    ConfirmKeyPath
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
    step: Step,
    aes_key: Vec<u8>,
    accounts: Vec<Account>,
    show_password: Option<usize>,
    modal: Option<Modal>,
    host_name: String,
    username: String,
    aes_key_path: Option<String>,
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
                    *self = match is_key_created() {
                        true => ModalExample::Loaded(State {
                            accounts: Vec::new(),
                            step: Step::PasswordCheck,
                            ..Default::default()
                        }),
                        false => ModalExample::Loaded(State::default()),
                    }
                }
            }
            ModalExample::Loaded(state) => match message {
                Message::PasswordChanged(password) => state.password = password,
                Message::ConfirmPasswordChanged(password) => state.confirm_password = password,
                Message::PasswordCreated => {
                    if state.password != state.confirm_password {
                        println!("Password not match!")
                    } else if state.password.len() < 8
                        || state.password.chars().all(char::is_alphanumeric)
                    {
                        println!("Weak password!")
                    } else {
                        state.step = Step::StoreSecretKey;
                        println!("Password create!")
                    }
                }, 
                Message::SelectPath => {
                    state.aes_key_path = select_path();
                    println!("{:?}", state.aes_key_path);
                },
                Message::ConfirmKeyPath => {
                    if state.aes_key_path.is_none() {
                        println!("Please select path");
                        return Command::none();
                    }

                    if !state.aes_key_path.is_none() && !PathBuf::from_str(state.aes_key_path.clone().unwrap().as_str()).unwrap().is_file() {
                        create_passvault_files();
                        state.aes_key = generate_key_pair(state.aes_key_path.clone().unwrap(), state.password.clone());
                    }

                    println!("{:?}", state.aes_key.clone());

                    println!("{}", state.aes_key_path.clone().unwrap());
                    
                    state.password.clear();
                    state.confirm_password.clear();

                    state.step = Step::PasswordManager;
                }
                Message::Login => {

                    if state.aes_key_path.is_none() || state.password.is_empty() {
                        println!("Please enter password and select path");
                        return Command::none();
                    }
                    
                    match check_decryption_key(state.aes_key_path.clone().unwrap(), state.password.clone()) {
                        Ok(key) => {
                            state.aes_key = key;

                            state.password.clear();
                            state.confirm_password.clear();

                            //TODO: decrypt the accounts and update the state
                            state.accounts = deserialize_accounts(&state.aes_key).unwrap();

                            state.step = Step::PasswordManager;
                        },
                        Err(_) => {
                            println!("Wrong password or path");
                            return Command::none();
                        }
                    };

                },
                Message::DeleteAccount(index) => {
                    state.accounts.remove(index);

                    println!("Serialization with key: {:?}", state.aes_key);

                    let _ = serialize_accounts(&state.accounts, &state.aes_key);
                    println!("Delete account at index: {}", index);
                },
                Message::Start => state.step = Step::PasswordCreation,
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

                    let _ = serialize_accounts(&state.accounts, &state.aes_key);

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
                    println!("Serialization with key: {:?}", state.aes_key);
                    let _ = serialize_accounts(&state.accounts, &state.aes_key);
                    
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
                },
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self {
            ModalExample::Loading => container(
                text("Loading...")
                    .horizontal_alignment(Horizontal::Center)
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


