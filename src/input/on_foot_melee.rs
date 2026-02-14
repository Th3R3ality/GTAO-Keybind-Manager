pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("ON_FOOT_MELEE", "On Foot Melee", "On Foot Melee");
pub const MELEE_ATTACK_LIGHT: &'static(&'static str, &'static str, &'static str) = &("INPUT_MELEE_ATTACK_LIGHT", "Melee Attack Light", "Unknown");
pub const MELEE_ATTACK_HEAVY: &'static(&'static str, &'static str, &'static str) = &("INPUT_MELEE_ATTACK_HEAVY", "Melee Attack Heavy", "Unknown");
pub const MELEE_BLOCK: &'static(&'static str, &'static str, &'static str) = &("INPUT_MELEE_BLOCK", "Melee Block", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // ON_FOOT_MELEE
	MELEE_ATTACK_LIGHT,// INPUT_MELEE_ATTACK_LIGHT
	MELEE_ATTACK_HEAVY,// INPUT_MELEE_ATTACK_HEAVY
	MELEE_BLOCK,// INPUT_MELEE_BLOCK
];
