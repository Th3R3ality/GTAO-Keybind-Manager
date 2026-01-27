use std::path::PathBuf;

use crate::keybind_manager;
use crate::keybind_manager::State;
use crate::keybind_manager::Message;

use crate::input;
use crate::keycode;

use iced::{
    Element,
    Task,
    Theme,
    widget::{
        button, column, row, scrollable, stack, text
    }
};

pub fn run(state: State) -> iced::Result {
    iced::application( move || (state.clone(), iced::Task::none()), update, view)
        .title(keybind_manager::State::title)
        .theme(Theme::Dark)
        .antialiasing(true)
        .run()
}

fn update(state: &mut State, message: Message) -> Task<Message> {

    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {


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
    
    let main_content =
    column![
            text("Counter Example")
                .size(32)
                .width(iced::Length::Fill)
                .align_x(iced::alignment::Horizontal::Center),

            row![
                button("-10").on_press(Message::Decrement),
                button("-1") .on_press(Message::Decrement),
                button("+1") .on_press(Message::Increment),
                button("+10").on_press(Message::Increment),
            ]
            .spacing(12)
            .align_y(iced::Alignment::Center),

            text("lalala"),
            button("Reset")
                .on_press(Message::Reset)
                .padding([12, 32]),
        ]
        .padding(40)
        .spacing(32)
        .align_x(iced::Alignment::Center);

    stack!(background, main_content, lalala).into()
}