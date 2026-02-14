pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("MULTIPLAYER_WHEEL_CONSUMABLES", "Multiplayer Wheel Consumables", "Multiplayer Wheel Consumables");
pub const EAT_SNACK: &'static(&'static str, &'static str, &'static str) = &("INPUT_EAT_SNACK", "Eat Snack", "Unknown");
pub const USE_ARMOR: &'static(&'static str, &'static str, &'static str) = &("INPUT_USE_ARMOR", "Use Armor", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // MULTIPLAYER_WHEEL_CONSUMABLES
	EAT_SNACK,// INPUT_EAT_SNACK
	USE_ARMOR,// INPUT_USE_ARMOR
];
