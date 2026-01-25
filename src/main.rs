pub mod controls;
pub mod gui;
pub mod keycode;
pub mod input;

use rfd::FileDialog;

pub fn main() {
    let profiles_folder = FileDialog::new()
        .set_title("Rockstar Profiles Folder")
        .pick_folder();
    if (profiles_folder.is_none()){
        println!("no path... exiting");
        return;
    }
    let _res = controls::load_profiles(&profiles_folder.unwrap());

    
    let _res = gui::run();
}