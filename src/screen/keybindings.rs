use iced::{
    Element, Length, Task, clipboard, widget::{
        button, column, combo_box, container, pick_list, row, scrollable, space, text, text::LineHeight, text_input
    }
};

use fuzzy_matcher::{
    FuzzyMatcher, skim::SkimMatcherV2
};

use crate::{
    asset::{
        Icon,
        icon,
    }, gui::Prompt, keybind::*, keybind_manager::{
        self,
        State,
    }, profile::{
        ProfileRef
    }, screen::Screen, styling
};

pub const HEADER_TEXT_SIZE: f32 = 28.0;

pub const TOOLBAR_HEIGHT: f32 = 40.0;
pub const TOOLBAR_ICON_SIZE: f32 = 28.0;

pub const KEYBIND_TEXT_SIZE: f32 = 20.0;
pub const KEYBIND_SUBTEXT_SIZE: f32 = 12.0;
pub const KEYBIND_ROW_PADDING: f32 = 4.0;
pub const KEYBIND_HEIGHT: f32 = 50.0;
pub const KEYBIND_ICON_SIZE: f32 = 36.0;

pub const UNSAVED_PROMPT_WIDTH: f32 = 500.0;
pub const UNSAVED_TITLE_SIZE: f32 = 28.0;
pub const UNSAVED_TITLE_PADDING: f32 = 24.0;
pub const UNSAVED_TITLE_GAP: f32 = 48.0; // space().height() between title and buttons
pub const UNSAVED_BUTTON_TEXT_SIZE: f32 = 20.0;
pub const UNSAVED_BUTTON_ICON_SIZE: f32 = 28.0;
pub const UNSAVED_BUTTON_PADDING: f32 = 14.0;

pub const MISMATCH_PROMPT_WIDTH: f32 = UNSAVED_PROMPT_WIDTH; //500.0;
pub const MISMATCH_TITLE_SIZE: f32 = UNSAVED_TITLE_SIZE; //28.0;
pub const MISMATCH_TITLE_PADDING: f32 = UNSAVED_TITLE_PADDING; //24.0;
pub const MISMATCH_TITLE_GAP: f32 = UNSAVED_TITLE_GAP; //48.0;
pub const MISMATCH_BUTTON_TEXT_SIZE: f32 = UNSAVED_BUTTON_TEXT_SIZE; //20.0;
pub const MISMATCH_BUTTON_ICON_SIZE: f32 = UNSAVED_BUTTON_ICON_SIZE; //28.0;
pub const MISMATCH_BUTTON_PADDING: f32 = UNSAVED_BUTTON_PADDING; //14.0;

pub const NEWKEYBIND_PROMPT_WIDTH: f32 = 600.0;
pub const NEWKEYBIND_BUTTON_ICON_SIZE: f32 = 36.0;
pub const NEWKEYBIND_BUTTON_PADDING: f32 = 8.0;
pub const NEWKEYBIND_COMBO_PADDING: f32 = 12.0;

pub const SHARE_TEXT_SIZE: f32 = HEADER_TEXT_SIZE;
pub const SHARE_PADDING: f32 = 24.0;

#[derive(Debug, Clone, Default)]
pub enum EditorMode {
#[default]
    Adding,
    Modifying(KeybindId),
}

#[derive(Debug, Clone)]
pub enum Message {
    ProfileSelect(String),
    ProfileSave,

    ProfileUnsavedChangesSave,
    ProfileUnsavedChangesIgnore,
    ProfileUnsavedChangesCancel,

    Search(String),
    
    KeybindEditorBeginNew,
    KeybindEditorBeginEdit(KeybindId),
    KeybindEditorSelectInputCategory(KeybindInputCategory),
    KeybindEditorSelectInputCode(KeybindInputCode),
    KeybindEditorSelectSource(KeybindSource),
    KeybindEditorSelectKeycode(KeybindKeycode),
    KeybindEditorSave,
    KeybindEditorCancel,
    
    RenameBegin,
    RenameInputBoxChanged(String),
    RenameFinalize,

    DeleteKeybind(KeybindId),
    
    ShareExportProfileToClipboard,
    ShareImportInputBox(String),
    ShareImport,
    ShareImportUnsavedSave,
    ShareImportUnsavedIgnore,
    ShareImportUnsavedCancel,

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
    fn set_editor_combo_boxes(state: &mut State) {
        state.keybind_editor_builder.input_category.as_ref().map(|input_category|{
            state.keybind_editor_input_code_list_state = combo_box::State::new(state.input_codes
                .get(input_category.index)
                .unwrap()
                .clone()
            );
        });
        state.keybind_editor_builder.source.as_ref().map(|source|{
            state.keybind_editor_keycode_list_state = combo_box::State::new(state.keycodes
                .get(source.index)
                .unwrap()
                .clone()
            );
        });
    }
    fn save_selected_profile(state: &State) {
        state.selected_profile.as_ref().map(|profile_ref| {
            let mut profile = profile_ref.borrow_mut();
            if let Err(err) = profile.write_xml(){
                println!("Error writing xml '{}': {} | {}", profile.name, err.0, err.1);
            }
        });
    }
    pub fn load_selected_profile(state: &mut State) {
        let Some(selected_profile_name) = state.requested_profile.as_ref() else { return };

        let Some(profile_ref) = state
            .available_profiles
            .iter()
            .find(|p| &p.borrow().name == selected_profile_name)
            .cloned() else { return };

        let mut profile = profile_ref.borrow_mut();

        let res = profile.load().err();
        match res {
            Some(err) => println!("load: Error reading xml: {} | {}", err.0, err.1),
            None => {
                state.selected_profile = Some(profile_ref.clone())
            },
        }
    }
    fn restore_selected_profile(state: &mut State) {
        state.selected_profile.as_ref().map(|profile_ref|{
            let mut profile = profile_ref.borrow_mut();
            profile.restore().err().map(|err| println!("restore: Error reading xml: {} | {}", err.0, err.1));
        });
        ()
    }

    pub fn load_serialized(state: &mut State) {
        let Some(profile_ref) = state.selected_profile.clone() else { return };
        let mut profile = profile_ref.borrow_mut();
        
        match profile.load_serialized(state.import_string.to_owned()){
            Ok(_) => {
                state.import_string = "".to_owned()
            },
            Err(err) => state.import_string = err,
        }
    }

    pub fn verify_latest(state: &mut State){
        let Some(profile_ref) = &state.selected_profile else { return };
        let profile = profile_ref.borrow();
        if !profile.verify_latest() {
            state.prompt = Some(Prompt::ProfileMismatch(Self::view_prompt_profile_mismatch));
        }
    }

    pub fn update(state: &mut State, message: &Message) -> iced::Task<Message>{
        match message {
            Message::ProfileSelect(name) => {
                state.requested_profile = Some(name.clone());

                if let Some(profile) = state.selected_profile.as_ref() {
                    if profile.borrow().modified {
                        state.prompt = Some(
                            Prompt::UnsavedChanges(
                                Self::view_prompt_unsaved_changes,
                                Message::ProfileUnsavedChangesSave,
                                Message::ProfileUnsavedChangesIgnore,
                                Message::ProfileUnsavedChangesCancel
                            )
                        );
                        return Task::none()
                    }
                }
                Self::load_selected_profile(state);
                Self::verify_latest(state);
            }
            Message::ProfileSave => {
                Self::save_selected_profile(state);
            }
            
            Message::ProfileUnsavedChangesSave => {
                state.prompt = None;
                Self::save_selected_profile(state);
                Self::load_selected_profile(state)
            }
            Message::ProfileUnsavedChangesIgnore => {
                state.prompt = None;
                Self::load_selected_profile(state)
            }
            Message::ProfileUnsavedChangesCancel => {
                state.prompt = None
            }
            
            Message::Search(string) => {
                state.search_string = string.clone();
            }
            
            Message::KeybindEditorBeginNew => {
                state.keybind_editor_mode = EditorMode::Adding;
                state.prompt = Some(Prompt::KeybindEditor(Self::view_prompt_keybind_editor));
                Self::set_editor_combo_boxes(state);
            }

            Message::KeybindEditorBeginEdit(id) => {
                let Some(builder) = state.selected_profile.as_ref().map(|p| p.borrow().get_keybind_as_builder(id)) else { return Task::none() };
                
                state.keybind_editor_builder = builder;
                state.keybind_editor_mode = EditorMode::Modifying(*id);
                state.prompt = Some(Prompt::KeybindEditor(Self::view_prompt_keybind_editor));
                Self::set_editor_combo_boxes(state);
            }
            Message::KeybindEditorSelectInputCategory(_input_category) => {
                let input_category = Some(_input_category.clone());
                if state.keybind_editor_builder.input_category == input_category { return Task::none() };

                state.keybind_editor_builder.input_code = None;
                state.keybind_editor_builder.input_category = input_category;

                Self::set_editor_combo_boxes(state);
            }
            Message::KeybindEditorSelectInputCode(input_code) => {
                state.keybind_editor_builder.input_code = Some(input_code.clone());
            }
            Message::KeybindEditorSelectSource(_source) => {
                let source = Some(_source.clone());
                if state.keybind_editor_builder.source == source { return Task::none() };

                state.keybind_editor_builder.keycode = None;
                state.keybind_editor_builder.source = source;

                Self::set_editor_combo_boxes(state);
            }
            Message::KeybindEditorSelectKeycode(keycode) => {
                state.keybind_editor_builder.keycode = Some(keycode.clone());
            }
            Message::KeybindEditorSave => {
                state.prompt = None;

                let (Some(input_category),Some(input_code),Some(source),Some(keycode))
                    = state.keybind_editor_builder.clone().into() else { return Task::none()};
                

                state.selected_profile.as_ref().map(|profile_ref| {
                    let mut profile = profile_ref.borrow_mut();

                    let id = match state.keybind_editor_mode {
                        EditorMode::Adding => profile.next_id(),
                        EditorMode::Modifying(id) => id,
                    };

                    profile.modify_or_add_keybind(Keybind::new_from_primitives(input_category, input_code, source, keycode, id));
                });
            }
            Message::KeybindEditorCancel => {
                state.prompt = None;
            }
            
            Message::RenameBegin => {
                state.selected_profile.as_ref().map(|profile_ref| {
                    state.renaming_profile = true;
                    state.renaming_profile_string = profile_ref.borrow_mut().name.clone();
                });
            }
            Message::RenameInputBoxChanged(string) => {
                state.renaming_profile_string = string.clone();
            }
            Message::RenameFinalize => {
                state.selected_profile.as_ref().map(|profile_ref| {
                    profile_ref.borrow_mut().name = state.renaming_profile_string.clone();
                    state.renaming_profile = false;
                });
            }

            Message::DeleteKeybind(id) => {
                state.selected_profile.as_ref().map(|profile_ref| {
                    profile_ref.borrow_mut().remove_keybind(id);
                });
            }
            Message::ShareExportProfileToClipboard => {
                let Some(profile_ref) = state.selected_profile.clone() else { return Task::none() };
                let profile = profile_ref.borrow();
                let serialized = profile.serialized();
                println!("{:?}", serialized);
                return clipboard::write(serialized)
            }
            Message::ShareImportInputBox(str) => {
                state.import_string = str.clone();
            }
            Message::ShareImport => {
                {
                    let Some(profile_ref) = state.selected_profile.clone() else { return Task::none() };
                    let profile = profile_ref.borrow();
                    if profile.modified {
                        state.prompt = Some(Prompt::UnsavedChanges(
                            Self::view_prompt_unsaved_changes,
                            Message::ShareImportUnsavedSave,
                            Message::ShareImportUnsavedIgnore,
                            Message::ShareImportUnsavedCancel
                        ));
                        return Task::none()
                    }
                }
                Self::load_serialized(state);

                // TODO: list of buttons to load config/presets/<names>
                // probably just save them as xml files in config/presets
            }
            Message::ShareImportUnsavedSave => {
                Self::save_selected_profile(state);
                
                let Some(profile_ref) = state.selected_profile.clone() else { return Task::none() };
                let mut profile = profile_ref.borrow_mut();
                match profile.load_serialized(state.import_string.to_owned()){
                    Ok(_) => state.import_string = "".to_owned(),
                    Err(err) => state.import_string = err,
                }
                state.prompt = None;
            }
            Message::ShareImportUnsavedIgnore => {
                let Some(profile_ref) = state.selected_profile.clone() else { return Task::none() };
                let mut profile = profile_ref.borrow_mut();
                match profile.load_serialized(state.import_string.to_owned()){
                    Ok(_) => state.import_string = "".to_owned(),
                    Err(err) => state.import_string = err,
                }
                state.prompt = None;
            }
            Message::ShareImportUnsavedCancel => {
                state.prompt = None;
            }
            
            Message::MismatchRestore => {
                state.prompt = None;
                Self::restore_selected_profile(state);
                Self::save_selected_profile(state);
            }
            Message::MismatchReload => {
                state.prompt = None;
                Self::load_selected_profile(state);
                Self::save_selected_profile(state);
            }
        }

        Task::none()
    }

    pub fn view<'a>(&self, state: &'a State) -> (Element<'a, Message>, Element<'a, Message>) {

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

        let picker_content: iced::Element<'_, Message> = match state.renaming_profile {
                    false => {
                        pick_list(
                            available_profile_names,
                            selected_profile_name,
                            Message::ProfileSelect
                        )
                        .placeholder("none")
                        .text_size(HEADER_TEXT_SIZE)
                        .into()
                    },
                    true => {
                        text_input("", &state.renaming_profile_string)
                        .size(HEADER_TEXT_SIZE)
                        .width(400)
                        .on_input(Message::RenameInputBoxChanged)
                        .on_submit(Message::RenameFinalize)
                        .into()
                    },
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
                picker_content
            )
            .height(iced::Fill)
            .align_y(iced::Center),
            space().width(Length::Fixed(HEADER_TEXT_SIZE / 4.0)),
            container(
                button(
                    icon(if state.renaming_profile { Icon::Add } else { Icon::Edit }).size(HEADER_TEXT_SIZE).center()
                    .style( if state.renaming_profile { text::success } else { text::default })
                )
                .on_press_maybe( if state.renaming_profile { Some(Message::RenameFinalize) } else { Some(Message::RenameBegin) })
                .style(styling::button_transparent)
            )
            .height(iced::Fill)
            .align_y(iced::Center),
            container(
                button(
                    icon( if profile_modified_flag { Icon::SaveAs } else { Icon::Save }).size(HEADER_TEXT_SIZE).center()
                    .style( if profile_modified_flag { text::warning } else { text::default })
                )
                .on_press_maybe( if profile_modified_flag { Some(Message::ProfileSave) } else { None }).style(styling::button_transparent)
            )
            .height(iced::Fill)
            .align_y(iced::Center),
        ]
        ).align_left(iced::Fill).into();
        
        
        let Some(profile_ref) = state.selected_profile.clone() else {
            return (container(text("No Profile Selected").center()).center(Length::Fill).into(), header)
        };

        let content = match state.screen {
            Screen::Keybindings(_) => Self::view_keybindings_content(state, profile_ref),
            Screen::Share(_) => Self::view_share_content(state, profile_ref),
            _ => text("error you cant possibly be on this screen").into(),
        };

        return (
            content,
            header
        )
    }

    fn view_keybindings_content(state: &State, profile_ref: ProfileRef) -> Element<'_, Message> {
        let content = {
        let profile = profile_ref.borrow();
        let keybinds = &profile.keybinds;

        let matcher = SkimMatcherV2::default();
        let mut keybinds_sorted = keybinds.clone();
        keybinds_sorted.sort_by_key(|keybind|
            (
                match &state.search_string {
                    s if s.starts_with("i:") => matcher.fuzzy_match(keybind.input.code.pretty_name, &s[2..]).unwrap_or(0),
                    s if s.starts_with("s:") => matcher.fuzzy_match(keybind.key.source.pretty_name, &s[2..]).unwrap_or(0),
                    s if s.starts_with("k:") => matcher.fuzzy_match(keybind.key.keycode.pretty_name, &s[2..]).unwrap_or(0),
                    _ => [
                        matcher.fuzzy_match(keybind.input.category.pretty_name, &state.search_string).unwrap_or(0),
                        matcher.fuzzy_match(keybind.input.code.pretty_name, &state.search_string).unwrap_or(0),
                        matcher.fuzzy_match(keybind.key.source.pretty_name, &state.search_string).unwrap_or(0),
                        matcher.fuzzy_match(keybind.key.keycode.pretty_name, &state.search_string).unwrap_or(0)
                    ].into_iter().max().unwrap_or(0),
                },
                keybind.input.category.index,
                keybind.id
            )
        );
        keybinds_sorted.reverse();


        let keybind_list = column![].extend(
            keybinds_sorted.iter()
            .enumerate()
            .map(|(index, keybind)|{
                container(row![
                    space().width(10),
                    
                    container(column![
                            text(keybind.input.code.pretty_name).size(KEYBIND_TEXT_SIZE),
                            text(keybind.input.category.pretty_name).size(KEYBIND_SUBTEXT_SIZE).style(text::secondary),
                    ]).clip(true).width(iced::FillPortion(1)),

                    space().width(10),
                    
                    container(column![
                        text(keybind.key.source.pretty_name).size(KEYBIND_TEXT_SIZE),
                        text(keybind.key.source.desc).size(KEYBIND_SUBTEXT_SIZE).style(text::primary),
                    ]).clip(true).width(iced::FillPortion(1)),
                    
                    space().width(10),
                    
                    container(column![
                        text(keybind.key.keycode.pretty_name).size(KEYBIND_TEXT_SIZE),
                        text(keybind.key.keycode.desc).size(KEYBIND_SUBTEXT_SIZE).style(text::primary),
                    ]).clip(true).width(iced::FillPortion(1)),
                    
                    container(
                        button(
                            icon(Icon::Edit).size(KEYBIND_ICON_SIZE).center()
                        )
                        .on_press(Message::KeybindEditorBeginEdit(keybind.id))
                        .style(styling::button_transparent)
                    )
                    .align_right(Length::Fixed(KEYBIND_HEIGHT)),

                    space().width(10),

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
        scrollable(keybind_list)
        };




        let toolbar = container(
            row![
                text_input("Search        Filters: \"i:<input>\", \"s:<source>\", \"k:<key>\"", &state.search_string)
                .on_input(Message::Search)
                .line_height(LineHeight::Absolute(TOOLBAR_HEIGHT.into()))
                .size(24),
                container( button(
                    icon(Icon::Add).size(TOOLBAR_ICON_SIZE).center(),
                ).on_press(Message::KeybindEditorBeginNew).style(button::success))
                .height(Length::Fill)
                .width(Length::Shrink)
                .style(container::bordered_box),
            ],
        )
        .align_right(Length::Fill)
        .height(Length::Fixed(TOOLBAR_HEIGHT));

        column!(
            toolbar,
            content,
        ).into()
    }
    
    fn view_share_content(state: &State, _profile_ref: ProfileRef) -> Element<'_, Message> {
        let content = container(column![
            row![
                button(text("Export to clipboard").size(SHARE_TEXT_SIZE))
                .on_press(Message::ShareExportProfileToClipboard),
            ],
            
            space().height(SHARE_PADDING),

            text_input("Import from code (press enter to submit)", &state.import_string)
            .size(SHARE_TEXT_SIZE)
            .on_input(Message::ShareImportInputBox)
            .on_submit(Message::ShareImport),
        ]).padding(SHARE_PADDING);
        
        content.into()
    }

    pub fn view_prompt_unsaved_changes(_state: &State, accept_message: Message, ignore_message: Message, cancel_message: Message) -> Element<'_, keybind_manager::Message> {
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
                    .on_press(accept_message)
                    .style(button::success),
                    space().width(UNSAVED_BUTTON_PADDING),
                    button(row![
                        text("Ignore  ").size(UNSAVED_BUTTON_TEXT_SIZE).center(),
                        icon(Icon::Block).size(UNSAVED_BUTTON_ICON_SIZE).center()
                    ].align_y(iced::Center))
                    .on_press(ignore_message)
                    .style(button::danger),
                    space().width(UNSAVED_BUTTON_PADDING),
                    button(row![
                        text("Cancel  ").size(UNSAVED_BUTTON_TEXT_SIZE).center(),
                        icon(Icon::Cancel).size(UNSAVED_BUTTON_ICON_SIZE).center()
                    ].align_y(iced::Center))
                    .on_press(cancel_message)
                    .style(button::warning),
                ]).align_right(Length::Fill)
            ]).padding(UNSAVED_BUTTON_PADDING),
        ])
        .style(container::bordered_box)
        .width(Length::Fixed(UNSAVED_PROMPT_WIDTH))
        .into();
        
        content.map(keybind_manager::Message::Keybindings)
    }
    pub fn view_prompt_keybind_editor(state: &State) -> Element<'_, keybind_manager::Message> {

        let content: Element<'_, Message> =
        container(column![
            space().height(NEWKEYBIND_COMBO_PADDING),

            container(
                match state.keybind_editor_mode{
                    EditorMode::Adding => {row![text("New Keybind ").size(KEYBIND_TEXT_SIZE), icon(Icon::Add).size(KEYBIND_TEXT_SIZE).style(text::success)]},
                    EditorMode::Modifying(_) => {row![text("Modify Keybind ").size(KEYBIND_TEXT_SIZE), icon(Icon::Edit).size(KEYBIND_TEXT_SIZE).style(text::warning)]},
                }
            )
            .padding(iced::padding::left(NEWKEYBIND_COMBO_PADDING)),
            
            container(
                combo_box(
                    &state.keybind_editor_input_category_list_state, 
                    "Input Category",
                    state.keybind_editor_builder.input_category.as_ref(),
                    Message::KeybindEditorSelectInputCategory
                )
                .size(KEYBIND_TEXT_SIZE)
            )
            .padding(NEWKEYBIND_COMBO_PADDING),
            
            container(
                combo_box(
                    &state.keybind_editor_input_code_list_state, 
                    "Input Code",
                    state.keybind_editor_builder.input_code.as_ref(),
                    Message::KeybindEditorSelectInputCode
                )
                .size(KEYBIND_TEXT_SIZE)
            )
            .padding(NEWKEYBIND_COMBO_PADDING),

            row![
                container(
                    combo_box(
                        &state.keybind_editor_source_list_state, 
                        "Source",
                        state.keybind_editor_builder.source.as_ref(),
                        Message::KeybindEditorSelectSource
                    )
                    .size(KEYBIND_TEXT_SIZE),
                )
                .padding( iced::padding::horizontal(NEWKEYBIND_COMBO_PADDING)),
            
                container(
                    combo_box(
                        &state.keybind_editor_keycode_list_state, 
                        match state.keybind_editor_builder.source {
                            None => "<- Select source",
                            _ => "Keycode",
                        },
                        state.keybind_editor_builder.keycode.as_ref(),
                        Message::KeybindEditorSelectKeycode
                    )
                    .size(KEYBIND_TEXT_SIZE),
                )
                .width(Length::Fill),
            
                container(row![
                    button(icon(Icon::AddCircle).style(match state.keybind_editor_builder.clone().into() {
                            (Some(_), Some(_), Some(_), Some(_)) => text::success,
                            _ => text::default,
                        })
                        .size(NEWKEYBIND_BUTTON_ICON_SIZE))
                        .on_press_maybe( match state.keybind_editor_builder.clone().into() {
                            (Some(_), Some(_), Some(_), Some(_)) => Some(Message::KeybindEditorSave),
                            _ => None,
                        })
                    .style(styling::button_transparent),
                    space().width(NEWKEYBIND_BUTTON_PADDING),
                    button(icon(Icon::Cancel).style(text::danger).size(NEWKEYBIND_BUTTON_ICON_SIZE))
                    .on_press(Message::KeybindEditorCancel)
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