pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("PAUSE", "Pause", "Pause");
pub const FRONTEND_PAUSE: &'static(&'static str, &'static str, &'static str) = &("INPUT_FRONTEND_PAUSE", "Frontend Pause", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // PAUSE
	FRONTEND_PAUSE,// INPUT_FRONTEND_PAUSE
];
