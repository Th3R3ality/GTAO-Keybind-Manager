pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("VEHICLE_GROUND", "Vehicle Ground", "Vehicle Ground");
pub const VEH_ACCELERATE: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_ACCELERATE", "Veh Accelerate", "Unknown");
pub const VEH_BRAKE: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_BRAKE", "Veh Brake", "Unknown");
pub const VEH_MOVE_LEFT_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_MOVE_LEFT_ONLY", "Veh Move Left Only", "Unknown");
pub const VEH_MOVE_RIGHT_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_MOVE_RIGHT_ONLY", "Veh Move Right Only", "Unknown");
pub const VEH_MOVE_UP_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_MOVE_UP_ONLY", "Veh Move Up Only", "Unknown");
pub const VEH_MOVE_DOWN_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_MOVE_DOWN_ONLY", "Veh Move Down Only", "Unknown");
pub const VEH_ATTACK: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_ATTACK", "Veh Attack", "Unknown");
pub const VEH_AIM: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_AIM", "Veh Aim", "Unknown");
pub const VEH_HANDBRAKE: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_HANDBRAKE", "Veh Handbrake", "Unknown");
pub const VEH_HORN: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_HORN", "Veh Horn", "Unknown");
pub const VEH_PUSHBIKE_SPRINT: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_PUSHBIKE_SPRINT", "Veh Pushbike Sprint", "Unknown");
pub const VEH_PUSHBIKE_FRONT_BRAKE: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_PUSHBIKE_FRONT_BRAKE", "Veh Pushbike Front Brake", "Unknown");
pub const VEH_MOUSE_CONTROL_OVERRIDE: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_MOUSE_CONTROL_OVERRIDE", "Veh Mouse Control Override", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // VEHICLE_GROUND
	VEH_ACCELERATE,// INPUT_VEH_ACCELERATE
	VEH_BRAKE,// INPUT_VEH_BRAKE
	VEH_MOVE_LEFT_ONLY,// INPUT_VEH_MOVE_LEFT_ONLY
	VEH_MOVE_RIGHT_ONLY,// INPUT_VEH_MOVE_RIGHT_ONLY
	VEH_MOVE_UP_ONLY,// INPUT_VEH_MOVE_UP_ONLY
	VEH_MOVE_DOWN_ONLY,// INPUT_VEH_MOVE_DOWN_ONLY
	VEH_ATTACK,// INPUT_VEH_ATTACK
	VEH_AIM,// INPUT_VEH_AIM
	VEH_HANDBRAKE,// INPUT_VEH_HANDBRAKE
	VEH_HORN,// INPUT_VEH_HORN
	VEH_PUSHBIKE_SPRINT,// INPUT_VEH_PUSHBIKE_SPRINT
	VEH_PUSHBIKE_FRONT_BRAKE,// INPUT_VEH_PUSHBIKE_FRONT_BRAKE
	VEH_MOUSE_CONTROL_OVERRIDE,// INPUT_VEH_MOUSE_CONTROL_OVERRIDE
];
