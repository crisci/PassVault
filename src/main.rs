// [] start and create the (pk, sk)
// [] choose a device to store the sk
// [] create the password (symmetric key) and encrypt the pk and sk stored on the USB
// [] decrypt the sk and check if the format is correct (PEM)
use iced::{
    alignment::{self},
    widget::{container, text},
    Application, Command, Element, Length, Settings, Theme, font, 
};

use login::login;

mod login;
mod enums;


fn main() -> iced::Result {
    ModalExample::run(Settings::default())
}

#[derive(Clone, Debug)]
enum Message {
    PasswordChanged(String),
    ConfirmPasswordChanged(String),
    SavePassword,
    #[allow(dead_code)]
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
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
    confirm_password: String
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
        String::from("Modal example")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self {
            ModalExample::Loading => {
                if let Message::Loaded(_) = message {
                    *self = ModalExample::Loaded(State::default())
                }
            }
            ModalExample::Loaded(state) => {
                match message {
                    Message::PasswordChanged(password) => state.password = password,
                    Message::ConfirmPasswordChanged(password) => state.confirm_password = password,
                    Message::SavePassword => if state.password == state.confirm_password {println!("Equal!")} else {println!("Not Equal!")}
                    _ => {}
                }
            }
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
            ModalExample::Loaded(state) =>  login(state)
            
        }
    }

    fn theme(&self) -> Theme {
        match self {
            ModalExample::Loading => Theme::Light,
            ModalExample::Loaded(state) => state.theme.clone(),
        }
    }
}
