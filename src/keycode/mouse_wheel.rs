pub const IOMS_MOUSE_WHEEL: &'static(&'static str, &'static str, &'static str) = &("IOMS_MOUSE_WHEEL", "MOUSE_WHEEL", "Mouse wheel input");
pub const IOM_WHEEL_UP: &'static(&'static str, &'static str, &'static str) = &("IOM_WHEEL_UP", "WHEEL_UP", "Mouse Wheel Up");
pub const IOM_WHEEL_DOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_WHEEL_DOWN", "WHEEL_DOWN", "Mouse Wheel Down");
pub const NULL: &'static(&'static str, &'static str, &'static str) = &("KEY_NULL", "NULL", "Unbound"); // hardcoded in keycodes_to_rust.py
pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	IOMS_MOUSE_WHEEL, // Mouse wheel input
	IOM_WHEEL_UP, // Mouse Wheel Up
	IOM_WHEEL_DOWN, // Mouse Wheel Down
	NULL, // hardcoded in keycodes_to_rust.py
];
