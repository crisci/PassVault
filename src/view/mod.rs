use iced::{widget::{column, container, svg, Container}, Element, Length};

use crate::{enums::step::step::Step, Message, State};

use self::{password_check_view::password_check_view::password_check_view, password_creation_view::password_creation_view::password_creation_view, password_manager_view::password_manager_view::password_manager, sk_view::sk_view::sk_view, welcome_view::welcome_view::welcome_view};




pub mod sk_view;
pub mod password_manager_view;
pub mod add_modal_view;
pub mod edit_modal_view;
pub mod welcome_view;
pub mod password_creation_view;
pub mod password_check_view;

pub fn view_logic(state: &State) -> Element<'static, Message> {
    let content = match state.step {
        Step::StoreSecretKey => sk_view(),
        Step::Welcome => welcome_view(),
        Step::GetSecretKey => sk_view(),
        Step::PasswordManager => password_manager(state),
        Step::PasswordCreation => password_creation_view(state),
        Step::PasswordCheck => password_check_view(state),
    };
    Container::new(column![
        content
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

fn _load_image<'a>(image_name: String) -> Container<'a, Message> {
    let handle = svg::Handle::from_path(format!(
        "{}/resources/{}.svg",
        env!("CARGO_MANIFEST_DIR"),
        image_name
    ));

    let svg = svg(handle).width(Length::Fill).height(Length::Fill);

    container(svg).width(300).height(300).center_x().center_y()
}
