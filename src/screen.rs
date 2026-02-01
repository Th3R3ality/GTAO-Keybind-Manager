pub mod keybinding;
pub mod about;



#[derive(Debug, Clone)]
pub enum Screen {
    Keybinding(keybinding::Screen),
    About(about::Screen),
}