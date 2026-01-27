pub mod keybind_manager;
pub mod gui;
pub mod keycode;
pub mod input;

use rfd::FileDialog;

pub fn main() {
    let profiles_folder = FileDialog::new()
        .set_title("Rockstar Profiles Folder")
        .pick_folder()
        .unwrap();

    let state = keybind_manager::State::new(&profiles_folder);

    let _res = gui::run(state);
}