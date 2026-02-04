use iced::{
    Element,
    Length,
    widget::{
        button,
        column,
        container,
        pick_list,
        row,
        scrollable,
        space,
        text::LineHeight,
        text,
        text_input,
    }
};

use crate::asset::{
    Icon,
    icon,
};
use crate::keycode;
use crate::input;
use crate::keybind_manager::{
    State,
};
use crate::profile::{
    KeybindId,
};
use crate::screen::Screen;

const TOOLBAR_HEIGHT: f32 = 40.0;
const TOOLBAR_ICON_SIZE: f32 = 28.0;

const KEYBIND_HEIGHT: f32 = 50.0;
const KEYBIND_ICON_SIZE: f32 = 36.0;

#[derive(Debug, Clone)]
pub enum Message {
    ProfileSelected(String),
    Search(String),
    NewKeybind,
    DeleteKeybind(KeybindId)
}
#[derive(Debug, Clone, PartialEq)]
pub struct Keybindings {
}

impl Keybindings {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn update(state: &mut State, message: &Message) {
        match message {
            Message::ProfileSelected(name) => {
                let Some(profile_ref) = state
                    .available_profiles
                    .iter()
                    .find(|p| &p.borrow().name == name)
                    .cloned() else { return };

                let mut profile = profile_ref.borrow_mut();

                let res = profile.load_xml().err();
                match res {
                    Some(err) => println!("Error: {} | {}", err.0, err.1),
                    None => {
                        if let Err(err) = profile.write_xml(){
                            println!("Error writing xml '{}': {} | {}", profile.name, err.0, err.1);
                        }
                        state.selected_profile = Some(profile_ref.clone());
                        if state.screen == Screen::Landing {
                            state.screen = Screen::Keybindings(Keybindings::new());
                        }
                    },
                }
            },
            Message::Search(string) => {
                state.search_string = string.clone();
                println!("Search: {}", string)
            }
            Message::NewKeybind => {
                println!("new keybind!")
            },
            Message::DeleteKeybind(id) => {
                let Some(profile_ref) = state.selected_profile.clone() else { return };
                let mut profile = profile_ref.borrow_mut();
                if let Some(keybinds) = &mut profile.keybinds {
                    keybinds.retain(|keybind| keybind.3 != *id);
                }
            },
        }
    }

    pub fn view(&self, state: &State) -> (Element<'_, Message>, Element<'_, Message>) {

        
        let available_profile_names: Vec<String> = state.available_profiles
            .iter()
            .map(|p| p.borrow().name.clone())
            .collect();
        let selected_profile_name: Option<String> = state.selected_profile
            .as_ref()
            .map(|p| p.borrow().name.clone());

        let header: Element<'_, Message> = container(
        row![
            text("Profile: ")
                .size(28)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center)
                .width(iced::Shrink).height(iced::Fill)
            ,container(
                pick_list(
                    available_profile_names,
                    selected_profile_name,
                    Message::ProfileSelected
                )
                .placeholder("none")
                .text_size(28)
            )
            .height(iced::Fill)
            .align_y(iced::Center),
        ]
        ).align_left(iced::Fill).into();
        
        let Some(profile_ref) = &state.selected_profile else {
            return (container(text("No Profile Selected").center()).center(Length::Fill).into(), header)
        };
        let profile = profile_ref.borrow();
        
        let Some(keybinds) = &profile.keybinds else { 
            return (text("no keybind data").size(48).style(text::warning).into(), header)
        };

        let mut keybinds_sorted = keybinds.clone();
        keybinds_sorted.sort_by_key(|keybind|
            (keybind.1, keybind.0)
        );
        
        let keybind_list = column![].extend(
            keybinds_sorted.iter()
                .enumerate()
                .map(|(index, keybind)|{

                let (input, category, keycode) =
                    (input::from_index(keybind.0),
                    keycode::category_from_index(keybind.1),
                    keycode::keycode_from_indexes(keybind.1, keybind.2)
                );
                
                container(row![
                    space().width(10),
                    
                    container(column![
                            text(input).size(20),
                            text!("#{}", keybind.3).size(12).style(text::warning),
                    ]).clip(true).width(iced::FillPortion(20)),

                    space().width(10),
                    
                    container(column![
                        text(category.1).size(20),
                        text(category.2).size(12).style(text::primary),
                    ]).clip(true).width(iced::FillPortion(15)),
                    
                    space().width(10),
                    
                    container(column![
                        text(keycode.2).size(20),
                        text(keycode.1).size(12).style(text::primary),
                    ]).clip(true).width(iced::FillPortion(10)),
                    
                    container(
                        button(
                            icon(Icon::Cancel).size(KEYBIND_ICON_SIZE).center().style(text::danger)
                        )
                        .on_press(Message::DeleteKeybind(keybind.3))
                        .style(button::background)
                    )
                    .align_right(Length::Fixed(KEYBIND_HEIGHT)),
                    space().width(10),
                ])
                .height(KEYBIND_HEIGHT)
                .width(iced::Fill)
                .style( if index % 2 == 1 {container::transparent} else {container::dark} )
                .padding(4)
                .into()
            })
        );
    
        let toolbar = container(
            row![
                text_input("Search            Examples    \"weapon\"    \"i:frontend\"    \"s:mouse\"    \"k:enter\"", &state.search_string)
                .on_input(Message::Search)
                .line_height(LineHeight::Absolute(TOOLBAR_HEIGHT.into()))
                .size(24),
                container( button(
                    icon(Icon::Add).size(TOOLBAR_ICON_SIZE).center(),
                ).on_press(Message::NewKeybind).style(button::success))
                .height(Length::Fill)
                .width(Length::Shrink)
                .style(container::bordered_box),
            ],
        )
        .align_right(Length::Fill)
        .height(Length::Fixed(TOOLBAR_HEIGHT));
        
        return (
            column!(
                toolbar,
                scrollable(keybind_list)
            ).into(),
            header
        )
    }
}