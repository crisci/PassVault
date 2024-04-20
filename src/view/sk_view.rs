pub mod sk_view {
    use iced::{alignment::Horizontal, theme, widget::{container, text, Button, Column, Container}, Alignment, Element, Font, Length};
    use iced_aw::BOOTSTRAP_FONT;

    use crate::{custom_widget::circle_button::circle_button::CircleButtonStyle, Message};


    pub fn sk_view() -> Element<'static, Message> {
        Container::new(
            Column::new()
                .align_items(Alignment::Center)
                .max_width(600)
                .padding(20)
                .spacing(16)
                .push(
                    text("Store your secret key")
                        .size(50)
                        .horizontal_alignment(Horizontal::Left),
                )
                .push(
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