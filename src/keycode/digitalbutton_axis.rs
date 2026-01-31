pub const IOMS_DIGITALBUTTON_AXIS: &'static(&'static str, &'static str, &'static str) = &("IOMS_DIGITALBUTTON_AXIS", "Digitalbutton Axis", "Controller button input as axis input");
pub const UNKNOWN1: &'static(&'static str, &'static str, &'static str) = &("UNKNOWN1", "Unknown1", "Valid parameters for this mapper are unknown.");
pub const UNKNOWN2: &'static(&'static str, &'static str, &'static str) = &("UNKNOWN2", "Unknown2", "This is presumably an input mapper for axis input from digital controller buttons.");
pub const NULL: &'static(&'static str, &'static str, &'static str) = &("KEY_NULL", "NULL", "Unbound"); // hardcoded in keycodes_to_rust.py
pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	IOMS_DIGITALBUTTON_AXIS, // Controller button input as axis input
	UNKNOWN1, // Valid parameters for this mapper are unknown.
	UNKNOWN2, // This is presumably an input mapper for axis input from digital controller buttons.
	NULL, // hardcoded in keycodes_to_rust.py
];
