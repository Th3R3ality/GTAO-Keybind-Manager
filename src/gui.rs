use crate::keybind_manager;
use crate::keybind_manager::State;
use crate::keybind_manager::Message;

use crate::input;
use crate::keycode;
use crate::asset;

use iced::Border;
use iced::Length::FillPortion;
use iced::padding;
use iced::widget::container::{
    Style,
};
use iced::widget::space;
use iced::window;
use iced::{
    Element,
    Task,
    Theme,
    widget::{
        pick_list,
        container, column, row, scrollable, text
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
        .theme(Theme::TokyoNight)
        .antialiasing(true)
        .run()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message{
        Message::ProfileSelected(mut profile) => {
            let res = profile.load_xml().err();
            match res {
                Some(err) => println!("Error: {} | {}", err.0, err.1),
                None => {
                    if let Err(err) = profile.write_xml(){
                        println!("Error writing xml '{}': {} | {}", profile.name, err.0, err.1);
                    }
                    state.current_profile = Some(profile);
                },
            }
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
            )
            .height(iced::Fill)
            .align_y(iced::Center),
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
                radius: 0.into(),
                },
            ..Style::default()
        }
    });

    let mut background_column: iced::widget::Column<'_, Message> = column![];
    if let Some(profile) = &state.current_profile {
        if let Some(keybinds) = &profile.keybinds {
            let mut keybinds_sorted = keybinds.clone();
            keybinds_sorted.sort_by_key(|keybind|
                (keybind.1, keybind.0)
            );
            background_column = background_column.extend(
                keybinds_sorted.iter().enumerate().map(|(index, keybind)|{

                    let (input, category, keycode) =
                        (input::from_index(keybind.0),
                        keycode::category_from_index(keybind.1),
                        keycode::keycode_from_indexes(keybind.1, keybind.2)
                    );
                    
                    container(row![
                        space().width(FillPortion(1)),
                        container(column![
                                text(input).size(20),
                                text!("#{}", keybind.3).size(12).style(text::warning),
                        ]).width(iced::FillPortion(20))
                        ,container(column![
                            text(category.1).size(20),
                            text(category.2).size(12).style(text::primary),
                        ]).width(iced::FillPortion(15))
                        ,container(column![
                            text(keycode.2).size(20),
                            text(keycode.1).size(12).style(text::primary),
                        ]).width(iced::FillPortion(10))
                        ,
                    ])
                    .width(iced::Fill)
                    .style( if index % 2 == 0 {container::transparent} else {container::dark} )
                    //.height(iced::Length::Fixed(75.0))
                    .padding(4)
                    .into()
                })
            );
        }
    }
    let content: Element<Message> = scrollable(
        column![background_column]
    )
    .width(iced::Fill)
    .height(iced::Fill)
    .into();

    column![
        container(header).height(iced::FillPortion(9)),
        container(content).height(iced::FillPortion(90)),
    ]
    .into()
}