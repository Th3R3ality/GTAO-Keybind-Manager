pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("ON_FOOT_SNIPER_ZOOM", "On Foot Sniper Zoom", "On Foot Sniper Zoom");
pub const SNIPER_ZOOM_IN_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_SNIPER_ZOOM_IN_ONLY", "Sniper Zoom In Only", "Unknown");
pub const SNIPER_ZOOM_OUT_ONLY: &'static(&'static str, &'static str, &'static str) = &("INPUT_SNIPER_ZOOM_OUT_ONLY", "Sniper Zoom Out Only", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // ON_FOOT_SNIPER_ZOOM
	SNIPER_ZOOM_IN_ONLY,// INPUT_SNIPER_ZOOM_IN_ONLY
	SNIPER_ZOOM_OUT_ONLY,// INPUT_SNIPER_ZOOM_OUT_ONLY
];
