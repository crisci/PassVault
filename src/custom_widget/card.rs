pub mod card {
    use iced::widget::container;
    use iced::{Background, Border, Color, Shadow, Theme};

    #[derive(Debug, Clone, Copy, Default)]
    pub struct Card {
        background: Option<Background>
    }


    impl Card {
        pub fn new(background: Background) -> Self { Card { background: Some(background) } }
    }


    impl container::StyleSheet for Card {
        type Style = Theme;

        fn appearance(&self, _: &Self::Style) -> container::Appearance {
            container::Appearance {
                border: Border::with_radius(25.0),
                shadow: Shadow { 
                    offset: iced::Vector { x: -2. , y: 2. },
                    color: Color::BLACK,
                    blur_radius: 5.
                },
                background: self.background,
                ..Default::default()
            }
        }
    }
}