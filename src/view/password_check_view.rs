pub mod password_check_view {
    use iced::{alignment::Horizontal, theme, widget::{container, text, Button, Column, Container, TextInput}, Alignment, Element, Font, Length};
    use iced_aw::BOOTSTRAP_FONT;

    use crate::{custom_widget::circle_button::circle_button::CircleButtonStyle, Message, State};
    pub fn password_check_view(state: &State) -> Element<'static, Message> {
        Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(
                    text("Insert your password and upload the key file")
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
                    container(Button::new(text("Confirm")).on_press(Message::Login))
                        .align_x(Horizontal::Right)
                        .width(Length::Fill),
                ).push(
                    container(Button::new(text("Select the path").size(26).font(
                        Font { weight: iced::font::Weight::Semibold, ..BOOTSTRAP_FONT } 
                    )
                ).style(theme::Button::Custom(Box::new(CircleButtonStyle::new(
                    theme::Button::Primary,
                )))).padding(16.)
                    .on_press(Message::SelectPath))
                        .align_x(Horizontal::Center)
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