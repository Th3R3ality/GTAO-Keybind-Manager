use std::{
    cell::RefCell, fmt::{
        self,
    }, fs::{
        self,
        OpenOptions,
    }, io::Write, path::{Path, PathBuf}, rc::Rc
};

use chrono::Local;

use crate::{
    input,
    keycode,
};

pub type KeybindInput = usize;
pub type KeybindSource = usize;
pub type KeybindKeycode = usize;
pub type KeybindId = usize;
pub type Keybind = (KeybindInput, KeybindSource, KeybindKeycode, KeybindId);
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Profile {
    pub name: String,
    pub path: PathBuf,
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
        Rc::new(RefCell::new(Profile{
            name: name,
            path: xml_path,
            ..Default::default()
        }))
    }

    pub fn add_keybind(&mut self, keybind: Keybind) {
        self.keybinds.push(keybind);
        self.modified = true;
    }

    pub fn remove_keybind(&mut self, id: KeybindId) {
        self.keybinds.retain(|keybind| keybind.3 != id);
        self.modified = true;
    }

    /// writes profile to file and marks it as p.modified = false on success
    pub fn write_xml(&mut self) -> std::result::Result<(), (String, String)> {

        let parent = self.path.parent().unwrap_or(Path::new(""));
        let filename = self.path
            .file_stem()
            .unwrap()
            .to_string_lossy();

        let suffix = Local::now().format("_%d_%m_%y_%H%M").to_string();

        let mut counter = 0;
        let mut backup_path = parent.join(format!("{filename}{suffix}")).with_extension("backup");
        while backup_path.exists() {
            counter += 1;
            backup_path = parent.join(format!("{filename}{suffix}__{counter}")).with_extension("backup");
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
                    let (input, category, keycode) = (
                        input::from_index(keybind.0),
                        keycode::category_from_index(keybind.1),
                        keycode::keycode_from_indexes(keybind.1, keybind.2)
                    );

                    let _ = file.write(format!("    {}\n", opening(USER_ITEM)).as_bytes());
                    let _ = file.write(format!("      {}{}{}\n",
                        opening(USER_INPUT), input, closing(USER_INPUT)).as_bytes());
                    let _ = file.write(format!("      {}{}{}\n",
                        opening(USER_SOURCE), category.0, closing(USER_SOURCE)).as_bytes());
                    let _ = file.write(format!("      {}\n", opening(USER_PARAMS)).as_bytes());       
                    let _ = file.write(format!("        {}{}{}\n",
                        opening(USER_ITEM), keycode.0, closing(USER_ITEM)).as_bytes());
                    let _ = file.write(format!("      {}\n", closing(USER_PARAMS)).as_bytes());       
                    
                    let _ = file.write(format!("    {}\n", closing(USER_ITEM)).as_bytes());       
                }


                let _ = file.write(format!("  {}\n", closing(USER_MAPPINGS)).as_bytes());
                let _ = file.write(format!("{}", closing(USER_WRAPPER)).as_bytes());
            },
        }
        
        self.modified = false;
        Ok(())
    }

    pub fn load_xml(&mut self) -> std::result::Result<(), (String, String)> {
        
        let res = fs::read_to_string(&self.path);

        let lines: Vec<String> = match res {
            Ok(string) => string.lines().map(String::from).collect(),
            Err(_) => return Err(("".to_owned(),"".to_owned())),
        };

        let mut iter= lines.iter().enumerate();
        
        while let Some((_, line)) = iter.next() {
            if line.trim() == &opening(USER_MAPPINGS) {
                break;
            }
        }
        
        let mut keybind_collector: Vec<Keybind> = Vec::new();
        let mut keybind_builder: (Option<usize>, Option<usize>, Option<usize>) = (None, None, None);
        let mut keybind_builder_names: (Option<String>, Option<String>, Option<String>) = (None, None, None);
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
                    keybind_builder = (None, None, None);
                    keybind_builder_names = (None, None, None);
                },
                s if s.starts_with(&opening(USER_INPUT)) => {
                    let Some(value) = extract_xml_value(trimmed, USER_INPUT) else {
                        return Err((format!("None value while extracting '{}'", USER_INPUT), "".to_owned()))
                    };
                    keybind_builder.0 = match input::get_index(&value) {
                        Some(index) => {
                            keybind_builder_names.0 = Some(value);
                            Some(index)
                        },
                        None => None,
                    };

                },
                s if s.starts_with(&opening(USER_SOURCE)) => {
                    let Some(value) = extract_xml_value(trimmed, USER_SOURCE) else {
                        return Err((format!("None value while extracting '{}'", USER_SOURCE), "".to_owned()))
                    };
                    keybind_builder.1 = match keycode::get_category_index(&value) {
                        Some(index) => {
                            keybind_builder_names.1 = Some(value);
                            Some(index)
                        },
                        None => None,
                    };
                },
                s if s.starts_with(&opening(USER_PARAMS)) => {
                    match iter.next(){
                        Some((_, item_line)) => {
                            let Some(value) = extract_xml_value(item_line.trim(), USER_ITEM) else {
                                return Err((format!("None value while extracting '{}'", USER_ITEM), "".to_owned()))
                            };

                            let category_index = match keybind_builder.1 {
                                Some(category) => category,
                                None => {
                                    return Err(("Trying to get keycode without category".to_owned(), builder_to_string(keybind_builder_names)))
                                }
                            };
                            keybind_builder.2 = match keycode::get_keycode_index_with_category_index(
                                    category_index,
                                    &value) {
                                Some(index) => {
                                    keybind_builder_names.2 = Some(value);
                                    Some(index)
                                },
                                None => None,
                            };
                        },
                        None => break,
                    }
                    match iter.next() {
                        Some((_, params_end)) if params_end.trim() == &closing(USER_PARAMS) => (),
                        None | _ => break,
                    }
                },
                s if s.starts_with(&closing(USER_ITEM)) => {
                    match keybind_builder {
                        (Some(input), Some(category), Some(keycode)) => {
                            keybind_collector.push((input, category, keycode, self.new_id()));
                        },
                        _ => {
                            return Err(("Incomplete Keybind".to_owned(), builder_to_string(keybind_builder_names)))
                        },
                    }
                    keybind_builder = (None, None, None);
                },
                _ => break,
            }
        }

        self.keybinds = keybind_collector;
        self.modified = false;
        Ok(())
    }

    pub fn new_id(&mut self) -> KeybindId {
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

fn builder_to_string<T: ToString>(builder: (Option<T>, Option<T>, Option<T>)) -> String {
    let input_string = match builder.0 {
        Some(input) => input.to_string(),
        None => "None".to_owned(),
    };
    let category_string = match builder.1 {
        Some(category) => category.to_string(),
        None => "None".to_owned(),
    };
    let keycode_string = match builder.2 {
        Some(keycode) => keycode.to_string(),
        None => "None".to_owned(),
    };

    return format!("builder: {} {} {}", input_string, category_string, keycode_string)
}