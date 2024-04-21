pub mod error_text {
    use iced::{theme::Text as TextTheme, widget::Text, Color, Length};

    pub fn error_text(text: &String) -> Text<'static> {
        Text::new(text.clone())
            .size(20)
            .style(TextTheme::Color(Color::from_rgb(1.0, 0., 0.)))
            .width(Length::Fill)
    }
}