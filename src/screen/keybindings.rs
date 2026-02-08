use iced::{
    Background,
    Element,
    Length,
    color,
    widget::{
        combo_box,
        button,
        column,
        container,
        pick_list,
        row,
        scrollable,
        space,
        text,
        text::{ 
            LineHeight,
        },
        text_input,
    }
};

use crate::{
    keycode,
    input,
    styling,
    asset::{
        Icon,
        icon,
    },
    keybind_manager::{
        self,
        State,
    },
    profile::{
        KeybindId,
        KeybindInput,
        KeybindSource,
        KeybindKeycode,
    },
    gui::{
        Prompt,
    },
    screen::{
        Screen,
    },
};

const TOOLBAR_HEIGHT: f32 = 40.0;
const TOOLBAR_ICON_SIZE: f32 = 28.0;

const KEYBIND_ROW_PADDING: f32 = 4.0;
const KEYBIND_HEIGHT: f32 = 50.0;
const KEYBIND_ICON_SIZE: f32 = 36.0;

#[derive(Debug, Clone)]
pub enum Message {
    ProfileSelected(String),
    Search(String),
    BeginNewKeybind,
    ModifyNewKeybind(Option<KeybindInput>, Option<KeybindSource>, Option<KeybindKeycode>),
    EndNewKeybind,
    CancelNewKeybind,
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

                // TODO: add unsaved changes prompt

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
            Message::BeginNewKeybind => {
                state.prompt = Some(Prompt::NewKeybind(Self::view_new_keybind));
            },
            Message::ModifyNewKeybind(input, source, keycode) => {
                if let Some(input) = input {
                    state.dummy_new_keybind.0 = input.clone();
                }
                if let Some(source) = source {
                    state.dummy_new_keybind.1 = source.clone();
                }
                if let Some(keycode) = keycode {
                    state.dummy_new_keybind.2 = keycode.clone();
                }
            }
            Message::EndNewKeybind => {
                state.prompt = None;
                state.selected_profile.as_ref().map(|profile_ref| {
                    let mut profile = profile_ref.borrow_mut();
                    state.dummy_new_keybind.3 = profile.new_id();
                    profile.add_keybind(state.dummy_new_keybind);
                });
            }
            Message::CancelNewKeybind => {
                state.prompt = None;
            }
            Message::DeleteKeybind(id) => {
                state.selected_profile.as_ref().map(|profile_ref| {
                    profile_ref.borrow_mut().remove_keybind(*id);
                });
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

        // TODO: make profile name italic when modified flag is set
        // TODO: add save button when modified flag is set
        // or activate/deactive save button depending on modified flag
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
        let keybinds = &profile.keybinds;
        if profile.keybinds.len() <= 0 {
            return (
                container(
                    text("no keybind data").center().size(48).style(text::warning)
                ).center(Length::Fill).into(), 
                header
            )
        }


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
                        .style(styling::button_transparent)
                    )
                    .align_right(Length::Fixed(KEYBIND_HEIGHT)),
                    space().width(10),
                ])
                .height(KEYBIND_HEIGHT)
                .width(iced::Fill)
                .style( if index % 2 == 1 {container::transparent} else {container::dark} )
                .padding(KEYBIND_ROW_PADDING)
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
                ).on_press(Message::BeginNewKeybind).style(button::success))
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

    pub fn view_new_keybind(_state: &State) -> Element<'static, keybind_manager::Message> {


        let content: Element<'_, Message> = container(container(column![
        row![
            // TODO: make these combo boxes (on change send Message::ModifyNewKeybind)
            text("input"),
            text("source"),
            text("key"),
        ].padding(KEYBIND_ROW_PADDING),
        row![
            button(icon(Icon::AddCircle).style(text::success).size(KEYBIND_ICON_SIZE))
                .on_press(Message::EndNewKeybind)
                .style(styling::button_transparent),
            button(icon(Icon::Cancel).style(text::danger).size(KEYBIND_ICON_SIZE))
                .on_press(Message::CancelNewKeybind)
                .style(styling::button_transparent),
        ].padding(KEYBIND_ROW_PADDING)
        ]).style(container::bordered_box)).center(Length::Fill).style(|_|container::background(Background::Color(color!(0,0,0,0.5))))
        .into();
        content.map(keybind_manager::Message::Keybindings)
    }
}