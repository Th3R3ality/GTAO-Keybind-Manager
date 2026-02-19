//pub const NONE: &(&str,&str,&str) = &("KEY_NONE", "None", "Unbound"); i think its null and not this
//pub const NULL: &'static(&'static str,&'static str,&'static str) = &("KEY_NULL", "Null", "Unbound");
// KEY_NULL is appended to every category (it may only work/is used with "keyboard")

use crate::keybind::*;

pub mod keyboard;
pub mod mouse_button;
pub mod mouse_wheel;

pub const KEYCODES: &[&[&(&str,&str,&str)]] = &[
    keyboard::ALL,
    mouse_button::ALL,
    mouse_wheel::ALL,
];

pub const KEYCODE_ERR_KEY: &'static (&'static str, &'static str, &'static str) = &(&"???_ERR_KEY",&"??_ERR_KEY",&"???_ERR_KEY");
pub const KEYCODE_ERR_CATEGORY: &'static (&'static str, &'static str, &'static str) = &(&"???_ERR_CATEGORY",&"??_ERR_CATEGORY",&"???_ERR_CATEGORY");

//type NameTuple = &'static (&'static str,&'static str,&'static str);

pub fn get_source(category_name: &str) -> Option<KeybindSource> {
    KEYCODES
    .iter()
    .enumerate()
    .find_map(|(index, category)|{
        if category.first()?.0 == category_name {
            let first = category.first()?;
            Some(KeybindSource {
                index,
                name: first.0,
                pretty_name: first.1,
                desc: first.2,
            })
        }
        else {
            None
        }
    })
}

pub fn get_keycode(source: &KeybindSource, keycode_name: &str) -> Option<KeybindKeycode> {
    KEYCODES
    .get(source.index)
    .and_then(|category|{
        category
        .iter()
        .enumerate()
        .find_map(|(index, keycode)|{
            if keycode.0 == keycode_name {
                Some(KeybindKeycode {
                    index,
                    name: keycode.0,
                    pretty_name: keycode.1,
                    desc: keycode.2,
                })
            }
            else {
                None
            }
        })
    })
}
pub fn from_indices(source_index: usize, keycode_index: usize) -> Option<KeybindKey> {
    KEYCODES
    .get(source_index)
    .and_then(|category| {
        let source = category.first()?;
        let keycode = category.iter().nth(keycode_index)?;
        Some(KeybindKey {
            source: KeybindSource {
                index: source_index,
                name: source.0,
                pretty_name: source.1,
                desc: source.2,
            },
            keycode: KeybindKeycode {
                index: keycode_index,
                name: keycode.0,
                pretty_name: keycode.1,
                desc: keycode.2,
            }
        })
    })
}
pub fn source_from_index(source_index: usize) -> Option<KeybindSource> {
    KEYCODES
    .get(source_index)
    .and_then(|x| x.first())
    .and_then(|source|{
        Some(KeybindSource {
            index: source_index,
            name: source.0,
            pretty_name: source.1,
            desc: source.2,
        })
    })
}
pub fn keycode_from_index(source: &KeybindSource, keycode_index: usize) -> Option<KeybindKeycode> {
    KEYCODES
    .get(source.index)
    .and_then(|category|{
        category
        .iter()
        .nth(keycode_index)
        .and_then(|keycode|{
            Some(KeybindKeycode {
                index: keycode_index,
                name: keycode.0,
                pretty_name: keycode.1,
                desc: keycode.2,
            })
        })
    })
}

pub fn get_all_sources() -> Vec<KeybindSource> {
    KEYCODES
    .iter()
    .enumerate()
    .map(|(index, source)| {
        let first = source.first().unwrap();
        KeybindSource {
            index,
            name: first.0,
            pretty_name: first.1,
            desc: first.2,
        }
    })
    .collect()
}

pub fn get_all_keycodes_for_source(source: &KeybindSource) -> Vec<KeybindKeycode> {
    KEYCODES
    .get(source.index)
    .unwrap()
    .iter()
    .enumerate()
    .filter_map(|(index, keycode)|{
        if index == 0 { return None }
        Some(KeybindKeycode {
            index,
            name: keycode.0,
            pretty_name: keycode.1,
            desc: keycode.2,
        })
    })
    .collect()
}