pub const CATEGORY: &'static(&'static str, &'static str) = &("IOMS_MOUSE_BUTTON", "Mouse button input");
pub const LEFT: &'static(&'static str, &'static str) = &("MOUSE_LEFT", "Mouse Button 0 (Left Click)");
pub const RIGHT: &'static(&'static str, &'static str) = &("MOUSE_RIGHT", "Mouse Button 1 (Right Click)");
pub const MIDDLE: &'static(&'static str, &'static str) = &("MOUSE_MIDDLE", "Mouse Button 2 (Middle Click)");
pub const EXTRABTN1: &'static(&'static str, &'static str) = &("MOUSE_EXTRABTN1", "Mouse Button 3");
pub const EXTRABTN2: &'static(&'static str, &'static str) = &("MOUSE_EXTRABTN2", "Mouse Button 4");
pub const EXTRABTN3: &'static(&'static str, &'static str) = &("MOUSE_EXTRABTN3", "Mouse Button 5");
pub const EXTRABTN4: &'static(&'static str, &'static str) = &("MOUSE_EXTRABTN4", "Mouse Button 6");
pub const EXTRABTN5: &'static(&'static str, &'static str) = &("MOUSE_EXTRABTN5", "Mouse Button 7");
pub const WHEEL_UP: &'static(&'static str, &'static str) = &("IOM_WHEEL_UP", "Mouse Wheel Up");
pub const WHEEL_DOWN: &'static(&'static str, &'static str) = &("IOM_WHEEL_DOWN", "Mouse Wheel Down");
pub const ALL: &'static[&'static(&'static str,&'static str)] = &[
	CATEGORY, //Mouse button input
	LEFT, //Mouse Button 0 (Left Click)
	RIGHT, //Mouse Button 1 (Right Click)
	MIDDLE, //Mouse Button 2 (Middle Click)
	EXTRABTN1, //Mouse Button 3
	EXTRABTN2, //Mouse Button 4
	EXTRABTN3, //Mouse Button 5
	EXTRABTN4, //Mouse Button 6
	EXTRABTN5, //Mouse Button 7
	WHEEL_UP, //Mouse Wheel Up
	WHEEL_DOWN, //Mouse Wheel Down
];
