pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("ON_FOOT_SHOOTING", "On Foot Shooting", "On Foot Shooting");
pub const ATTACK: &'static(&'static str, &'static str, &'static str) = &("INPUT_ATTACK", "Attack", "Unknown");
pub const RELOAD: &'static(&'static str, &'static str, &'static str) = &("INPUT_RELOAD", "Reload", "Unknown");
pub const WEAPON_SPECIAL_TWO: &'static(&'static str, &'static str, &'static str) = &("INPUT_WEAPON_SPECIAL_TWO", "Weapon Special Two", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // ON_FOOT_SHOOTING
	ATTACK,// INPUT_ATTACK
	RELOAD,// INPUT_RELOAD
	WEAPON_SPECIAL_TWO,// INPUT_WEAPON_SPECIAL_TWO
];
