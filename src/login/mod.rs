        use iced::{widget::{Button, Column, Container, TextInput}, Alignment, Length};

        use crate::Message;
        use iced::Element;
        use iced::widget::text;

        use crate::State;

            pub fn login(state: &State) -> Element<'static, Message> {
                Container::new(
                    Column::new()
                    .align_items(Alignment::Center)
                    .max_width(600)
                    .padding(20)
                    .spacing(16)
                    .push(
                        TextInput::new("Password", &state.password)
                            .on_input(Message::PasswordChanged)
                            .padding(10)
                            .size(32)
                            .secure(true),
                    )
                    .push(
                        TextInput::new("Password Confirm", &state.confirm_password)
                            .on_input(Message::ConfirmPasswordChanged)
                            .padding(10)
                            .size(32)
                            .secure(true),
                    )
                    .push(Button::new(
                        text("Confirm")
                    ).on_press(Message::SavePassword))
                ).width(Length::Fill)
                .height(Length::Fill)
                .center_y()
                .center_x()
                .into()
            
            }
