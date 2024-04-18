pub mod card {
    use iced::widget::container;
    use iced::{Background, Border, Theme};

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
                border: Border::with_radius(10.0),
                background: self.background,
                ..Default::default()
            }
        }
    }
}