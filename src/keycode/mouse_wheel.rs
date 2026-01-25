pub const CATEGORY: &'static(&'static str, &'static str) = &("IOMS_MOUSE_WHEEL", "Mouse wheel input");
pub const WHEEL_UP: &'static(&'static str, &'static str) = &("IOM_WHEEL_UP", "Mouse Wheel Up");
pub const WHEEL_DOWN: &'static(&'static str, &'static str) = &("IOM_WHEEL_DOWN", "Mouse Wheel Down");
pub const ALL: &'static[&'static(&'static str,&'static str)] = &[
	CATEGORY, //Mouse wheel input
	WHEEL_UP, //Mouse Wheel Up
	WHEEL_DOWN, //Mouse Wheel Down
];
