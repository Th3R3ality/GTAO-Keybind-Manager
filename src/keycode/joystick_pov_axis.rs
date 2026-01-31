pub const IOMS_JOYSTICK_POV_AXIS: &'static(&'static str, &'static str, &'static str) = &("IOMS_JOYSTICK_POV_AXIS", "Joystick Pov Axis", "Joystick / flight stick point of view axis input");
pub const IOM_POV1_UP: &'static(&'static str, &'static str, &'static str) = &("IOM_POV1_UP", "Pov1 Up", "hardware dependent");
pub const IOM_POV1_RIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_POV1_RIGHT", "Pov1 Right", "hardware dependent");
pub const IOM_POV1_DOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_POV1_DOWN", "Pov1 Down", "hardware dependent");
pub const IOM_POV1_LEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_POV1_LEFT", "Pov1 Left", "hardware dependent");
pub const IOM_POV2_UP: &'static(&'static str, &'static str, &'static str) = &("IOM_POV2_UP", "Pov2 Up", "hardware dependent");
pub const IOM_POV2_RIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_POV2_RIGHT", "Pov2 Right", "hardware dependent");
pub const IOM_POV2_DOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_POV2_DOWN", "Pov2 Down", "hardware dependent");
pub const IOM_POV2_LEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_POV2_LEFT", "Pov2 Left", "hardware dependent");
pub const IOM_POV3_UP: &'static(&'static str, &'static str, &'static str) = &("IOM_POV3_UP", "Pov3 Up", "hardware dependent");
pub const IOM_POV3_RIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_POV3_RIGHT", "Pov3 Right", "hardware dependent");
pub const IOM_POV3_DOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_POV3_DOWN", "Pov3 Down", "hardware dependent");
pub const IOM_POV3_LEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_POV3_LEFT", "Pov3 Left", "hardware dependent");
pub const IOM_POV4_UP: &'static(&'static str, &'static str, &'static str) = &("IOM_POV4_UP", "Pov4 Up", "hardware dependent");
pub const IOM_POV4_RIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_POV4_RIGHT", "Pov4 Right", "hardware dependent");
pub const IOM_POV4_DOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_POV4_DOWN", "Pov4 Down", "hardware dependent");
pub const IOM_POV4_LEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_POV4_LEFT", "Pov4 Left", "hardware dependent");
pub const NULL: &'static(&'static str, &'static str, &'static str) = &("KEY_NULL", "NULL", "Unbound"); // hardcoded in keycodes_to_rust.py
pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	IOMS_JOYSTICK_POV_AXIS, // Joystick / flight stick point of view axis input
	IOM_POV1_UP, // hardware dependent
	IOM_POV1_RIGHT, // hardware dependent
	IOM_POV1_DOWN, // hardware dependent
	IOM_POV1_LEFT, // hardware dependent
	IOM_POV2_UP, // hardware dependent
	IOM_POV2_RIGHT, // hardware dependent
	IOM_POV2_DOWN, // hardware dependent
	IOM_POV2_LEFT, // hardware dependent
	IOM_POV3_UP, // hardware dependent
	IOM_POV3_RIGHT, // hardware dependent
	IOM_POV3_DOWN, // hardware dependent
	IOM_POV3_LEFT, // hardware dependent
	IOM_POV4_UP, // hardware dependent
	IOM_POV4_RIGHT, // hardware dependent
	IOM_POV4_DOWN, // hardware dependent
	IOM_POV4_LEFT, // hardware dependent
	NULL, // hardcoded in keycodes_to_rust.py
];
