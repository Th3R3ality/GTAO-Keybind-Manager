pub const CATEGORY: &'static(&'static str, &'static str) = &("IOMS_GAME_CONTROLLED", "Game controlled input");
pub const UNKNOWN1: &'static(&'static str, &'static str) = &("UNKNOWN1", "Valid parameters for this mapper are unknown.");
pub const UNKNOWN2: &'static(&'static str, &'static str) = &("UNKNOWN2", "This is presumably an input source for input being forced/controlled by the game. More testing is needed to confirm this.");
pub const ALL: &'static[&'static(&'static str,&'static str)] = &[
	CATEGORY, //Game controlled input
	UNKNOWN1, //Valid parameters for this mapper are unknown.
	UNKNOWN2, //This is presumably an input source for input being forced/controlled by the game. More testing is needed to confirm this.
];
