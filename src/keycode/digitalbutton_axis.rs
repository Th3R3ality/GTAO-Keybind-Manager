pub const IOMS_DIGITALBUTTON_AXIS: &'static(&'static str, &'static str, &'static str) = &("IOMS_DIGITALBUTTON_AXIS", "DIGITALBUTTON_AXIS", "Controller button input as axis input");
pub const UNKNOWN1: &'static(&'static str, &'static str, &'static str) = &("UNKNOWN1", "UNKNOWN1", "Valid parameters for this mapper are unknown.");
pub const UNKNOWN2: &'static(&'static str, &'static str, &'static str) = &("UNKNOWN2", "UNKNOWN2", "This is presumably an input mapper for axis input from digital controller buttons.");
pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	IOMS_DIGITALBUTTON_AXIS, //Controller button input as axis input
	UNKNOWN1, //Valid parameters for this mapper are unknown.
	UNKNOWN2, //This is presumably an input mapper for axis input from digital controller buttons.
];
