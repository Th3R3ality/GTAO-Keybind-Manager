pub const IOMS_GAME_CONTROLLED: &'static(&'static str, &'static str, &'static str) = &("IOMS_GAME_CONTROLLED", "Game Controlled", "Game controlled input");
pub const UNKNOWN1: &'static(&'static str, &'static str, &'static str) = &("UNKNOWN1", "Unknown1", "Valid parameters for this mapper are unknown.");
pub const UNKNOWN2: &'static(&'static str, &'static str, &'static str) = &("UNKNOWN2", "Unknown2", "This is presumably an input source for input being forced/controlled by the game. More testing is needed to confirm this.");
pub const NULL: &'static(&'static str, &'static str, &'static str) = &("KEY_NULL", "NULL", "Unbound"); // hardcoded in keycodes_to_rust.py
pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	IOMS_GAME_CONTROLLED, // Game controlled input
	UNKNOWN1, // Valid parameters for this mapper are unknown.
	UNKNOWN2, // This is presumably an input source for input being forced/controlled by the game. More testing is needed to confirm this.
	NULL, // hardcoded in keycodes_to_rust.py
];
