use std::path::PathBuf;
use std::fs;
use std::io::{
    Error, 
    ErrorKind
};

use iced::widget::combo_box;

use crate::{
    input,
    keycode,
    screen::{
        Screen,
        keybindings,
        about,
    },
    profile::{
        ProfileRef,
        Profile,
    },
    keybind::*,
    gui::{
        Prompt,
    },
};

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");


#[derive(Debug, Clone, Default)]
pub struct State {
    //universal
    pub screen: Screen,
    pub prompt: Option<Prompt>,

    pub input_categories: Vec<KeybindInputCategory>,
    pub input_codes: Vec<Vec<KeybindInputCode>>,
    pub key_sources: Vec<KeybindSource>,
    pub keycodes: Vec<Vec<KeybindKeycode>>,

    // keybindings screen
    pub available_profiles: Vec<ProfileRef>,
    pub selected_profile: Option<ProfileRef>,
    pub selected_profile_name: Option<String>,
    pub search_string: String,
    
    
    // // modify keybind prompt (used for new keybind aswell)
    pub keybind_editor_builder: KeybindBuilder,
    pub keybind_editor_mode: keybindings::EditorMode,
    pub keybind_editor_input_category_list_state: combo_box::State<KeybindInputCategory>,
    pub keybind_editor_input_code_list_state: combo_box::State<KeybindInputCode>,
    pub keybind_editor_source_list_state: combo_box::State<KeybindSource>,
    pub keybind_editor_keycode_list_state: combo_box::State<KeybindKeycode>,

}

#[derive(Debug, Clone)]
pub enum Message {
    Ignore,
    Focused,
    ScreenSelected(Screen),
    Keybindings(keybindings::Message),
    About(about::Message),
}

impl State {
    pub fn new(profiles_folder: &PathBuf) -> State {
    
        let mut new: State = State {
            input_categories: input::get_all_categories(),
            key_sources: keycode::get_all_sources(),
            ..Default::default()
        };

        for (index, category) in new.input_categories.iter().enumerate() {
            assert_eq!(index, category.index);
            assert_eq!(index, new.input_codes.len());
            new.input_codes.push(input::get_all_input_codes_for_category(category));
        }

        for (index, source) in new.key_sources.iter().enumerate() {
            assert_eq!(index, source.index);
            assert_eq!(index, new.keycodes.len());
            new.keycodes.push(keycode::get_all_keycodes_for_source(source));
        }

        new.keybind_editor_input_category_list_state = combo_box::State::new(new.input_categories.clone());
        new.keybind_editor_source_list_state = combo_box::State::new(new.key_sources.clone());

        let _ = new.discover_profiles(profiles_folder);

        return new
    }
    pub fn title(&self) -> String {
        format!("GTAO Keybind Manager {}", VERSION)
    }
    fn discover_profiles(&mut self, profiles_folder: &PathBuf) -> std::io::Result<()>{
        for entry in fs::read_dir(profiles_folder)? {
            let folder = entry?.path();
            if folder.is_dir() {
                let name = folder
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| {
                        Error::new(ErrorKind::InvalidData, "invalid folder name")
                    })?
                    .to_owned();

                let user_xml = folder.join("control").join("user.xml");

                if user_xml.exists() {
                    self.available_profiles.push(Profile::new(name, user_xml));
                }
            }
        }
        Ok(())
    }
}

