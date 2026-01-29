use crate::keybind_manager;
use crate::keybind_manager::State;
use crate::keybind_manager::Message;

use crate::keycode;
use crate::asset;

use iced::Border;
use iced::widget::container::{
    Style,
};
use iced::window;
use iced::{
    Element,
    Task,
    Theme,
    widget::{
        pick_list,
        container, column, row, scrollable, stack, text
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
    match message{
        Message::ProfileSelected(profile) => {
            state.current_profile = Some(profile);
        }
    }

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
        .extend(
            keycode::KEYCODES.iter().flat_map(|category|
                category.iter().enumerate().map(|(i, code)|
                    if i == 0{
                        text!("{} - {}", code.1, code.2).size(28).into()
                    }
                    else {
                        row![
                        container(text(code.1)).style(container::bordered_box).align_x(iced::Left),
                        container(text(code.2)).style(container::bordered_box).align_x(iced::Right).width(iced::Fill),
                        ].width(iced::Fill).padding(5).into()
                    }
                )
            )
        ),
    )
    .width(iced::Fill)
    .height(iced::Fill)
    .into();

   

    column![
        header.height(iced::FillPortion(1)),
        stack!(background).height(iced::FillPortion(9)),
    ]
    .into()
}