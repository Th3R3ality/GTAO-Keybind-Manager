pub const IOMS_TOUCHPAD_CENTERED_AXIS: &'static(&'static str, &'static str, &'static str) = &("IOMS_TOUCHPAD_CENTERED_AXIS", "TOUCHPAD_CENTERED_AXIS", "PS4/5 controller touchpad axis input (centered)");
pub const UNKNOWN1: &'static(&'static str, &'static str, &'static str) = &("UNKNOWN1", "UNKNOWN1", "Valid parameters for this mapper are unknown.");
pub const UNKNOWN2: &'static(&'static str, &'static str, &'static str) = &("UNKNOWN2", "UNKNOWN2", "This is presumably an input source for the PS4/5 controller touchpad.");
pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	IOMS_TOUCHPAD_CENTERED_AXIS, //PS4/5 controller touchpad axis input (centered)
	UNKNOWN1, //Valid parameters for this mapper are unknown.
	UNKNOWN2, //This is presumably an input source for the PS4/5 controller touchpad.
];
