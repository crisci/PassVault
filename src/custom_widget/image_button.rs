pub mod image_button {

    use iced::application;
    use iced::widget::{button, button::Appearance};
    use iced::{
        color, theme,
        widget::{column, container, svg, text, Container},
        Alignment, Border, Color, Length, Renderer, Theme,
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
                "Hide" => hide_theme(),
                "Modify" => modify_theme(),
                "Copy" => copy_theme(),
                "Confirm" => save_theme(),
                _ => delete_theme(),
            };
        }
    }

    pub fn image_button<'a>(
        image_name: &'a str,
        description: &'static str,
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
        let (h, w, p) = match description {
            "Screenshot" => (80, 55, 0),
            "Folder" => (38, 32, 1),
            _ => (55, 55, 5),
        };
        let c = column![
            if description != "Folder" {
                text(description)
            } else {
                text("")
            },
            container(
                button(container(svg).padding(p))
                    .style(iced::theme::Button::Custom(Box::new(RadiusButton::new(
                        description.to_string()
                    ))))
                    .on_press(message)
                    .width(h)
                    .height(w)
            ),
        ]
        .align_items(Alignment::Center);

        container(c).center_x()
    }

    fn copy_theme() -> Appearance {
        Appearance {
            border: Border::with_radius(100.0),
            background: Option::from(iced::Background::Color(Color::from(color!(0x364F6B)))),
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
            background: Option::from(iced::Background::Color(Color::from(color!(0xF90851)))),
            ..Appearance::default()
        }
    }
}
