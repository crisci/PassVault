pub mod add_modal_view {
    use iced::{alignment::Horizontal, font, widget::{Button, Column, Container, Row, Text, TextInput}, Alignment, Element, Font, Length};
    use iced_aw::{Card, BOOTSTRAP_FONT};

    use crate::{custom_widget::error_text::error_text::error_text, Message, State};

    pub fn add_modal_view(state: &State) -> Option<Element<'static, Message>> {
        
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
                            Text::new("Generate")
                                .horizontal_alignment(Horizontal::Center),
                        )
                        .width(Length::Fill)
                        .on_press(Message::GeneratePassword),
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

        let error_text = match &state.error {
            Some(error) => {
                Some(error_text(error).horizontal_alignment(Horizontal::Center))
            },
            None => {
                None
            }
        };

        Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push_maybe(error_text)
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
}