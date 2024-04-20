pub mod welcome_view {
    use iced::{
        theme,
        widget::{button, column, container, row, text, Button, Container},
        Element, Font, Length, Padding,
    };
    use iced_aw::BOOTSTRAP_FONT;

    use crate::{custom_widget::circle_button::circle_button::CircleButtonStyle, Message};

    pub fn welcome_view() -> Element<'static, Message> {
        let start_button: Button<'static, Message> = button(text("Start").size(30).font(Font {
            weight: iced::font::Weight::Semibold,
            ..BOOTSTRAP_FONT
        }))
        .on_press(Message::Start)
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
        Container::new(
            column![
                text("Welcome to PassVault!").size(50),
                text("The place where your password are secure.").size(26),
                row![start_button].align_items(iced::Alignment::Center)
            ]
            .spacing(20)
            .align_items(iced::Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Shrink)
        .center_y()
        .center_x()
        .into()
    }
}
