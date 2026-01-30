pub const IOMS_TOUCHPAD_ABSOLUTE_AXIS: &'static(&'static str, &'static str, &'static str) = &("IOMS_TOUCHPAD_ABSOLUTE_AXIS", "TOUCHPAD_ABSOLUTE_AXIS", "PS4/5 controller touchpad axis input (absolute)");
pub const UNKNOWN1: &'static(&'static str, &'static str, &'static str) = &("UNKNOWN1", "UNKNOWN1", "Vaild parameters for this mapper are unknown.");
pub const UNKNOWN2: &'static(&'static str, &'static str, &'static str) = &("UNKNOWN2", "UNKNOWN2", "This is presumably an input source for the PS4/5 controller touchpad.");
pub const NULL: &'static(&'static str, &'static str, &'static str) = &("KEY_NULL", "NULL", "Unbound"); // hardcoded in keycodes_to_rust.py
pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	IOMS_TOUCHPAD_ABSOLUTE_AXIS, // PS4/5 controller touchpad axis input (absolute)
	UNKNOWN1, // Vaild parameters for this mapper are unknown.
	UNKNOWN2, // This is presumably an input source for the PS4/5 controller touchpad.
	NULL, // hardcoded in keycodes_to_rust.py
];
