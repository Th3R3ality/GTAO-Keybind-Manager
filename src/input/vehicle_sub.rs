pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("VEHICLE_SUB", "Vehicle Sub", "Vehicle Sub");
pub const VEH_SUB_THROTTLE_UP: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_SUB_THROTTLE_UP", "Veh Sub Throttle Up", "Unknown");
pub const VEH_SUB_THROTTLE_DOWN: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_SUB_THROTTLE_DOWN", "Veh Sub Throttle Down", "Unknown");
pub const VEH_SUB_TURN_HARD_LEFT: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_SUB_TURN_HARD_LEFT", "Veh Sub Turn Hard Left", "Unknown");
pub const VEH_SUB_TURN_HARD_RIGHT: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_SUB_TURN_HARD_RIGHT", "Veh Sub Turn Hard Right", "Unknown");
pub const VEH_SUB_TURN_LEFT_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_SUB_TURN_LEFT_ONLY", "Veh Sub Turn Left Only", "Unknown");
pub const VEH_SUB_TURN_RIGHT_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_SUB_TURN_RIGHT_ONLY", "Veh Sub Turn Right Only", "Unknown");
pub const VEH_SUB_PITCH_UP_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_SUB_PITCH_UP_ONLY", "Veh Sub Pitch Up Only", "Unknown");
pub const VEH_SUB_PITCH_DOWN_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_SUB_PITCH_DOWN_ONLY", "Veh Sub Pitch Down Only", "Unknown");
pub const VEH_SUB_ASCEND: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_SUB_ASCEND", "Veh Sub Ascend", "Unknown");
pub const VEH_SUB_DESCEND: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_SUB_DESCEND", "Veh Sub Descend", "Unknown");
pub const VEH_SUB_MOUSE_CONTROL_OVERRIDE: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_SUB_MOUSE_CONTROL_OVERRIDE", "Veh Sub Mouse Control Override", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // VEHICLE_SUB
	VEH_SUB_THROTTLE_UP,// INPUT_VEH_SUB_THROTTLE_UP
	VEH_SUB_THROTTLE_DOWN,// INPUT_VEH_SUB_THROTTLE_DOWN
	VEH_SUB_TURN_HARD_LEFT,// INPUT_VEH_SUB_TURN_HARD_LEFT
	VEH_SUB_TURN_HARD_RIGHT,// INPUT_VEH_SUB_TURN_HARD_RIGHT
	VEH_SUB_TURN_LEFT_ONLY,// INPUT_VEH_SUB_TURN_LEFT_ONLY
	VEH_SUB_TURN_RIGHT_ONLY,// INPUT_VEH_SUB_TURN_RIGHT_ONLY
	VEH_SUB_PITCH_UP_ONLY,// INPUT_VEH_SUB_PITCH_UP_ONLY
	VEH_SUB_PITCH_DOWN_ONLY,// INPUT_VEH_SUB_PITCH_DOWN_ONLY
	VEH_SUB_ASCEND,// INPUT_VEH_SUB_ASCEND
	VEH_SUB_DESCEND,// INPUT_VEH_SUB_DESCEND
	VEH_SUB_MOUSE_CONTROL_OVERRIDE,// INPUT_VEH_SUB_MOUSE_CONTROL_OVERRIDE
];
