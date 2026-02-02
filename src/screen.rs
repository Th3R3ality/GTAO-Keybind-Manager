pub mod keybindings;
pub mod about;

pub use keybindings::Keybindings;
pub use about::About;


#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Landing,
    Keybindings(Keybindings),
    About(About),
}