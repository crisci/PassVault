pub mod password_creation_view {
    use iced::{
        alignment::Horizontal, theme, widget::{button, container, text, Column, Container, TextInput}, Alignment, Element, Font, Length
    };
    use iced_aw::BOOTSTRAP_FONT;

    use crate::{custom_widget::circle_button::circle_button::CircleButtonStyle, Message, State};
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
                    container(
                        button(text("Confirm").size(20).font(Font {
                            weight: iced::font::Weight::Semibold,
                            ..BOOTSTRAP_FONT
                        }))
                            .padding(8)
                            .style(theme::Button::Custom(Box::new(CircleButtonStyle::new(
                                theme::Button::Primary,
                            ))))
                            .on_press(Message::PasswordCreated),
                    )
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
