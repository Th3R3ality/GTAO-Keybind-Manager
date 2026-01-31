pub const IOMS_MOUSE_BUTTON: &'static(&'static str, &'static str, &'static str) = &("IOMS_MOUSE_BUTTON", "Mouse Button", "Mouse button input");
pub const MOUSE_LEFT: &'static(&'static str, &'static str, &'static str) = &("MOUSE_LEFT", "Left", "Mouse Button 0 (Left Click)");
pub const MOUSE_RIGHT: &'static(&'static str, &'static str, &'static str) = &("MOUSE_RIGHT", "Right", "Mouse Button 1 (Right Click)");
pub const MOUSE_MIDDLE: &'static(&'static str, &'static str, &'static str) = &("MOUSE_MIDDLE", "Middle", "Mouse Button 2 (Middle Click)");
pub const MOUSE_EXTRABTN1: &'static(&'static str, &'static str, &'static str) = &("MOUSE_EXTRABTN1", "Extrabtn1", "Mouse Button 3");
pub const MOUSE_EXTRABTN2: &'static(&'static str, &'static str, &'static str) = &("MOUSE_EXTRABTN2", "Extrabtn2", "Mouse Button 4");
pub const MOUSE_EXTRABTN3: &'static(&'static str, &'static str, &'static str) = &("MOUSE_EXTRABTN3", "Extrabtn3", "Mouse Button 5");
pub const MOUSE_EXTRABTN4: &'static(&'static str, &'static str, &'static str) = &("MOUSE_EXTRABTN4", "Extrabtn4", "Mouse Button 6");
pub const MOUSE_EXTRABTN5: &'static(&'static str, &'static str, &'static str) = &("MOUSE_EXTRABTN5", "Extrabtn5", "Mouse Button 7");
pub const IOM_WHEEL_UP: &'static(&'static str, &'static str, &'static str) = &("IOM_WHEEL_UP", "Wheel Up", "Mouse Wheel Up");
pub const IOM_WHEEL_DOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_WHEEL_DOWN", "Wheel Down", "Mouse Wheel Down");
pub const NULL: &'static(&'static str, &'static str, &'static str) = &("KEY_NULL", "NULL", "Unbound"); // hardcoded in keycodes_to_rust.py
pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	IOMS_MOUSE_BUTTON, // Mouse button input
	MOUSE_LEFT, // Mouse Button 0 (Left Click)
	MOUSE_RIGHT, // Mouse Button 1 (Right Click)
	MOUSE_MIDDLE, // Mouse Button 2 (Middle Click)
	MOUSE_EXTRABTN1, // Mouse Button 3
	MOUSE_EXTRABTN2, // Mouse Button 4
	MOUSE_EXTRABTN3, // Mouse Button 5
	MOUSE_EXTRABTN4, // Mouse Button 6
	MOUSE_EXTRABTN5, // Mouse Button 7
	IOM_WHEEL_UP, // Mouse Wheel Up
	IOM_WHEEL_DOWN, // Mouse Wheel Down
	NULL, // hardcoded in keycodes_to_rust.py
];
