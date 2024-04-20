pub mod password_manager_view {
    use iced::{alignment, font, theme, widget::{column, container, row, scrollable, text, Button, Column, Container, Text}, Alignment, Color, Element, Font, Length, Padding};
    use iced_aw::{helpers::floating_element, floating_element::Anchor, modal, BOOTSTRAP_FONT};

    use crate::{custom_widget::{circle_button::circle_button::CircleButtonStyle, image_button::image_button::image_button}, data_structure::account::account::Account, enums::Modal, view::{add_modal_view::add_modal_view::add_modal_view, edit_modal_view::edit_modal_view::edit_modal_view}, Message, State};

    use crate::custom_widget::card::card::Card as PersonalCard;


    pub fn password_manager(state: &State) -> Element<'static, Message> {

        let mut account_list: Column<'static, Message> = Column::new();
        for (index, account) in state.accounts.iter().enumerate() {
            account_list = account_list.push(
                row![account_widget(account.clone(), index, &state)].padding(2.)
            );
        }
    
        let main_content = if state.accounts.len() > 4 {
            Container::new(scrollable( column![
                row![text("Your keys!").size(50)].align_items(iced::Alignment::Start),
                row![account_list].align_items(iced::Alignment::Start)
            ]
            .align_items(iced::Alignment::Center)
            .width(Length::Fill)))
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(10)
        } else {
            Container::new(column![
                row![text("Your keys!").size(50)].align_items(iced::Alignment::Start),
                row![account_list].align_items(iced::Alignment::Start)
            ]
            .align_items(iced::Alignment::Center)
            .width(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(10)
        };
    
        let modal_overlay: Option<Element<'static, Message>> =  match &state.modal {
    
            Some(m) => {
                match m {
                    Modal::ADD =>  add_modal_view(&state),
                    Modal::EDIT => edit_modal_view(&state),
                }
            },
            None => None,
        };
        
        let content = floating_element(
           main_content,
            Button::new(
                    container(
                        Text::new("New Item +")
                    .font(Font {
                        weight: font::Weight::Bold,
                        ..BOOTSTRAP_FONT
                    })
                    .size(18)
                    .line_height(1.0)
                    .shaping(text::Shaping::Advanced)
                    ).padding(8)
            )
            .on_press(Message::AddAccount)
            .padding(5)
            .style(theme::Button::Custom(Box::new(CircleButtonStyle::new(
                theme::Button::Primary,
            )))),
        )
        .anchor(Anchor::South)
        .offset(10.0)
        .hide(false);
    
        modal(Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        , modal_overlay).backdrop(Message::CloseAddModal).on_esc(Message::CloseAddModal).align_y(alignment::Vertical::Center).into()
    }

    fn account_widget(account: Account, index: usize, state: &State) -> Element<'static, Message> {
        let (switch_visibility, visibility_content) = if state.show_password == Some(index) {
           (image_button("visibility_off", Message::HidePassword), account.get_key())
        } else {
            (image_button("visibility_on", Message::ShowPassword(index)), account.get_username())
        };
        let delete_button = image_button("delete", Message::DeleteAccount(index));
        let edit_button = image_button("edit", Message::EditAccount(index));
        let copy_button = image_button("copy", Message::CopyPassword(index));
    
        let button_column = column![
                row![delete_button, edit_button, switch_visibility, copy_button].align_items(Alignment::Center)
            ].width(Length::FillPortion(1)).align_items(iced::Alignment::End);
    
        let account_column = column![
            row![text(account.get_host())
                .font(Font {
                    weight: font::Weight::Semibold,
                    ..BOOTSTRAP_FONT
                })
                .size(24)],
            row![text(visibility_content).size(22)]
        ].width(Length::FillPortion(2));
        Container::new(row![
            account_column,
            button_column
        
        ].align_items(Alignment::Center))
        .padding(Padding::new(20.))
        .width(600.)
        .max_width(800.)
        .style(iced::theme::Container::Custom(Box::new(PersonalCard::new(iced::Background::Color(Color::from_rgb(0.97, 0.97, 0.97)))))).into()
    }
    
}