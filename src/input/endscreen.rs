pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("ENDSCREEN", "Endscreen", "Endscreen");
pub const FRONTEND_ENDSCREEN_ACCEPT: &'static(&'static str, &'static str, &'static str) = &("INPUT_FRONTEND_ENDSCREEN_ACCEPT", "Frontend Endscreen Accept", "Unknown");
pub const FRONTEND_ENDSCREEN_EXPAND: &'static(&'static str, &'static str, &'static str) = &("INPUT_FRONTEND_ENDSCREEN_EXPAND", "Frontend Endscreen Expand", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // ENDSCREEN
	FRONTEND_ENDSCREEN_ACCEPT,// INPUT_FRONTEND_ENDSCREEN_ACCEPT
	FRONTEND_ENDSCREEN_EXPAND,// INPUT_FRONTEND_ENDSCREEN_EXPAND
];
