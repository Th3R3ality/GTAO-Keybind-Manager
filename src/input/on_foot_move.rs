pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("ON_FOOT_MOVE", "On Foot Move", "On Foot Move");
pub const MOVE_UP_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_MOVE_UP_ONLY", "Move Up Only", "Unknown");
pub const MOVE_DOWN_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_MOVE_DOWN_ONLY", "Move Down Only", "Unknown");
pub const MOVE_LEFT_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_MOVE_LEFT_ONLY", "Move Left Only", "Unknown");
pub const MOVE_RIGHT_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_MOVE_RIGHT_ONLY", "Move Right Only", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // ON_FOOT_MOVE
	MOVE_UP_ONLY,// INPUT_MOVE_UP_ONLY
	MOVE_DOWN_ONLY,// INPUT_MOVE_DOWN_ONLY
	MOVE_LEFT_ONLY,// INPUT_MOVE_LEFT_ONLY
	MOVE_RIGHT_ONLY,// INPUT_MOVE_RIGHT_ONLY
];
