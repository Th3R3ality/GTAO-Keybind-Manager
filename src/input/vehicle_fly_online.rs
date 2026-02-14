pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("VEHICLE_FLY_ONLINE", "Vehicle Fly Online", "Vehicle Fly Online");
pub const VEH_FLY_BOOST: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_FLY_BOOST", "Veh Fly Boost", "Unknown");
pub const VEH_FLY_BOMB_BAY: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_FLY_BOMB_BAY", "Veh Fly Bomb Bay", "Unknown");
pub const VEH_FLY_COUNTER: &'static(&'static str, &'static str, &'static str) = &("INPUT_VEH_FLY_COUNTER", "Veh Fly Counter", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // VEHICLE_FLY_ONLINE
	VEH_FLY_BOOST,// INPUT_VEH_FLY_BOOST
	VEH_FLY_BOMB_BAY,// INPUT_VEH_FLY_BOMB_BAY
	VEH_FLY_COUNTER,// INPUT_VEH_FLY_COUNTER
];
