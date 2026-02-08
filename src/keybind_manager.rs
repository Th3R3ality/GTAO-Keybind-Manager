use std::path::PathBuf;
use std::fs;
use std::io::{
    Error, 
    ErrorKind
};

use crate::profile::Keybind;
use crate::{
    screen::{
        Screen,
        keybindings,
        about,
    },
    profile::{
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
    pub dummy_new_keybind: Keybind,
}

#[derive(Debug, Clone)]
pub enum Message {
    ScreenSelected(Screen),
    Keybindings(keybindings::Message),
    About(about::Message),
}

impl State {
    pub fn new(profiles_folder: &PathBuf) -> State {
        let mut new: State = State{
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

