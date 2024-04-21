use std::{path::PathBuf, str::FromStr};

use custom_widget::toast::toast::{Status, Toast};
use enums::{step::step::Step, Modal};
use iced::{alignment::Horizontal, font, widget::{container, text}, window::Position, Application, Command, Element, Length, Size, Theme
};

use utils::{check_decryption_key, copy_to_clipboard, generate_key_pair, generate_password, is_key_created, select_path};

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
    ConfirmKeyPath,
    CopySuccess,
    AddToast(String, Status),
    CloseToast(usize),
    GeneratePassword
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
    edit_index: Option<usize>,
    error: Option<String>,
    toasts: Vec<Toast>,
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
                    if state.password.is_empty() {
                        state.error = Some("Please insert password".to_string());
                        return Command::none();
                    } else  if state.confirm_password.is_empty() {
                        state.error = Some("Please insert confirm password".to_string());
                        return Command::none();  
                    } else if state.password != state.confirm_password {
                        state.error = Some("Password not match".to_string());
                        state.password.clear();
                        state.confirm_password.clear();
                    } else if state.password.len() < 8
                        || state.password.chars().all(char::is_alphanumeric)
                    {
                        state.error = Some("Weak password".to_string());
                        state.password.clear();
                        state.confirm_password.clear();
                    } else {
                        state.error = None;
                        state.step = Step::StoreSecretKey;
                    }
                }, 
                Message::SelectPath => {
                    state.aes_key_path = select_path();
                    if state.aes_key_path.is_some() {
                        state.error = None;
                    }
                },
                Message::ConfirmKeyPath => {
                    if state.aes_key_path.is_none() {
                        state.error = Some("Please select path".to_string());
                        return Command::none();
                    }

                    if !state.aes_key_path.is_none() && !PathBuf::from_str(state.aes_key_path.clone().unwrap().as_str()).unwrap().is_file() {
                        create_passvault_files();
                        match generate_key_pair(state.aes_key_path.clone().unwrap(), state.password.clone()) {
                            Ok(key) => {
                                state.aes_key = key;
                                state.password.clear();
                                state.confirm_password.clear();
                                state.error = None;
                                state.step = Step::PasswordManager;
                            },
                            Err(_) => {
                                state.error = Some("Error creating key".to_string());
                                return Command::none();
                            }
                        }
                    }
                }
                Message::Login => {

                    if state.aes_key_path.is_none() || state.password.is_empty() {
                        state.error = Some("Please select path and insert password".to_string());
                        return Command::none();
                    }
                    
                    match check_decryption_key(state.aes_key_path.clone().unwrap(), state.password.clone()) {
                        Ok(key) => {
                            state.error = None;
                            state.aes_key = key;

                            state.password.clear();
                            state.confirm_password.clear();

                            state.accounts = deserialize_accounts(&state.aes_key).unwrap();

                            state.step = Step::PasswordManager;
                        },
                        Err(_) => {
                            state.error = Some("Wrong password or path".to_string());
                            return Command::none();
                        }
                    };

                },
                Message::DeleteAccount(index) => {
                    state.accounts.remove(index);

                    let _ = serialize_accounts(&state.accounts, &state.aes_key);
                },
                Message::Start => state.step = Step::PasswordCreation,
                Message::ShowPassword(index) => {
                    state.show_password = Some(index);
                },
                Message::HidePassword => {
                    state.show_password = None;
                },
                Message::CopyPassword(index) => {
                    let account = state.accounts.get(index);
                    match account {
                        Some(a) => {
                            let key = a.get_key().clone();
                            return Command::perform(copy_to_clipboard(key.clone()), |_| Message::CopySuccess);
                        },
                        None => {
                            return Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::AddToast("Error while copying to clipboard".into(), Status::Danger));
                        }
                    }
                },
                Message::CopySuccess => {
                    return Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::AddToast("Copied to clipboard".into(), Status::Success));
                },
                Message::AddToast(title, level) => {
                    let toast = Toast {
                        title,
                        status: level,
                        ..Default::default()
                    };
                    if !state.toasts.contains(&toast) { state.toasts.push(toast) }
                },
                Message::CloseToast(index) => {
                    state.toasts.remove(index);
                },
                Message::EditAccount(index) => {

                    let account_to_edit = state.accounts.get(index).unwrap();

                    state.host_name = account_to_edit.get_host().clone();
                    state.username = account_to_edit.get_username().clone();
                    state.password = account_to_edit.get_key().clone();
                    state.edit_index = Some(index);

                    state.modal = Some(Modal::EDIT);
                },
                Message::SaveEdit => { 

                    if state.password.is_empty() {
                        state.error = Some("Please insert password".to_string());
                        return Command::none();
                    } else  if state.confirm_password.is_empty() {
                        state.error = Some("Please insert confirm password".to_string());
                        return Command::none();  
                    } else if state.password.len() < 8 && state.password.chars().all(char::is_alphanumeric) {
                        state.error = Some("Weak password".to_string());
                        return Command::none();
                    } else if state.password != state.confirm_password {
                        state.error = Some("Password not match".to_string());
                        return Command::none();
                    }

                    state.error = None;

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
                },
                Message::SaveAccount => {
                    
                    if state.password.is_empty() {
                        state.error = Some("Please insert password".to_string());
                        return Command::none();
                    } else  if state.confirm_password.is_empty() {
                        state.error = Some("Please insert confirm password".to_string());
                        return Command::none();  
                    } else if state.password.len() < 8 && state.password.chars().all(char::is_alphanumeric) {
                        state.error = Some("Weak password".to_string());
                        return Command::none();
                    } else if state.password != state.confirm_password {
                        state.error = Some("Password not match".to_string());
                        return Command::none();
                    }

                    state.error = None;
                    let new_account = Account::new(state.host_name.clone(), state.username.clone(), state.password.clone());

                    state.password.clear();
                    state.host_name.clear();
                    state.username.clear();
                    state.confirm_password.clear();
                    if state.accounts.contains(&new_account) {
                        state.modal = None;
                        return Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::AddToast("Account already exist".into(), Status::Warning))
                    }
                    state.accounts.push(new_account);
                    state.modal = None;
                    match serialize_accounts(&state.accounts, &state.aes_key) {
                        Ok(_) => {
                            return Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::AddToast("Account correctly saved".into(), Status::Success))
                        },
                        Err(_) => {
                            return Command::perform(tokio::time::sleep(std::time::Duration::from_millis(0)), |_| Message::AddToast("Ops, something went wrong!".into(), Status::Success))
                        }
                    }
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
                Message::GeneratePassword => {
                    let rand_password = generate_password();
                    state.password = rand_password.clone();
                    state.confirm_password = rand_password.clone();
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


