pub mod image_button {

    use iced::widget::{button, button::Appearance};
    use iced::{
        color, theme,
        widget::{column, container, svg, Container},
        Alignment, Border, Color, Length, Theme,
    };
    use std::default::Default;

    use crate::Message;


    #[derive(Default)]
    pub struct RadiusButton {
        label: String,
    }

    impl RadiusButton {
        pub fn new(label: String) -> Self {
            Self { label }
        }
    }
    impl button::StyleSheet for RadiusButton {
        type Style = Theme;

        fn active(&self, _: &Self::Style) -> Appearance {
            return match self.label.as_str() {
                "delete" => delete_theme(),
                "edit" => modify_theme(),
                "copy" => copy_theme(),
                "confirm" => save_theme(),
                _ => hide_theme(),
            };
        }
    }

    pub fn image_button<'a>(
        image_name: &'a str,
        message: Message,
    ) -> Container<'a, Message> {
        let handle = svg::Handle::from_path(format!(
            "{}/resources/{}.svg",
            env!("CARGO_MANIFEST_DIR"),
            image_name
        ));

        let svg =
            svg(handle)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                    color: Some(color!(0xffffff)),
                }));
        let c = column![
            container(
                button(container(svg))
                    .style(iced::theme::Button::Custom(Box::new(RadiusButton::new(
                        image_name.to_string()
                    ))))
                    .width(32)
                    .height(32)
                    .on_press(message)
            ),
        ]
        .align_items(Alignment::Center);

        container(c).padding(4).center_x()
    }

    fn copy_theme() -> Appearance {
        Appearance {
            border: Border::with_radius(100.0),
            background: Option::from(iced::Background::Color(Color::from(color!(0xb4b4b4)))),
            ..Appearance::default()
        }
    }

    fn delete_theme() -> Appearance {
        Appearance {
            border: Border::with_radius(100.0),
            background: Option::from(iced::Background::Color(Color::from(color!(0xF90851)))),
            ..Appearance::default()
        }
    }

    fn hide_theme() -> Appearance {
        Appearance {
            border: Border::with_radius(100.0),
            background: Option::from(iced::Background::Color(Color::from(color!(0xFF8328)))),
            ..Appearance::default()
        }
    }

    fn save_theme() -> Appearance {
        Appearance {
            border: Border::with_radius(100.0),
            background: Option::from(iced::Background::Color(Color::from(color!(0x64BF45)))),
            ..Appearance::default()
        }
    }

    fn modify_theme() -> Appearance {
        Appearance {
            border: Border::with_radius(100.0),
            background: Option::from(iced::Background::Color(Color::from(color!(0x364F6B)))),
            ..Appearance::default()
        }
    }
}
