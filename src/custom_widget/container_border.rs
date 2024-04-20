use iced::{alignment::Horizontal, widget::{container, Text}, Background, Color, Renderer};

use crate::Message;

use self::container_border::{ContainerBordered, FolderPathContainer};

pub mod container_border {
    use iced::border::Radius;
    use iced::widget::container;
    use iced::{Border, Color, Shadow, Theme};

    #[derive(Debug, Clone, Default)]
    pub struct ContainerBordered;


    impl ContainerBordered {
        pub fn new() -> Self { Self }
    }


    impl container::StyleSheet for ContainerBordered {
        type Style = Theme;

        fn appearance(&self, _: &Self::Style) -> container::Appearance {
            container::Appearance {
                border: Border {
                    color: Color::BLACK,
                    radius: Radius::from(25.),
                    width: 2.
                },
                shadow: Shadow { 
                    offset: iced::Vector { x: -2. , y: 2. },
                    color: Color::BLACK,
                    blur_radius: 5.
                },
                ..Default::default()
            }
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct FolderPathContainer;


    impl FolderPathContainer {
        pub fn new() -> Self { Self }
    }


    impl container::StyleSheet for FolderPathContainer {
        type Style = Theme;

        fn appearance(&self, _: &Self::Style) -> container::Appearance {
            container::Appearance {
                border: Border {
                    color: Color::from_linear_rgba(0., 0., 0., 0.8),
                    radius: Radius::from(10.),
                    width: 2.
                },
                ..Default::default()
            }
        }
    }
}


pub fn rounded_container<'a>(text: String) -> iced::widget::Container<'a, Message> {
    container(Text::new(text.clone()).horizontal_alignment(Horizontal::Left))
        .style(iced::theme::Container::Custom(Box::new(FolderPathContainer::new())))
        .padding(6.0)
}