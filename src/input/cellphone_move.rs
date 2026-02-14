pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("CELLPHONE_MOVE", "Cellphone Move", "Cellphone Move");
pub const CELLPHONE_UP: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_UP", "Cellphone Up", "Unknown");
pub const CELLPHONE_DOWN: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_DOWN", "Cellphone Down", "Unknown");
pub const CELLPHONE_LEFT: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_LEFT", "Cellphone Left", "Unknown");
pub const CELLPHONE_RIGHT: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_RIGHT", "Cellphone Right", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // CELLPHONE_MOVE
	CELLPHONE_UP,// INPUT_CELLPHONE_UP
	CELLPHONE_DOWN,// INPUT_CELLPHONE_DOWN
	CELLPHONE_LEFT,// INPUT_CELLPHONE_LEFT
	CELLPHONE_RIGHT,// INPUT_CELLPHONE_RIGHT
];
