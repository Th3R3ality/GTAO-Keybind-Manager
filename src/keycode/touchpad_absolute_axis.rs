pub const CATEGORY: &'static(&'static str, &'static str) = &("IOMS_TOUCHPAD_ABSOLUTE_AXIS", "PS4/5 controller touchpad axis input (absolute)");
pub const UNKNOWN1: &'static(&'static str, &'static str) = &("UNKNOWN1", "Vaild parameters for this mapper are unknown.");
pub const UNKNOWN2: &'static(&'static str, &'static str) = &("UNKNOWN2", "This is presumably an input source for the PS4/5 controller touchpad.");
pub const ALL: &'static[&'static(&'static str,&'static str)] = &[
	CATEGORY, //PS4/5 controller touchpad axis input (absolute)
	UNKNOWN1, //Vaild parameters for this mapper are unknown.
	UNKNOWN2, //This is presumably an input source for the PS4/5 controller touchpad.
];
