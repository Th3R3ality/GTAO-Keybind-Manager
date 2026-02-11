pub mod keybind_manager;
pub mod gui;
pub mod keycode;
pub mod input;
pub mod screen;
pub mod asset;
pub mod profile;
pub mod styling;

use std::{
    fs,
    io::{
        Read, 
        Write
    }, 
    path::PathBuf
};

use rfd::FileDialog;
use dirs::config_local_dir;

pub fn main() {
    let mut config = Config::new();
    
    let res = config.try_create_and_read();
    match res {
        Err(err) => println!("ERR: try_create_and_read: {}", err),
        Ok(_ok) => {
            println!("Ok created/read config");
            println!("{:?}", config);
        },
    }

    if config.profiles_path.is_none() {
        config.profiles_path = FileDialog::new()
        .set_title("Rockstar Games > GTA/Enhanced > Profiles")
        .pick_folder();
    }

    let Some(profiles_folder) = config.profiles_path.clone() else { return };
    
    if let Err(err) = config.try_save() {
        println!("{}", err);
        return;
    }

    let state = keybind_manager::State::new(&profiles_folder);

    let _res = gui::run(state);
}


const CONFIG_VERSION: &'static str = "VERSION=";
const CONFIG_PROFILES_DIR: &'static str = "PROFILES_DIR=";
#[derive(Debug, Default)]
struct Config {
    pub version: Option<String>,
    pub profiles_path: Option<PathBuf>,
}
impl Config {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn try_get_parent_dir() -> Option<PathBuf> {
        match config_local_dir() {
            Some(dir) => {
                println!("INFO: try_get_parent_dir: {}", dir.clone().into_os_string().to_string_lossy());
                Some(dir)
            },
            None => None,
        }
    }
    pub fn try_get_or_create_config_dir() -> Option<PathBuf> {
        if let Some(dir) = Self::try_get_config_dir() { 
            println!("SUCC: try_get_or_create_config_dir: try_get_config_dir"); 
            return Some(dir)
        }
        let Some(dir) = Self::try_get_config_dir_unchecked() else { 
            println!("ERR: try_get_or_create_config_dir: try_get_config_dir_unchecked"); 
            return None 
        };
        let res = fs::create_dir(&dir);
        match res {
            Ok(_) => Some(dir),
            Err(_) => {
                println!("ERR: try_get_or_create_config_path: create_dir");
                None
            },
        }
    }
    pub fn try_get_or_create_config_path() -> Option<PathBuf> {
        let Some(_) = Self::try_get_or_create_config_dir() else { 
            println!("ERR: try_get_or_create_config_path: try_get_or_create_config_dir");
            return None
        };
        let Some(config_path) = Self::try_get_config_path_unchecked() else {
            println!("ERR: try_get_or_create_config_path: try_get_config_path_unchecked");
            return None
        };

        if let Err(err) = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&config_path)
        { 
            println!("ERR: try_get_or_create_config_path: OpenOptions: {}", err);
            return None
        };
        
        Some(config_path)
    }
    // pub fn try_create_config_dir() -> std::result::Result<(),()> {
    //     let Some(config_dir) = Self::try_get_config_dir() else { return Err(()) };
    //     if !config_dir.exists() {
    //         if let Err(_) = fs::create_dir(&config_dir) { return Err(()) }
    //     }
    //     Ok(())
    // }
    pub fn try_get_config_dir_unchecked() -> Option<PathBuf> {
        match Self::try_get_parent_dir().map(|dir|{
            println!("INFO: try_get_config_dir_unchecked: dir: {}", dir.clone().to_string_lossy());
            dir.join(".keybindmanager")
        }) {
            Some(dir) => {
                println!("INFO: try_get_config_dir_unchecked: try_get_parent_dir: {}", dir.clone().into_os_string().to_string_lossy());
                Some(dir)
            }
            _ => None,
        }
    }
    pub fn try_get_config_dir() -> Option<PathBuf> {
        let Some(dir) = Self::try_get_config_dir_unchecked() else { 
            println!("ERR: try_get_config_dir: try_get_config_dir_unchecked");
            return None 
        };
        match (dir.exists(), dir.is_dir()) {
            (true, true) => Some(dir),
            _ => {
                println!("ERR: try_get_config_dir: (dir.exists(), dir.is_dir()) == ({},{})",dir.exists().to_string(), dir.is_dir().to_string());
                None
            },
        }
    }
    pub fn try_get_config_path_unchecked() -> Option<PathBuf> {
        match Self::try_get_config_dir_unchecked().map(|dir| dir.join("config.txt")) {
            Some(path) => {
                println!("INFO: try_get_config_path_unchecked: try_get_config_dir_unchecked: {}", path.clone().into_os_string().to_string_lossy());
                Some(path)
            }
            _ => None,
        }
    }
    // pub fn try_get_config_path() -> Option<PathBuf> {
    //     let Some(dir) = Self::try_get_config_dir() else { return None };
    //     let path = dir.join("/config.txt");
    //     match (path.exists(), path.is_file()) {
    //         (true, true) => Some(path),
    //         _ => None,
    //     }
    // }

    pub fn try_save(&mut self) -> std::result::Result<(),String> {
        let Some(config_path) = Self::try_get_or_create_config_path() else { return Err("Error saving: getting config path".to_string()) };

        let Ok(mut config_file) = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(config_path) else { return Err("Error saving: couldn't open config path".to_string()) };
        
        let _res = writeln!(config_file, "{}{}", CONFIG_VERSION, keybind_manager::VERSION);
        if let Some(profiles_path) = &self.profiles_path {
            let _res = writeln!(config_file, "{}{}", CONFIG_PROFILES_DIR, profiles_path.clone().into_os_string().to_string_lossy());
        }

        Ok(())
    }

    pub fn try_create_and_read(&mut self) -> std::result::Result<(),String> {

        let Some(config_path) = Self::try_get_or_create_config_path() else { 
            return Err("ERR: try_create_and_read: try_get_or_create_config_path".to_owned())
        };

        let Ok(mut config_file) = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&config_path)
        else { 
            return Err("ERR: try_create_and_read: cant open config file".to_owned())
        };

        let mut config_string: String = "".to_owned();
        let Ok(_) = config_file.read_to_string(&mut config_string) else { return Err("ERR: try_create_and_read: can't read_to_string".to_owned()) };

        config_string.lines()
        .map(String::from)
        .for_each(|line| {
            line.strip_prefix(CONFIG_VERSION).map(|var| self.version = Some(var.to_string()));
            line.strip_prefix(CONFIG_PROFILES_DIR).map(PathBuf::from).map(|path| if path.exists() { self.profiles_path = Some(path) });
        });

        Ok(())
    }
}