use iced::Font;

pub const ICON32: &[u8] = include_bytes!("../assets/32x.png");
pub const ICON64: &[u8] = include_bytes!("../assets/64x.png");
pub const ICON128: &[u8] = include_bytes!("../assets/128x.png");
pub const ICON256: &[u8] = include_bytes!("../assets/256x.png");

pub const ICON_FONT_DATA: &[u8] = include_bytes!("../assets/MaterialIcons-Regular.ttf");
pub const ICON_FONT: Font = Font::with_name("Material Icons");