pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("ON_FOOT_COMBAT", "On Foot Combat", "On Foot Combat");
pub const AIM: &'static(&'static str, &'static str, &'static str) = &("INPUT_AIM", "Aim", "Unknown");
pub const COVER: &'static(&'static str, &'static str, &'static str) = &("INPUT_COVER", "Cover", "Unknown");
pub const DETONATE: &'static(&'static str, &'static str, &'static str) = &("INPUT_DETONATE", "Detonate", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // ON_FOOT_COMBAT
	AIM,// INPUT_AIM
	COVER,// INPUT_COVER
	DETONATE,// INPUT_DETONATE
];
