use std::default;

use crate::keybind_manager;
use crate::keybind_manager::State;
use crate::keybind_manager::Message;

use crate::input;
use crate::keycode;
use crate::screen;
use crate::asset;

use iced::Border;
use iced::widget::container::{
    Style,
};
use iced::window;
use iced::{
    window::Icon,
    Element,
    Task,
    Theme,
    widget::{
        pick_list,
        image,
        container, button, column, row, scrollable, stack, text
    }
};

pub fn run(state: State) -> iced::Result {
    let le_icon = window::icon::from_file_data(asset::ICON32, None).unwrap();
    iced::application( move || (state.clone(), iced::Task::none()), update, view)
        .window(window::Settings {
            icon: Some(le_icon),
            ..Default::default()
        })
        .title(keybind_manager::State::title)
        .theme(Theme::Dark)
        .antialiasing(true)
        .run()
}

fn update(state: &mut State, message: Message) -> Task<Message> {

    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {
    let header = container(
        row![
            text("Profile: ")
                .size(28)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center)
                .width(iced::Shrink).height(iced::Fill),
            container(
            pick_list(state.available_profiles.clone(), state.current_profile.clone(), keybind_manager::Message::ProfileSelected)
                .placeholder("none")
                .text_size(28)
            ).height(iced::Fill).align_y(iced::Center),
        ]
    )
        .width(iced::Fill)
        .padding(10)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            Style {
                border: Border { 
                    width: 2.0, 
                    color: palette.background.weak.color,
                    ..Border::default()
                 },
                ..Style::default()
            }
        })
    ;
    
    
    let background: Element<Message> = scrollable(
        column![]
        .extend(keycode::keyboard::ALL.iter().map(|code| text((*code).1).into()))
        ,
    )
    .width(iced::Fill)
    .height(iced::Fill)
    .into();


    let lalala =
        column![
            
            text(format!("Discovered Profiles: {}", state.available_profiles.len()))
            .size(40)
            .width(iced::Length::Fill)
            ,
            ].align_x(iced::alignment::Horizontal::Right)
            .extend(
            state.available_profiles.iter().map(|p| text(format!("lel {}", p.name.clone())).into())
            )
            ;
    

    column![
    header.height(iced::FillPortion(4)),
    stack!(background, lalala).height(iced::FillPortion(14)),
    ]
    .into()
}