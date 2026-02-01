use std::{
    path::{
        PathBuf,
    },
    fs::{
        self,
        OpenOptions,
    },
    fmt::{
        self,
    },
    io::{
        Write,
    }
};

use crate::{
    input,
    keycode,
};

type KeybindId = usize;
type Keybind = (usize, usize, usize, KeybindId);

#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub name: String,
    pub path: PathBuf,
    pub keybinds: Option<Vec<Keybind>>,
    pub id_counter: KeybindId,
}

const USER_HEADER: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>";
const USER_WRAPPER: &str = "rage__ControlInput__ControlSettings";

const USER_MAPPINGS: &str = "Mappings";
const USER_INPUT: &str = "Input";
const USER_SOURCE: &str = "Source";
const USER_PARAMS: &str = "Parameters";
const USER_ITEM: &str = "Item";

impl Profile{
    pub fn new(name: String, xml_path: PathBuf) -> Profile{
        Profile{
            name: name,
            path: xml_path,
            keybinds: None,
            id_counter: 0,
        }
    }

    pub fn write_xml(&self) -> std::result::Result<(), (String, String)> {

        let path = self.path.with_added_extension("txt");
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

                if let Some(keybinds) = &self.keybinds {

                    for keybind in keybinds {
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
                }


                let _ = file.write(format!("  {}\n", closing(USER_MAPPINGS)).as_bytes());
                let _ = file.write(format!("{}", closing(USER_WRAPPER)).as_bytes());
            },
        }
        
        Ok(())
    }

    pub fn load_xml(&mut self) -> std::result::Result<(), (String, String)> {
        let lines: Vec<String> = fs::read_to_string(&self.path)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

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
                            keybind_collector.push((input, category, keycode, self.id_counter));
                            self.id_counter += 1;
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

        self.keybinds = Some(keybind_collector);
        Ok(())
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