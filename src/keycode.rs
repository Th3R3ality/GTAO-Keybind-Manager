//pub const NONE: &(&str,&str,&str) = &("KEY_NONE", "None", "Unbound"); i think its null and not this
//pub const NULL: &'static(&'static str,&'static str,&'static str) = &("KEY_NULL", "Null", "Unbound");
// KEY_NULL is appended to every category (it may only work/is used with "keyboard")

pub mod keyboard;
pub mod mouse_button;
pub mod mouse_wheel;

pub const KEYCODES: &[&[&(&str,&str,&str)]] = &[
    keyboard::ALL,
    mouse_button::ALL,
    mouse_wheel::ALL,
];

pub fn category_from_index(index: usize) -> Option<&'static (&'static str,&'static str,&'static str)>{
    if let Some(category) = KEYCODES.iter().nth(index) {
        return category.iter()
            .next()
            .copied()
    }
    return None
}
pub fn get_category_index(category_name: &str) -> Option<usize> {
    for (category_index, category) in KEYCODES.iter().enumerate() {
        if let Some(value) = category.iter().next() {
            if value.0 != category_name {
                continue;
            };
            return Some(category_index)
        };
    }
    return None
}

pub fn keycode_from_indexes(category_index: usize, keycode_index: usize)
        -> Option<&'static(&'static str, &'static str, &'static str)> {
    if let Some(category) = KEYCODES.iter().nth(category_index) {
        return category.iter()
            .nth(keycode_index)
            .copied()
    }

    return None
}

pub fn get_keycode_index_with_category_index(category_index: usize, keycode_name: &str) -> Option<usize> {
    if let Some(category) = KEYCODES.iter().nth(category_index) {
        for (index, keycode) in category.iter().enumerate() {
            if keycode.0 == keycode_name{
                return Some(index)
            }
        }
    }
    return None
}

pub fn get_keycode_index_with_category(category_name: &str, keycode_name: &str) -> Option<usize> {
    for category in KEYCODES.iter() {
        if let Some(value) = category.iter().next() {
            if value.0 != category_name {
                continue;
            };
            for (keycode_index, keycode) in category.iter().enumerate(){
                if keycode.0 != keycode_name {
                    continue;
                };
                return Some(keycode_index)
            }
        };
    }
    return None
}