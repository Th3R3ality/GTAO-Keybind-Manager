pub const CATEGORY: &'static(&'static str, &'static str) = &("IOMS_MKB_AXIS", "Mouse & keyboard input as axis input");
pub const UNKNOWN1: &'static(&'static str, &'static str) = &("UNKNOWN1", "Valid parameters for this mapper are unknown.");
pub const UNKNOWN2: &'static(&'static str, &'static str) = &("UNKNOWN2", "This is presumably an input mapper for axis input from mouse and keyboard input.");
pub const ALL: &'static[&'static(&'static str,&'static str)] = &[
	CATEGORY, //Mouse & keyboard input as axis input
	UNKNOWN1, //Valid parameters for this mapper are unknown.
	UNKNOWN2, //This is presumably an input mapper for axis input from mouse and keyboard input.
];
