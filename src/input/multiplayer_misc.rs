pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("MULTIPLAYER_MISC", "Multiplayer Misc", "Multiplayer Misc");
pub const SPECIAL_ABILITY_SECONDARY: &'static(&'static str, &'static str, &'static str) = &("INPUT_SPECIAL_ABILITY_SECONDARY", "Special Ability Secondary", "Unknown");
pub const DROP_WEAPON: &'static(&'static str, &'static str, &'static str) = &("INPUT_DROP_WEAPON", "Drop Weapon", "Unknown");
pub const DROP_AMMO: &'static(&'static str, &'static str, &'static str) = &("INPUT_DROP_AMMO", "Drop Ammo", "Unknown");
pub const SWITCH_VISOR: &'static(&'static str, &'static str, &'static str) = &("INPUT_SWITCH_VISOR", "Switch Visor", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // MULTIPLAYER_MISC
	SPECIAL_ABILITY_SECONDARY,// INPUT_SPECIAL_ABILITY_SECONDARY
	DROP_WEAPON,// INPUT_DROP_WEAPON
	DROP_AMMO,// INPUT_DROP_AMMO
	SWITCH_VISOR,// INPUT_SWITCH_VISOR
];
