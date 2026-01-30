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

pub const INPUT_CODES: &'static[&'static[&'static str]] = &[
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

pub fn get_index(input_code: &str) -> Option<usize> {
    return match INPUT_CODES.iter()
        .flat_map(|x| x.iter())
        .enumerate()
        .find(|x| x.1 == &input_code) {
            Some((index, _)) => Some(index),
            _ => None,
        }
}
pub fn from_index(index: usize) -> Option<&'static str> {
    return INPUT_CODES
        .iter()
        .flat_map(|x| x.iter())
        .copied()
        .nth(index);
}