pub mod circle_button {
    use iced::{theme, widget::button, Theme};
    use iced::widget::button::Appearance;
    
    pub struct CircleButtonStyle {
        theme: theme::Button,
    }
    
    impl CircleButtonStyle {
        pub fn new(theme: theme::Button) -> Self {
            Self { theme }
        }
    }
    
    impl button::StyleSheet for CircleButtonStyle {
        type Style = Theme;
    
        fn active(&self, style: &Self::Style) -> Appearance {
            let mut appearance = style.active(&self.theme);
            appearance.border.radius = 25.0.into();
    
            appearance
        }
    }
}