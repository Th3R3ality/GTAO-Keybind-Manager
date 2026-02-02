use std::path::PathBuf;
use std::fs;
use std::io::{
    Error, 
    ErrorKind
};

use crate::{
    screen::{
        Screen,
        keybindings,
        about,
    },
    profile::Profile,
};


pub const VERSION: &str = &"0.1";

#[derive(Debug, Clone)]
pub struct State {
    pub available_profiles: Vec<Profile>,
    pub current_profile: Option<Profile>,
    pub screen: Screen,
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
            available_profiles: Vec::new(),
            current_profile: None,
            screen: Screen::Landing,
        };

        let _ = new.discover_profiles(profiles_folder);

        return new
    }
    pub fn title(&self) -> String {
        let suffix = self
            .current_profile
            .as_ref()
            .map(|p| format!(" | Current Profile: {}", p.name))
            .unwrap_or_default();
        format!("GTAO Keybind Manager {} by Reality{}", VERSION, suffix)
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

