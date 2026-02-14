pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("RESERVED", "Reserved", "Reserved");
pub const FRONTEND_SOCIAL_CLUB: &'static(&'static str, &'static str, &'static str) = &("INPUT_FRONTEND_SOCIAL_CLUB", "Frontend Social Club", "Unknown");
pub const FRONTEND_PAUSE_ALTERNATE: &'static(&'static str, &'static str, &'static str) = &("INPUT_FRONTEND_PAUSE_ALTERNATE", "Frontend Pause Alternate", "Unknown");
pub const ENTER_CHEAT_CODE: &'static(&'static str, &'static str, &'static str) = &("INPUT_ENTER_CHEAT_CODE", "Enter Cheat Code", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // RESERVED
	FRONTEND_SOCIAL_CLUB,// INPUT_FRONTEND_SOCIAL_CLUB
	FRONTEND_PAUSE_ALTERNATE,// INPUT_FRONTEND_PAUSE_ALTERNATE
	ENTER_CHEAT_CODE,// INPUT_ENTER_CHEAT_CODE
];
