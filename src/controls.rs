use crate::keycode;
use crate::input;

use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::io::{Error, ErrorKind};

static USER_XML_PATHS: OnceLock<Vec<(String, PathBuf)>> = OnceLock::new();

fn init_user_xmls(root: &PathBuf) -> std::io::Result<()> {
    let mut xmls = Vec::new();

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let folder = entry.path();
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
                xmls.push((name, user_xml));
            }
        }
    }

    if xmls.is_empty() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "no control/user.xml files found in profiles directory",
        ));
    }    

    USER_XML_PATHS
        .set(xmls)
        .map_err(|_| Error::new(ErrorKind::AlreadyExists, "USER_XML_PATHS already initialized"))?;

    Ok(())
}

pub fn load_profiles(root: &PathBuf) -> std::io::Result<()> {
    init_user_xmls(root)?;

    let profiles = USER_XML_PATHS.get().unwrap();

    for (name, xml) in profiles {
        println!("profile: {}, xml: {:?}", name, xml);
    }
    Ok(())
}

fn get_default_for_input(input_code: &str) -> &&(&str, &str, &str) {
    match input_code {
        input::switch_camera::NEXT_CAMERA => &keycode::keyboard::KEY_V,
        _ => &keycode::NONE,
    }
}