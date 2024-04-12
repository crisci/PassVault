use iced::{
    alignment::Horizontal,
    color, theme,
    widget::{container, svg, Button, Column, Container, Row, TextInput},
    Alignment, Length, Renderer,
};

use crate::Message;
use iced::widget::text;
use iced::Element;

use crate::State;

pub fn login(state: &State) -> Element<'static, Message> {
    Container::new(
        Column::new()
            .align_items(Alignment::Center)
            .max_width(600)
            .padding(20)
            .spacing(16)
            .push(
                text("Create your password")
                    .size(50)
                    .horizontal_alignment(Horizontal::Left),
            )
            .push(
                TextInput::new("Password", &state.password)
                    .on_input(Message::PasswordChanged)
                    .padding(10)
                    .size(18)
                    .secure(true),
            )
            .push(
                TextInput::new("Password Confirm", &state.confirm_password)
                    .on_input(Message::ConfirmPasswordChanged)
                    .padding(10)
                    .size(18)
                    .secure(true),
            )
            .push(
                container(Button::new(text("Confirm")).on_press(Message::SavePassword))
                    .align_x(Horizontal::Right)
                    .width(Length::Fill),
            ),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .center_x()
    .into()
}

fn load_image<'a>(image_name: String) -> Container<'a, Message> {
    let handle = svg::Handle::from_path(format!(
        "{}/resources/{}.svg",
        env!("CARGO_MANIFEST_DIR"),
        image_name
    ));

    let svg = svg(handle).width(Length::Fill).height(Length::Fill);

    container(svg).width(300).height(300).center_x().center_y()
}
