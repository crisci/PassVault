pub mod welcome_view {
    use iced::{theme, widget::{button, column, text, Container}, Element, Length};

    use crate::Message;

    pub fn welcome_view() -> Element<'static, Message> {
    Container::new(
        column![
            text("Welcome to PassVault!").size(50),
            text("The place where your password are secure.").size(26),
            button(text("Start").size(26))
                .on_press(Message::Start)
                .style(theme::Button::Primary)
                .padding(10)
        ]
        .align_items(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Shrink)
    .center_y()
    .center_x()
    .into()
}
}