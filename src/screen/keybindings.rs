use iced::{
    Element,
    Length,
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
    },
    gui::{
        Prompt,
    },
};

const HEADER_TEXT_SIZE: f32 = 28.0;

const TOOLBAR_HEIGHT: f32 = 40.0;
const TOOLBAR_ICON_SIZE: f32 = 28.0;

const KEYBIND_TEXT_SIZE: f32 = 20.0;
const KEYBIND_SUBTEXT_SIZE: f32 = 12.0;
const KEYBIND_ROW_PADDING: f32 = 4.0;
const KEYBIND_HEIGHT: f32 = 50.0;
const KEYBIND_ICON_SIZE: f32 = 36.0;


#[derive(Debug, Clone)]
pub enum Message {
    ProfileSelected(String),
    SaveChangesProfileSelected,
    IgnoreChangesProfileSelected,
    CancelProfileSelected,
    SaveSelectedProfile,
    Search(String),
    BeginNewKeybind,
    SelectNewKeybindInput(String),
    SelectNewKeybindSource(String),
    SelectNewKeybindKeycode(String),
    SaveNewKeybind,
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

    fn save_selected_profile(state: &State) {
        state.selected_profile.as_ref().map(|profile_ref| {
            let mut profile = profile_ref.borrow_mut();
            if let Err(err) = profile.write_xml(){
                println!("Error writing xml '{}': {} | {}", profile.name, err.0, err.1);
            }
        });
    }
        
    fn load_selected_profile(state: &mut State) {
        let Some(selected_profile_name) = state.selected_profile_name.as_ref() else { return };

        let Some(profile_ref) = state
            .available_profiles
            .iter()
            .find(|p| &p.borrow().name == selected_profile_name)
            .cloned() else { return };

        let mut profile = profile_ref.borrow_mut();

        let res = profile.load_xml().err();
        match res {
            Some(err) => println!("Error reading xml: {} | {}", err.0, err.1),
            None => state.selected_profile = Some(profile_ref.clone()),
        }
    }

    pub fn update(state: &mut State, message: &Message) {
        match message {
            Message::ProfileSelected(name) => {
                state.selected_profile_name = Some(name.clone());

                if let Some(profile) = state.selected_profile.as_ref() {
                    if profile.borrow().modified {
                        state.prompt = Some(Prompt::UnsavedChanges(Self::view_prompt_unsaved_changes));
                        return
                    }
                }
                Self::load_selected_profile(state)
            },
            Message::SaveChangesProfileSelected => {
                state.prompt = None;
                Self::save_selected_profile(state);
                Self::load_selected_profile(state)
            },
            Message::IgnoreChangesProfileSelected => {
                state.prompt = None;
                Self::load_selected_profile(state)
            },
            Message::CancelProfileSelected => {
                state.prompt = None
            }
            Message::SaveSelectedProfile => {
                Self::save_selected_profile(state);
            },
            Message::Search(string) => {
                state.search_string = string.clone();
                println!("Search: {}", string)
            }
            Message::BeginNewKeybind => {
                state.prompt = Some(Prompt::NewKeybind(Self::view_prompt_new_keybind));
            },
            Message::SelectNewKeybindInput(input_name) => {
                state.selected_input_new_keybind = Some(input_name.clone());
                state.dummy_new_keybind.0 = input::get_index(input_name)
                    .unwrap_or_default();
            },
            Message::SelectNewKeybindSource(source_name) => {
                state.selected_source_new_keybind = Some(source_name.clone());
                state.dummy_new_keybind.1 = keycode::get_category_index(source_name)
                    .unwrap_or_default();

                let keycode_list: Vec<String> = keycode::KEYCODES[state.dummy_new_keybind.1]
                    .iter()
                    .enumerate()
                    .filter_map(|(index, elem)| if index > 0 { Some(elem.0.to_string()) } else { None } )
                    .collect();
                state.keycode_list_state_new_keybind = combo_box::State::new(keycode_list);
            },
            Message::SelectNewKeybindKeycode(keycode_name) => {
                state.selected_keycode_new_keybind = Some(keycode_name.clone());
                state.dummy_new_keybind.2 = keycode::get_keycode_index_with_category_index(state.dummy_new_keybind.1, keycode_name)
                    .unwrap_or_default();
            },
            Message::SaveNewKeybind => {
                state.prompt = None;
                if state.dummy_new_keybind.0 == 0 { return };
                // keycode/2 will always be 0 if source/1 has not been set
                if state.dummy_new_keybind.2 == 0 { return };

                
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
            .map(|profile_ref| profile_ref.borrow().name.clone())
            .collect();
        let selected_profile_name: Option<String> = state.selected_profile
            .as_ref()
            .map(|profile_ref| {
                let profile = profile_ref.borrow();
                let modified_tag = if profile.modified {"*"} else {""};
                format!("{}{}{}", modified_tag, profile.name.clone(), modified_tag)
            });

        let profile_modified_flag: bool = match &state.selected_profile {
            Some(profile_ref) => if profile_ref.borrow().modified { true } else { false },
            _ => false,
        };

        let header: Element<'_, Message> = container(
        row![
            text("Profile:")
            .size(HEADER_TEXT_SIZE)
            .align_y(iced::Center)
            .align_x(iced::Center)
            .width(iced::Shrink).height(iced::Fill),
            space().width(Length::Fixed(HEADER_TEXT_SIZE / 2.0)),
            container(
                pick_list(
                    available_profile_names,
                    selected_profile_name,
                    Message::ProfileSelected
                )
                .placeholder("none")
                .text_size(HEADER_TEXT_SIZE)
            )
            .height(iced::Fill)
            .align_y(iced::Center),
            space().width(Length::Fixed(HEADER_TEXT_SIZE / 4.0)),
            container(
                button(
                    icon( if profile_modified_flag { Icon::SaveAs } else { Icon::Save }).size(HEADER_TEXT_SIZE).center()
                    .style( if profile_modified_flag { text::warning } else { text::default })
                )
                .on_press_maybe( if profile_modified_flag { Some(Message::SaveSelectedProfile) } else { None }).style(styling::button_transparent)
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
                            text(input).size(KEYBIND_TEXT_SIZE),
                            text!("#{}", keybind.3).size(KEYBIND_SUBTEXT_SIZE).style(text::warning),
                    ]).clip(true).width(iced::FillPortion(20)),

                    space().width(10),
                    
                    container(column![
                        text(category.1).size(KEYBIND_TEXT_SIZE),
                        text(category.2).size(KEYBIND_SUBTEXT_SIZE).style(text::primary),
                    ]).clip(true).width(iced::FillPortion(15)),
                    
                    space().width(10),
                    
                    container(column![
                        text(keycode.2).size(KEYBIND_TEXT_SIZE),
                        text(keycode.1).size(KEYBIND_SUBTEXT_SIZE).style(text::primary),
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

    pub fn view_prompt_unsaved_changes(_state: &State) -> Element<'_, keybind_manager::Message> {

        let content: Element<'_, Message> =
        container(column![
            text("You have unsaved changes!").size(KEYBIND_TEXT_SIZE),
            row![
                container(row![
                    button(row![
                        text("Save").size(KEYBIND_TEXT_SIZE),
                        icon(Icon::AddCircle).style(text::success).size(KEYBIND_ICON_SIZE)
                    ])
                    .padding(KEYBIND_ROW_PADDING)
                    .on_press(Message::SaveChangesProfileSelected)
                    .style(button::success),
                    button(row![
                        text("Ignore").size(KEYBIND_TEXT_SIZE),
                        icon(Icon::AddCircle).style(text::success).size(KEYBIND_ICON_SIZE)
                    ])
                    .padding(KEYBIND_ROW_PADDING)
                    .on_press(Message::IgnoreChangesProfileSelected)
                    .style(button::danger),
                    button(row![
                        text("Cancel").size(KEYBIND_TEXT_SIZE),
                        icon(Icon::Cancel).style(text::danger).size(KEYBIND_ICON_SIZE)
                    ])
                    .padding(KEYBIND_ROW_PADDING)
                    .on_press(Message::CancelProfileSelected)
                    .style(button::warning),
                ]).align_right(Length::Fill)
            ],
        ]).style(container::bordered_box).into();
        
        content.map(keybind_manager::Message::Keybindings)
    }
    pub fn view_prompt_new_keybind(state: &State) -> Element<'_, keybind_manager::Message> {

        let content: Element<'_, Message> = container(column![
            combo_box(
                &state.input_list_state_new_keybind, 
                "Input",
                state.selected_input_new_keybind.as_ref(),
                Message::SelectNewKeybindInput
            )
            .size(KEYBIND_TEXT_SIZE)
            .padding(KEYBIND_ROW_PADDING),

            row![
                combo_box(
                    &state.source_list_state_new_keybind, 
                    "Source",
                    state.selected_source_new_keybind.as_ref(),
                    Message::SelectNewKeybindSource
                )
                .size(KEYBIND_TEXT_SIZE)
                .padding(KEYBIND_ROW_PADDING),

                combo_box(
                    &state.keycode_list_state_new_keybind, 
                    match state.selected_source_new_keybind {
                        None => "<- Select source",
                        _ => "Keycode",
                    },
                    state.selected_keycode_new_keybind.as_ref(),
                    Message::SelectNewKeybindKeycode
                )
                .size(KEYBIND_TEXT_SIZE)
                .padding(KEYBIND_ROW_PADDING),
            
                container(row![
                    button(icon(Icon::AddCircle).style(text::success).size(KEYBIND_ICON_SIZE))
                        .padding(KEYBIND_ROW_PADDING)
                        .on_press(Message::SaveNewKeybind)
                        .style(styling::button_transparent),
                    button(icon(Icon::Cancel).style(text::danger).size(KEYBIND_ICON_SIZE))
                        .padding(KEYBIND_ROW_PADDING)
                        .on_press(Message::CancelNewKeybind)
                        .style(styling::button_transparent),
                ]).align_right(Length::Fill)
            ],
        ]).style(container::bordered_box).into();
        content.map(keybind_manager::Message::Keybindings)
    }
}