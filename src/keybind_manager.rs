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
        Keybind,
        ProfileRef,
        Profile,
    },
    gui::{
        Prompt,
    },
};

pub const VERSION: &str = &"0.0";

#[derive(Debug, Clone, Default)]
pub struct State {
    //universal
    pub screen: Screen,
    pub prompt: Option<Prompt>,

    // keybindings screen
    pub available_profiles: Vec<ProfileRef>,
    pub selected_profile: Option<ProfileRef>,
    pub search_string: String,
    // // new keybind prompt
    pub dummy_new_keybind: Keybind,

    pub input_list_state_new_keybind: combo_box::State<String>,
    pub selected_input_new_keybind: Option<String>,

    pub source_list_state_new_keybind: combo_box::State<String>,
    pub selected_source_new_keybind: Option<String>,

    pub keycode_list_state_new_keybind: combo_box::State<String>,
    pub selected_keycode_new_keybind: Option<String>,

}

#[derive(Debug, Clone)]
pub enum Message {
    ScreenSelected(Screen),
    Keybindings(keybindings::Message),
    About(about::Message),
}

impl State {
    pub fn new(profiles_folder: &PathBuf) -> State {

        let input_items: Vec<String> = input::INPUT_CODES
            .iter()
            .flat_map(|input_category| 
                input_category.iter()
                    .enumerate()
                    .filter(|(index, _)| index > &0).map(|(_, elem)| elem)
                )
            .map(|input_name| input_name.to_string())
            .collect();

        let source_items: Vec<String> = keycode::KEYCODES
            .iter()
            .map(|category| category.first().unwrap().0.to_string())
            .collect();
        let mut new: State = State {
            input_list_state_new_keybind: combo_box::State::new(input_items),
            source_list_state_new_keybind: combo_box::State::new(source_items),
            ..Default::default()
        };

        let _ = new.discover_profiles(profiles_folder);

        return new
    }
    pub fn title(&self) -> String {

        match self.selected_profile.as_ref() {
            None => format!("GTAO Keybind Manager {} by Reality", VERSION).to_owned(),
            Some(profile_ref) => {
                format!("GTAO Keybind Manager {} by Reality | Profile: {}", VERSION, profile_ref.borrow().name).to_owned()
            },
        }
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

