use iced::{
    Element,
    FillPortion,
    widget::{
        pick_list, column, container, row, scrollable, space, text
    }
};

use crate::keycode;
use crate::input;
use crate::keybind_manager::{
    State,
};
use crate::profile::Profile;
use crate::profile;
use crate::screen::Screen;

#[derive(Debug, Clone, PartialEq)]
pub struct Keybindings {

}

#[derive(Debug, Clone)]
pub enum Message {
    ProfileSelected(Profile),
}

impl Keybindings {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(state: &mut State, message: &Message) {
        match message {
            Message::ProfileSelected(profile) => {
                let mut profile = profile.clone();
                let res = profile.load_xml().err();
                match res {
                    Some(err) => println!("Error: {} | {}", err.0, err.1),
                    None => {
                        if let Err(err) = profile.write_xml(){
                            println!("Error writing xml '{}': {} | {}", profile.name, err.0, err.1);
                        }
                        state.current_profile = Some(profile);
                        if state.screen == Screen::Landing {
                            state.screen = Screen::Keybindings(Keybindings::new());
                        }
                    },
                }
            },
        }
    }

    pub fn view(&self, state: &State) -> (Element<'_, Message>, Element<'_, Message>) {

        let header: Element<'_, Message> = container(
        row![
            text("Profile: ")
                .size(28)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center)
                .width(iced::Shrink).height(iced::Fill)
            ,container(
            pick_list(state.available_profiles.clone(), state.current_profile.clone(), Message::ProfileSelected)
                .placeholder("none")
                .text_size(28)
            )
            .height(iced::Fill)
            .align_y(iced::Center),
        ]
        ).align_left(iced::Fill).into();



        let mut background_column = column![];
        
        let Some(profile) = &state.current_profile else { return (background_column.into(), header); };
        let Some(keybinds) = &profile.keybinds else { return (background_column.into(), header); };

        let mut keybinds_sorted = keybinds.clone();
        keybinds_sorted.sort_by_key(|keybind|
            (keybind.1, keybind.0)
        );

        background_column = background_column.extend(
            keybinds_sorted.iter()
                .enumerate()
                .map(|(index, keybind)|{

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
    
        return (scrollable(background_column).into(), header)
    }
}