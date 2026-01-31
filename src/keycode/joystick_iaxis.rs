pub const IOMS_JOYSTICK_IAXIS: &'static(&'static str, &'static str, &'static str) = &("IOMS_JOYSTICK_IAXIS", "Joystick Iaxis", "Joystick / flight stick axis input (inverted)");
pub const IOM_JOYSTICK_AXIS1: &'static(&'static str, &'static str, &'static str) = &("IOM_JOYSTICK_AXIS1", "Joystick Axis1", "hardware dependent");
pub const IOM_JOYSTICK_AXIS2: &'static(&'static str, &'static str, &'static str) = &("IOM_JOYSTICK_AXIS2", "Joystick Axis2", "hardware dependent");
pub const IOM_JOYSTICK_AXIS3: &'static(&'static str, &'static str, &'static str) = &("IOM_JOYSTICK_AXIS3", "Joystick Axis3", "hardware dependent");
pub const IOM_JOYSTICK_AXIS4: &'static(&'static str, &'static str, &'static str) = &("IOM_JOYSTICK_AXIS4", "Joystick Axis4", "hardware dependent");
pub const IOM_JOYSTICK_AXIS5: &'static(&'static str, &'static str, &'static str) = &("IOM_JOYSTICK_AXIS5", "Joystick Axis5", "hardware dependent");
pub const IOM_JOYSTICK_AXIS6: &'static(&'static str, &'static str, &'static str) = &("IOM_JOYSTICK_AXIS6", "Joystick Axis6", "hardware dependent");
pub const IOM_JOYSTICK_AXIS7: &'static(&'static str, &'static str, &'static str) = &("IOM_JOYSTICK_AXIS7", "Joystick Axis7", "hardware dependent");
pub const IOM_JOYSTICK_AXIS8: &'static(&'static str, &'static str, &'static str) = &("IOM_JOYSTICK_AXIS8", "Joystick Axis8", "hardware dependent");
pub const NULL: &'static(&'static str, &'static str, &'static str) = &("KEY_NULL", "NULL", "Unbound"); // hardcoded in keycodes_to_rust.py
pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	IOMS_JOYSTICK_IAXIS, // Joystick / flight stick axis input (inverted)
	IOM_JOYSTICK_AXIS1, // hardware dependent
	IOM_JOYSTICK_AXIS2, // hardware dependent
	IOM_JOYSTICK_AXIS3, // hardware dependent
	IOM_JOYSTICK_AXIS4, // hardware dependent
	IOM_JOYSTICK_AXIS5, // hardware dependent
	IOM_JOYSTICK_AXIS6, // hardware dependent
	IOM_JOYSTICK_AXIS7, // hardware dependent
	IOM_JOYSTICK_AXIS8, // hardware dependent
	NULL, // hardcoded in keycodes_to_rust.py
];
