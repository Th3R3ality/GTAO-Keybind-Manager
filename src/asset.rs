use std::fmt;
use iced::{Font, widget::{Text, text}};

pub const ICON32: &[u8] = include_bytes!("../assets/32x.png");
pub const ICON64: &[u8] = include_bytes!("../assets/64x.png");
pub const ICON128: &[u8] = include_bytes!("../assets/128x.png");
pub const ICON256: &[u8] = include_bytes!("../assets/256x.png");

pub const ICON_FONT_DATA: &[u8] = include_bytes!("../assets/MaterialIcons-Regular.ttf");
pub const ICON_FONT: Font = Font::with_name("Material Icons");

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Icon {
    SaveAs = '\u{eb60}' as u32,
    Save = '\u{e161}' as u32,
    Help = '\u{e8fd}' as u32,
    Keyboard = '\u{f028}' as u32,
    Add = '\u{e145}' as u32,
    AddCircle = '\u{e148}' as u32,
    AddBox = '\u{e146}' as u32,
    Cancel = '\u{e5c9}' as u32,
}
impl Icon {
    pub fn as_char(self) -> char {
        char::from_u32(self as u32).expect("invalid unicode icon")
    }
}
impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}
pub fn icon(icon: Icon) -> Text<'static> {
    text!("{}", icon).font(ICON_FONT)
    
}