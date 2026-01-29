use std::fmt::{self};
use std::path::PathBuf;
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct State {
    pub available_profiles: Vec<Profile>,
    pub current_profile: Option<Profile>,
}
#[derive(Debug, Clone)]
pub enum Message {
    ProfileSelected(Profile),
}

impl State {
    pub fn new(profiles_folder: &PathBuf) -> State {
        let mut _new: State = State{
            available_profiles: Vec::new(),
            current_profile: None,
        };

        let _ = _new.discover_profiles(profiles_folder);

        _new
    }
    pub fn title(&self) -> String {
        let suffix = self
            .current_profile
            .as_ref()
            .map(|p| format!(" - Current Profile: {}", p.name))
            .unwrap_or_default();
        format!("GTAO Keybind Manager by Reality{}", suffix)
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

#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub name: String,
    pub path: PathBuf,
    pub keybinds: Option<Vec<(usize, usize, usize)>>,
}

impl Profile{
    pub fn new(name: String, xml_path: PathBuf) -> Profile{
        Profile{
            name: name,
            path: xml_path,
            keybinds: None,
        }
    }
    pub fn load_xml(){

    }
}

impl fmt::Display for Profile{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}