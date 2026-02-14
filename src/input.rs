use crate::keybind::*;

pub mod cellphone_camera;
pub mod cellphone_misc;
pub mod cellphone_move;
pub mod cellphone_takeout;
pub mod context;
pub mod creator_on_foot;
pub mod creator;
pub mod endscreen;
pub mod frontend;
pub mod general;
pub mod ignore;
pub mod multiplayer_chat;
pub mod multiplayer_misc;
pub mod multiplayer_wheel_consumables;
pub mod on_foot_combat;
pub mod on_foot_melee;
pub mod on_foot_move;
pub mod on_foot_shooting;
pub mod on_foot_sniper_zoom;
pub mod on_foot_sprint;
pub mod on_foot_weapon_select;
pub mod open_weapon_wheel;
pub mod parachute;
pub mod pause;
pub mod pm_pane_foot;
pub mod reserved;
pub mod switch_camera;
pub mod vehicle_fly_online;
pub mod vehicle_fly;
pub mod vehicle_general;
pub mod vehicle_ground_online;
pub mod vehicle_ground;
pub mod vehicle_sub;

pub const INPUT_ERR_CODE: &'static str = &"???_INPUT_???";
pub const INPUT_CODES: &'static[&'static[&'static (&'static str, &'static str, &'static str)]] = &[
    cellphone_camera::ALL,
    cellphone_misc::ALL,
    cellphone_move::ALL,
    cellphone_takeout::ALL,
    context::ALL,
    creator_on_foot::ALL,
    creator::ALL,
    endscreen::ALL,
    frontend::ALL,
    general::ALL,
    ignore::ALL,
    multiplayer_chat::ALL,
    multiplayer_misc::ALL,
    multiplayer_wheel_consumables::ALL,
    on_foot_combat::ALL,
    on_foot_melee::ALL,
    on_foot_move::ALL,
    on_foot_shooting::ALL,
    on_foot_sniper_zoom::ALL,
    on_foot_sprint::ALL,
    on_foot_weapon_select::ALL,
    open_weapon_wheel::ALL,
    parachute::ALL,
    pause::ALL,
    pm_pane_foot::ALL,
    reserved::ALL,
    switch_camera::ALL,
    vehicle_fly_online::ALL,
    vehicle_fly::ALL,
    vehicle_general::ALL,
    vehicle_ground_online::ALL,
    vehicle_ground::ALL,
    vehicle_sub::ALL,
];

pub fn get_indices(input_code: &str) -> Option<KeybindInput> {
    INPUT_CODES
    .iter()
    .enumerate()
    .find_map(|(category_index, category)| {
        category
        .iter()
        .enumerate()
        .find_map(|(code_index, code)|{
            if code.0 == input_code {
                let first = category.first()?;
                Some(KeybindInput {
                    category: KeybindThing {
                        index: category_index,
                        name: first.0,
                        pretty_name: first.1,
                        desc: first.2,
                    },
                    code: KeybindThing {
                        index: code_index,
                        name: code.0,
                        pretty_name: code.1,
                        desc: code.2,
                    }
                })
            }
            else {
                None
            }
        })
    })
}

pub fn get_all_categories() -> Vec<KeybindInputCategory> {
    INPUT_CODES
    .iter()
    .enumerate()
    .map(|(source_index, source)| {
        let first = source.first().unwrap();
        KeybindInputCategory {
            index: source_index,
            name: first.0,
            pretty_name: first.1,
            desc: first.2,
        }
    })
    .collect()
}

pub fn get_all_input_codes_for_category(category: &KeybindInputCategory) -> Vec<KeybindInputCode> {
    INPUT_CODES
    .get(category.index)
    .unwrap()
    .iter()
    .enumerate()
    .filter_map(|(index, input_code)|{
        if index == 0 { return None }
        Some(KeybindInputCode {
            index,
            name: input_code.0,
            pretty_name: input_code.1,
            desc: input_code.2,
        })
    })
    .collect()
}