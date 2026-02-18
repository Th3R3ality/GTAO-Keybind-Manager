pub mod keybindings;
pub mod about;

pub use keybindings::Keybindings;
pub use about::About;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Screen {
    #[default]
    Landing,
    Keybindings(Keybindings),
    About(About),
    Share(Keybindings),
}