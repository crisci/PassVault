pub mod password_check_view {
    use iced::{
        alignment::Horizontal,
        theme,
        widget::{button, row, text, Button, Column, Container, TextInput},
        Alignment, Element, Font, Length, Padding,
    };
    use iced_aw::BOOTSTRAP_FONT;

    use crate::{
        custom_widget::{
            circle_button::circle_button::CircleButtonStyle, container_border::rounded_container,
            error_text::error_text::error_text, image_button::image_button::image_button,
        },
        Message, State,
    };
    pub fn password_check_view(state: &State) -> Element<'static, Message> {
        let folder_button =
            image_button("folder", Message::SelectPath).width(Length::FillPortion(1));
        let path_container = rounded_container(match &state.aes_key_path {
            Some(path) => path.to_string(),
            None => "Select the path...".to_string(),
        })
        .width(Length::FillPortion(6));

        let confirm_button: Button<'static, Message> =
            button(text("Confirm").size(30).font(Font {
                weight: iced::font::Weight::Semibold,
                ..BOOTSTRAP_FONT
            }))
            .on_press(Message::Login)
            .style(theme::Button::Primary)
            .padding(Padding {
                bottom: 4.,
                top: 4.,
                left: 28.,
                right: 28.,
            })
            .style(theme::Button::Custom(Box::new(CircleButtonStyle::new(
                theme::Button::Primary,
            ))));

        let error_text = match &state.error {
            Some(error) => Some(error_text(error).horizontal_alignment(Horizontal::Center)),
            None => None,
        };

        Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(
                    text("Insert your password and upload the key file")
                        .font(Font {
                            weight: iced::font::Weight::Semibold,
                            ..BOOTSTRAP_FONT
                        })
                        .size(50)
                        .horizontal_alignment(Horizontal::Left),
                )
                .push_maybe(error_text)
                .push(
                    TextInput::new("Password", &state.password)
                        .on_input(Message::PasswordChanged)
                        .padding(10)
                        .size(18)
                        .secure(true),
                )
                .push(row![path_container, folder_button].align_items(Alignment::End))
                .push(confirm_button),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .center_x()
        .into()
    }
}
