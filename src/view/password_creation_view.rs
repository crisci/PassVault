pub mod password_creation_view {
    use iced::{alignment::Horizontal, widget::{container, text, Button, Column, Container, TextInput}, Alignment, Element, Length};

    use crate::{Message, State};
    pub fn password_creation_view(state: &State) -> Element<'static, Message> {
        Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(
                    text("Create your password")
                        .size(50)
                        .horizontal_alignment(Horizontal::Left),
                )
                .push(
                    TextInput::new("Password", &state.password)
                        .on_input(Message::PasswordChanged)
                        .padding(10)
                        .size(18)
                        .secure(true),
                )
                .push(
                    TextInput::new("Password Confirm", &state.confirm_password)
                        .on_input(Message::ConfirmPasswordChanged)
                        .padding(10)
                        .size(18)
                        .secure(true),
                )
                .push(
                    container(Button::new(text("Confirm")).on_press(Message::PasswordCreated))
                        .align_x(Horizontal::Right)
                        .width(Length::Fill),
                ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .center_x()
        .into()
    }
}