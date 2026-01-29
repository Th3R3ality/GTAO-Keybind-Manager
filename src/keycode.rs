pub const NONE: &(&str,&str,&str) = &("KEY_NONE", "None", "Unbound");

pub mod keyboard;
pub mod mouse_button;
pub mod mouse_wheel;

pub const KEYCODES: &[&[&(&str,&str,&str)]] = &[
    keyboard::ALL,
    mouse_button::ALL,
    mouse_wheel::ALL,
];