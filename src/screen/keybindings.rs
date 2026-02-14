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
    styling,
    asset::{
        Icon,
        icon,
    },
    keybind_manager::{
        self,
        State,
    },
    keybind::*,
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

const UNSAVED_PROMPT_WIDTH: f32 = 500.0;
const UNSAVED_TITLE_SIZE: f32 = 28.0;
const UNSAVED_TITLE_PADDING: f32 = 24.0;
const UNSAVED_TITLE_GAP: f32 = 48.0; // space().height() between title and buttons
const UNSAVED_BUTTON_TEXT_SIZE: f32 = 20.0;
const UNSAVED_BUTTON_ICON_SIZE: f32 = 28.0;
const UNSAVED_BUTTON_PADDING: f32 = 14.0;

const MISMATCH_PROMPT_WIDTH: f32 = UNSAVED_PROMPT_WIDTH; //500.0;
const MISMATCH_TITLE_SIZE: f32 = UNSAVED_TITLE_SIZE; //28.0;
const MISMATCH_TITLE_PADDING: f32 = UNSAVED_TITLE_PADDING; //24.0;
const MISMATCH_TITLE_GAP: f32 = UNSAVED_TITLE_GAP; //48.0;
const MISMATCH_BUTTON_TEXT_SIZE: f32 = UNSAVED_BUTTON_TEXT_SIZE; //20.0;
const MISMATCH_BUTTON_ICON_SIZE: f32 = UNSAVED_BUTTON_ICON_SIZE; //28.0;
const MISMATCH_BUTTON_PADDING: f32 = UNSAVED_BUTTON_PADDING; //14.0;

const NEWKEYBIND_PROMPT_WIDTH: f32 = 600.0;
const NEWKEYBIND_BUTTON_ICON_SIZE: f32 = 36.0;
const NEWKEYBIND_BUTTON_PADDING: f32 = 8.0;
const NEWKEYBIND_COMBO_PADDING: f32 = 12.0;

#[derive(Debug, Clone)]
pub enum Message {
    SelectNewProfile(String),
    SaveChangesToSelectedProfile,
    IgnoreChangesToSelectedProfile,
    CancelChangesToSelectedProfile,
    SaveSelectedProfile,
    Search(String),
    NewKeybindBegin,
    NewKeybindSelectInputCategory(KeybindInputCategory),
    NewKeybindSelectInputCode(KeybindInputCode),
    NewKeybindSelectSource(KeybindSource),
    NewKeybindSelectKeycode(KeybindKeycode),
    NewKeybindSave,
    NewKeybindCancel,
    DeleteKeybind(KeybindId),
    MismatchRestore,
    MismatchReload,
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

        let res = profile.load().err();
        match res {
            Some(err) => println!("load: Error reading xml: {} | {}", err.0, err.1),
            None => state.selected_profile = Some(profile_ref.clone()),
        }
    }
    fn restore_selected_profile(state: &mut State) {
        state.selected_profile.as_ref().map(|profile_ref|{
            let mut profile = profile_ref.borrow_mut();
            profile.restore().err().map(|err| println!("restore: Error reading xml: {} | {}", err.0, err.1));
        });
        ()
    }

    pub fn verify_latest(state: &mut State){
        let Some(profile_ref) = &state.selected_profile else { return };
        let profile = profile_ref.borrow();
        if !profile.verify_latest() {
            state.prompt = Some(Prompt::ProfileMismatch(Self::view_prompt_profile_mismatch));
        }
    }

    pub fn update(state: &mut State, message: &Message) {
        match message {
            Message::SelectNewProfile(name) => {
                state.selected_profile_name = Some(name.clone());

                if let Some(profile) = state.selected_profile.as_ref() {
                    if profile.borrow().modified {
                        state.prompt = Some(Prompt::UnsavedChanges(Self::view_prompt_unsaved_changes));
                        return
                    }
                }
                Self::load_selected_profile(state);
                Self::verify_latest(state);
            },
            Message::SaveChangesToSelectedProfile => {
                state.prompt = None;
                Self::save_selected_profile(state);
                Self::load_selected_profile(state)
            },
            Message::IgnoreChangesToSelectedProfile => {
                state.prompt = None;
                Self::load_selected_profile(state)
            },
            Message::CancelChangesToSelectedProfile => {
                state.prompt = None
            }
            Message::SaveSelectedProfile => {
                Self::save_selected_profile(state);
            },
            Message::Search(string) => {
                state.search_string = string.clone();
                println!("Search: {}", string)
            }
            Message::NewKeybindBegin => {
                state.prompt = Some(Prompt::NewKeybind(Self::view_prompt_new_keybind));
            },
            Message::NewKeybindSelectInputCategory(_input_category) => {
                let input_category = Some(_input_category.clone());
                if state.new_keybind_builder.input_category == input_category { return };

                state.new_keybind_builder.input_code = None;
                state.new_keybind_builder.input_category = input_category;

                state.new_keybind_input_code_list_state = combo_box::State::new(state.input_codes
                    .get(_input_category.index)
                    .unwrap()
                    .clone()
                );
            },
            Message::NewKeybindSelectInputCode(input_code) => {
                state.new_keybind_builder.input_code = Some(input_code.clone());
            },
            Message::NewKeybindSelectSource(_source) => {
                let source = Some(_source.clone());
                if state.new_keybind_builder.source == source { return };

                state.new_keybind_builder.keycode = None;
                state.new_keybind_builder.source = source;

                state.new_keybind_keycode_list_state = combo_box::State::new(state.keycodes
                    .get(_source.index)
                    .unwrap()
                    .clone()
                );
            },
            Message::NewKeybindSelectKeycode(keycode) => {
                state.new_keybind_builder.keycode = Some(keycode.clone());
            },
            Message::NewKeybindSave => {
                state.prompt = None;

                let (Some(input_category),Some(input_code),Some(source),Some(keycode))
                    = state.new_keybind_builder.clone().into() else { return };
                

                state.selected_profile.as_ref().map(|profile_ref| {
                    let mut profile = profile_ref.borrow_mut();
                    let id = profile.next_id();
                    profile.add_keybind(Keybind::new_from_primitives(input_category, input_code, source, keycode, id));
                });
            }
            Message::NewKeybindCancel => {
                state.prompt = None;
            }
            Message::DeleteKeybind(id) => {
                state.selected_profile.as_ref().map(|profile_ref| {
                    profile_ref.borrow_mut().remove_keybind(*id);
                });
            },
            Message::MismatchRestore => {
                state.prompt = None;
                Self::restore_selected_profile(state);
                Self::save_selected_profile(state);
            }
            Message::MismatchReload => {
                state.prompt = None;
                Self::load_selected_profile(state);
                Self::save_selected_profile(state);
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
                    Message::SelectNewProfile
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

        let mut keybinds_sorted = keybinds.clone();
        keybinds_sorted.sort_by_key(|keybind|
            (keybind.input.category.index, keybind.id)
        );
        
        let keybind_list = column![].extend(
            keybinds_sorted.iter()
                .enumerate()
                .map(|(index, keybind)|{

                
                container(row![
                    space().width(10),
                    
                    container(column![
                            text(keybind.input.code.pretty_name).size(KEYBIND_TEXT_SIZE),
                            text(keybind.input.category.pretty_name).size(KEYBIND_SUBTEXT_SIZE).style(text::secondary),
                            //text!("#{}", keybind.id).size(KEYBIND_SUBTEXT_SIZE).style(text::warning),
                    ]).clip(true).width(iced::FillPortion(20)),

                    space().width(10),
                    
                    container(column![
                        text(keybind.key.source.pretty_name).size(KEYBIND_TEXT_SIZE),
                        text(keybind.key.source.desc).size(KEYBIND_SUBTEXT_SIZE).style(text::primary),
                    ]).clip(true).width(iced::FillPortion(15)),
                    
                    space().width(10),
                    
                    container(column![
                        text(keybind.key.keycode.pretty_name).size(KEYBIND_TEXT_SIZE),
                        text(keybind.key.keycode.desc).size(KEYBIND_SUBTEXT_SIZE).style(text::primary),
                    ]).clip(true).width(iced::FillPortion(10)),
                    
                    container(
                        button(
                            icon(Icon::Cancel).size(KEYBIND_ICON_SIZE).center().style(text::danger)
                        )
                        .on_press(Message::DeleteKeybind(keybind.id))
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
                //text_input("Search            Examples    \"weapon\"    \"i:frontend\"    \"s:mouse\"    \"k:enter\"", &state.search_string)
                text_input("Search     ! not implemented !", &state.search_string)
                .on_input(Message::Search)
                .line_height(LineHeight::Absolute(TOOLBAR_HEIGHT.into()))
                .size(24),
                container( button(
                    icon(Icon::Add).size(TOOLBAR_ICON_SIZE).center(),
                ).on_press(Message::NewKeybindBegin).style(button::success))
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
            container(
                text("You have unsaved changes!").size(UNSAVED_TITLE_SIZE).center()
            ).padding(iced::padding::vertical(UNSAVED_TITLE_PADDING))
            .center_x(Length::Fill),
            space().height(UNSAVED_TITLE_GAP),
            container(row![
                container(row![
                    button(row![
                        text("Save  ").size(UNSAVED_BUTTON_TEXT_SIZE).center(),
                        icon(Icon::AddCircle).size(UNSAVED_BUTTON_ICON_SIZE).center()
                    ].align_y(iced::Center))
                    .on_press(Message::SaveChangesToSelectedProfile)
                    .style(button::success),
                    space().width(UNSAVED_BUTTON_PADDING),
                    button(row![
                        text("Ignore  ").size(UNSAVED_BUTTON_TEXT_SIZE).center(),
                        icon(Icon::Block).size(UNSAVED_BUTTON_ICON_SIZE).center()
                    ].align_y(iced::Center))
                    .on_press(Message::IgnoreChangesToSelectedProfile)
                    .style(button::danger),
                    space().width(UNSAVED_BUTTON_PADDING),
                    button(row![
                        text("Cancel  ").size(UNSAVED_BUTTON_TEXT_SIZE).center(),
                        icon(Icon::Cancel).size(UNSAVED_BUTTON_ICON_SIZE).center()
                    ].align_y(iced::Center))
                    .on_press(Message::CancelChangesToSelectedProfile)
                    .style(button::warning),
                ]).align_right(Length::Fill)
            ]).padding(UNSAVED_BUTTON_PADDING),
        ])
        .style(container::bordered_box)
        .width(Length::Fixed(UNSAVED_PROMPT_WIDTH))
        .into();
        
        content.map(keybind_manager::Message::Keybindings)
    }
    pub fn view_prompt_new_keybind(state: &State) -> Element<'_, keybind_manager::Message> {

        let content: Element<'_, Message> =
        container(column![
            container(
                combo_box(
                    &state.new_keybind_input_category_list_state, 
                    "Input Category",
                    state.new_keybind_builder.input_category.as_ref(),
                    Message::NewKeybindSelectInputCategory
                )
                .size(KEYBIND_TEXT_SIZE)
            )
            .padding(NEWKEYBIND_COMBO_PADDING),
            container(
                combo_box(
                    &state.new_keybind_input_code_list_state, 
                    "Input Code",
                    state.new_keybind_builder.input_code.as_ref(),
                    Message::NewKeybindSelectInputCode
                )
                .size(KEYBIND_TEXT_SIZE)
            )
            .padding(NEWKEYBIND_COMBO_PADDING),

            row![
                container(
                    combo_box(
                        &state.new_keybind_source_list_state, 
                        "Source",
                        state.new_keybind_builder.source.as_ref(),
                        Message::NewKeybindSelectSource
                    )
                    .size(KEYBIND_TEXT_SIZE),
                )
                .padding( iced::padding::horizontal(NEWKEYBIND_COMBO_PADDING)),
                container(
                    combo_box(
                        &state.new_keybind_keycode_list_state, 
                        match state.new_keybind_builder.source {
                            None => "<- Select source",
                            _ => "Keycode",
                        },
                        state.new_keybind_builder.keycode.as_ref(),
                        Message::NewKeybindSelectKeycode
                    )
                    .size(KEYBIND_TEXT_SIZE),
                )
                .width(Length::Fill),
                container(row![
                    button(icon(Icon::AddCircle).style(match state.new_keybind_builder.clone().into() {
                            (Some(_), Some(_), Some(_), Some(_)) => text::success,
                            _ => text::default,
                        })
                        .size(NEWKEYBIND_BUTTON_ICON_SIZE))
                        .on_press_maybe( match state.new_keybind_builder.clone().into() {
                            (Some(_), Some(_), Some(_), Some(_)) => Some(Message::NewKeybindSave),
                            _ => None,
                        })
                    .style(styling::button_transparent),
                    space().width(NEWKEYBIND_BUTTON_PADDING),
                    button(icon(Icon::Cancel).style(text::danger).size(NEWKEYBIND_BUTTON_ICON_SIZE))
                    .on_press(Message::NewKeybindCancel)
                    .style(styling::button_transparent),
                ])
                .padding(NEWKEYBIND_BUTTON_PADDING)
                .align_right(Length::Shrink)
            ].align_y(iced::Center),
        ])
        .style(container::bordered_box)
        .width(Length::Fixed(NEWKEYBIND_PROMPT_WIDTH))
        .into();
        content.map(keybind_manager::Message::Keybindings)
    }
    pub fn view_prompt_profile_mismatch(_state: &State) -> Element<'_, keybind_manager::Message> {
        let content: Element<'_, Message> =
        container(column![
            container(column![
                text("Keybind Mismatch!").style(text::danger).size(MISMATCH_TITLE_SIZE).center(),
                text("user.xml was modified since last save").style(text::secondary),
                text("\"Restore\" will reapply your old keybinds").style(text::secondary),
                text("\"Restore\" will load user.xml as is and save it as latest").style(text::secondary),
            ].align_x(iced::Center)).padding(iced::padding::vertical(MISMATCH_TITLE_PADDING))
            .center_x(Length::Fill),
            space().height(MISMATCH_TITLE_GAP),
            container(row![
                container(row![
                    button(row![
                        text("Restore  ").size(MISMATCH_BUTTON_TEXT_SIZE).center(),
                        icon(Icon::AddCircle).size(MISMATCH_BUTTON_ICON_SIZE).center()
                    ].align_y(iced::Center))
                    .on_press(Message::MismatchRestore)
                    .style(button::success),
                    space().width(MISMATCH_BUTTON_PADDING),
                    button(row![
                        text("Reload  ").size(MISMATCH_BUTTON_TEXT_SIZE).center(),
                        icon(Icon::Block).size(MISMATCH_BUTTON_ICON_SIZE).center()
                    ].align_y(iced::Center))
                    .on_press(Message::MismatchReload)
                    .style(button::danger),
                    space().width(MISMATCH_BUTTON_PADDING),
                ]).align_right(Length::Fill)
            ]).padding(MISMATCH_BUTTON_PADDING),
        ])
        .style(container::bordered_box)
        .width(Length::Fixed(MISMATCH_PROMPT_WIDTH))
        .into();
        
        content.map(keybind_manager::Message::Keybindings)
    }
}