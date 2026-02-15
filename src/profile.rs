use std::{
    cell::RefCell, fmt::{
        self,
    }, fs::{
        self,
        OpenOptions,
    }, io::Write, path::PathBuf, rc::Rc
};

use chrono::Local;

use crate::{
    input,
    keycode,
    keybind::*,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Profile {
    pub name: String,
    pub filename: String,
    pub path: PathBuf,
    pub parent_path: PathBuf,
    pub latest_path: PathBuf,
    pub keybinds: Vec<Keybind>,
    pub id_counter: KeybindId,
    pub modified: bool,
}

pub type ProfileRef = Rc<RefCell<Profile>>;

const USER_HEADER: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>";
const USER_WRAPPER: &str = "rage__ControlInput__ControlSettings";

const USER_MAPPINGS: &str = "Mappings";
const USER_INPUT: &str = "Input";
const USER_SOURCE: &str = "Source";
const USER_PARAMS: &str = "Parameters";
const USER_ITEM: &str = "Item";

impl Profile{
    pub fn new(name: String, xml_path: PathBuf) -> ProfileRef {

        let filename = xml_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();

        let parent_path = xml_path
        .parent()
        .unwrap()
        .to_owned();

        let latest_path = parent_path.join("latest.xml");

        Rc::new(RefCell::new(Profile{
            name: name,
            filename: filename,
            path: xml_path,
            parent_path: parent_path,
            latest_path: latest_path,
            ..Default::default()
        }))
    }

    pub fn modify_or_add_keybind(&mut self, keybind: Keybind) {
        match self.keybinds.iter_mut().find(|_keybind| _keybind.id == keybind.id) {
            None => self.add_keybind(keybind),
            Some(_keybind) => {
                *_keybind = keybind;
            }
        }
        self.modified = true;        
    }

    pub fn add_keybind(&mut self, keybind: Keybind) {
        if self.keybinds.iter().find(|_keybind| _keybind.id == keybind.id).is_some() {
            panic!("Attempted to add a keybind with an id that already exists");
        }
        self.keybinds.push(keybind);
        self.modified = true;
    }

    pub fn remove_keybind(&mut self, id: &KeybindId) {
        self.keybinds.retain(|keybind| &keybind.id != id);
        self.modified = true;
    }

    pub fn get_keybind_as_builder(&self, id: &KeybindId) -> KeybindBuilder {
        match self.keybinds.iter().find(|keybind| &keybind.id == id) {
            Some(keybind) => keybind.to_builder(),
            None => KeybindBuilder::new_empty(),
        }
    }

    /// writes profile to file and marks it as p.modified = false on success
    pub fn write_xml(&mut self) -> std::result::Result<(), (String, String)> {

        let suffix = Local::now().format("_%d_%m_%y_%H%M").to_string();

        let mut counter = 0;
        let mut backup_path = self.parent_path.join(format!("{}{suffix}", self.filename)).with_extension("backup");
        while backup_path.exists() {
            counter += 1;
            backup_path = self.parent_path.join(format!("{}{suffix}__{counter}", self.filename)).with_extension("backup");
        }

        let res = fs::copy(&self.path, &backup_path);
        if let Err(err) = res {
            return Err((format!("couldn't backup profile: {}", self.name), err.to_string()))
        }

        let path = &self.path;
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path.clone());

        match file {
            Err(_err) => return Err(("Can't Open file".to_owned(), format!("{}", path.display()))),
            Ok(mut file) => {

                let _ = file.write(format!("{}\n\n", USER_HEADER).as_bytes());
                let _ = file.write(format!("{}\n", opening(USER_WRAPPER)).as_bytes());
                let _ = file.write(format!("  {}\n", opening(USER_MAPPINGS)).as_bytes());


                for keybind in &self.keybinds {
                    let _ = file.write(format!("    {}\n", opening(USER_ITEM)).as_bytes());
                    let _ = file.write(format!("      {}{}{}\n",
                        opening(USER_INPUT), keybind.input.code.name, closing(USER_INPUT)).as_bytes());
                    let _ = file.write(format!("      {}{}{}\n",
                        opening(USER_SOURCE), keybind.key.source.name, closing(USER_SOURCE)).as_bytes());
                    let _ = file.write(format!("      {}\n", opening(USER_PARAMS)).as_bytes());       
                    let _ = file.write(format!("        {}{}{}\n",
                        opening(USER_ITEM), keybind.key.keycode.name, closing(USER_ITEM)).as_bytes());
                    let _ = file.write(format!("      {}\n", closing(USER_PARAMS)).as_bytes());       
                    
                    let _ = file.write(format!("    {}\n", closing(USER_ITEM)).as_bytes());       
                }


                let _ = file.write(format!("  {}\n", closing(USER_MAPPINGS)).as_bytes());
                let _ = file.write(format!("{}", closing(USER_WRAPPER)).as_bytes());
            },
        }

        let res = fs::copy(&self.path, &self.latest_path);
        if let Err(err) = res {
            return Err((format!("couldn't create user_latest.xml: {}", self.name), err.to_string()))
        }
        
        self.modified = false;
        Ok(())
    }

    pub fn load(&mut self) -> std::result::Result<(), (String, String)> {
        let res = Self::load_xml(&self.path);
        match res {
            Err(err) => return Err(err),
            Ok(mut keybinds) => {
                for keybind in &mut keybinds {
                    if keybind.id == 0 {
                        keybind.id = self.next_id();
                    }
                }
                self.keybinds = keybinds;
            }
        }
        self.modified = false;
        Ok(())
    }
    pub fn restore(&mut self) -> std::result::Result<(), (String, String)> {
        let res = Self::load_xml(&self.latest_path);
        match res {
            Err(err) => return Err(err),
            Ok(mut keybinds) => {
                for keybind in &mut keybinds {
                    if keybind.id == 0 {
                        keybind.id = self.next_id();
                    }
                }
                self.keybinds = keybinds;
            }
        }
        self.modified = false;
        Ok(())
    }
    pub fn verify_latest(&self) -> bool {
        let Ok( mut current) = Self::load_xml( &self.path ) else { return true };
        current.sort_by_key(|keybind| (
            keybind.input.category.name,
            keybind.input.code.name,
            keybind.key.source.name,
            keybind.key.keycode.name
        ));

        let Ok( mut latest) = Self::load_xml( &self.latest_path ) else { return true };
        latest.sort_by_key(|keybind| (
            keybind.input.category.name,
            keybind.input.code.name,
            keybind.key.source.name,
            keybind.key.keycode.name
        ));

        latest == current
    }
    fn load_xml(path: &PathBuf) -> std::result::Result<Vec<Keybind>, (String, String)> {
        let res = fs::read_to_string(path);

        let lines: Vec<String> = match res {
            Ok(string) => string.lines().map(String::from).collect(),
            Err(_) => return Err(("".to_owned(),"".to_owned())),
        };

        let mut iter = lines.iter().enumerate();
        
        while let Some((_, line)) = iter.next() {
            if line.trim() == &opening(USER_MAPPINGS) {
                break;
            }
        }
        
        let mut keybind_collector: Vec<Keybind> = Vec::new();
        let mut keybind_builder: KeybindBuilder = KeybindBuilder::default();
        while let Some((_, line)) = iter.next()
        {
            let trimmed = line.trim();
            if trimmed == &closing(USER_MAPPINGS) {
                break;
            }
            //println!("line: {}", line);
            //println!("trimmed: {}", trimmed);
            match trimmed {
                s if s.starts_with(&opening(USER_ITEM)) => {
                    keybind_builder.empty();
                },
                s if s.starts_with(&opening(USER_INPUT)) => {
                    let Some(value) = extract_xml_value(trimmed, USER_INPUT) else {
                        return Err(("load_xml:".to_owned(), format!("None value while extracting '{}'", USER_INPUT)))
                    };

                    let Some(input) = input::get_indices(&value) else { 
                        return Err(("load_xml:".to_owned(), format!("Couldn't find'{}'", value)))
                    };
                    keybind_builder.input_category = Some(input.category);
                    keybind_builder.input_code = Some(input.code);
                },
                s if s.starts_with(&opening(USER_SOURCE)) => {
                    let Some(value) = extract_xml_value(trimmed, USER_SOURCE) else {
                        return Err(("load_xml:".to_owned(), format!("None value while extracting '{}'", USER_SOURCE)))
                    };
                    keybind_builder.source = keycode::get_source(&value);
                },
                s if s.starts_with(&opening(USER_PARAMS)) => {
                    let Some((_, item_line)) = iter.next() else {
                        return Err(("load_xml:".to_owned(), format!("Unexpected eof while reading '{}'", USER_ITEM)))
                    };

                    let Some(value) = extract_xml_value(item_line.trim(), USER_ITEM) else {
                        return Err(("load_xml:".to_owned(), format!("None value while extracting '{}'", USER_ITEM)))
                    };

                    let Some(source) = keybind_builder.source.as_ref() else {
                        return Err(("load_xml:".to_owned(), format!("Trying to get keycode without source '{:?}'", keybind_builder)))
                    };

                    keybind_builder.keycode = keycode::get_keycode(source, &value);

                    let Some((_, closing_params_line)) = iter.next() else {
                        return Err(("load_xml:".to_owned(), format!("Unexpected eof while expecting '{}'", &closing(USER_PARAMS))))
                    };

                    if closing_params_line.trim() != &closing(USER_PARAMS) {
                        return Err(("load_xml:".to_owned(),
                        format!("Expected '{}' but got '{}'", &closing(USER_PARAMS), closing_params_line.trim())))
                    }
                },
                s if s.starts_with(&closing(USER_ITEM)) => {
                    match keybind_builder.clone().into() {
                        (Some(input_category), Some(input_code), Some(source), Some(keycode)) => {
                            keybind_collector.push(Keybind::new_from_primitives(
                                input_category,
                                input_code,
                                source,
                                keycode,
                                0));
                        }
                        _ => {
                            return Err(("load_xml:".to_owned(), format!("keybind_builder incomplete {:?}", keybind_builder)));
                        },
                    };
                },
                _ => break,
            }
        }

        Ok(keybind_collector)
    }

    pub fn next_id(&mut self) -> KeybindId {
        self.id_counter += 1;
        self.id_counter
    }
}


impl fmt::Display for Profile{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
fn opening(tag: &str) -> String{
    format!("<{}>", tag)
}
fn closing(tag: &str) -> String{
    format!("</{}>", tag)
}
fn extract_xml_value(enclosed_value: &str, tag: &str) -> Option<String> {
    let res = enclosed_value
        .strip_prefix(&opening(tag))?
        .strip_suffix(&closing(tag));

    match res{
        Some(res) => Some(res.to_owned()),
        _ => None,
    }
}

// fn builder_to_string<T: ToString>(builder: (Option<T>, Option<T>, Option<T>)) -> String {
//     let input_string = match builder.0 {
//         Some(input) => input.to_string(),
//         None => "None".to_owned(),
//     };
//     let category_string = match builder.1 {
//         Some(category) => category.to_string(),
//         None => "None".to_owned(),
//     };
//     let keycode_string = match builder.2 {
//         Some(keycode) => keycode.to_string(),
//         None => "None".to_owned(),
//     };

//     return format!("builder: {} {} {}", input_string, category_string, keycode_string)
// }