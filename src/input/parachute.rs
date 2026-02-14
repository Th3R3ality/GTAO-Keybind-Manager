pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("PARACHUTE", "Parachute", "Parachute");
pub const PARACHUTE_PITCH_UP_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_PARACHUTE_PITCH_UP_ONLY", "Parachute Pitch Up Only", "Unknown");
pub const PARACHUTE_PITCH_DOWN_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_PARACHUTE_PITCH_DOWN_ONLY", "Parachute Pitch Down Only", "Unknown");
pub const PARACHUTE_TURN_LEFT_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_PARACHUTE_TURN_LEFT_ONLY", "Parachute Turn Left Only", "Unknown");
pub const PARACHUTE_TURN_RIGHT_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_PARACHUTE_TURN_RIGHT_ONLY", "Parachute Turn Right Only", "Unknown");
pub const PARACHUTE_DEPLOY: &'static(&'static str, &'static str, &'static str) = &("INPUT_PARACHUTE_DEPLOY", "Parachute Deploy", "Unknown");
pub const PARACHUTE_DETACH: &'static(&'static str, &'static str, &'static str) = &("INPUT_PARACHUTE_DETACH", "Parachute Detach", "Unknown");
pub const PARACHUTE_BRAKE_LEFT: &'static(&'static str, &'static str, &'static str) = &("INPUT_PARACHUTE_BRAKE_LEFT", "Parachute Brake Left", "Unknown");
pub const PARACHUTE_BRAKE_RIGHT: &'static(&'static str, &'static str, &'static str) = &("INPUT_PARACHUTE_BRAKE_RIGHT", "Parachute Brake Right", "Unknown");
pub const PARACHUTE_PRECISION_LANDING: &'static(&'static str, &'static str, &'static str) = &("INPUT_PARACHUTE_PRECISION_LANDING", "Parachute Precision Landing", "Unknown");
pub const PARACHUTE_SMOKE: &'static(&'static str, &'static str, &'static str) = &("INPUT_PARACHUTE_SMOKE", "Parachute Smoke", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // PARACHUTE
	PARACHUTE_PITCH_UP_ONLY,// INPUT_PARACHUTE_PITCH_UP_ONLY
	PARACHUTE_PITCH_DOWN_ONLY,// INPUT_PARACHUTE_PITCH_DOWN_ONLY
	PARACHUTE_TURN_LEFT_ONLY,// INPUT_PARACHUTE_TURN_LEFT_ONLY
	PARACHUTE_TURN_RIGHT_ONLY,// INPUT_PARACHUTE_TURN_RIGHT_ONLY
	PARACHUTE_DEPLOY,// INPUT_PARACHUTE_DEPLOY
	PARACHUTE_DETACH,// INPUT_PARACHUTE_DETACH
	PARACHUTE_BRAKE_LEFT,// INPUT_PARACHUTE_BRAKE_LEFT
	PARACHUTE_BRAKE_RIGHT,// INPUT_PARACHUTE_BRAKE_RIGHT
	PARACHUTE_PRECISION_LANDING,// INPUT_PARACHUTE_PRECISION_LANDING
	PARACHUTE_SMOKE,// INPUT_PARACHUTE_SMOKE
];
