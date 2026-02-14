pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("CELLPHONE_MISC", "Cellphone Misc", "Cellphone Misc");
pub const CELLPHONE_SELECT: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_SELECT", "Cellphone Select", "Unknown");
pub const CELLPHONE_CANCEL: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_CANCEL", "Cellphone Cancel", "Unknown");
pub const CELLPHONE_OPTION: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_OPTION", "Cellphone Option", "Unknown");
pub const CELLPHONE_EXTRA_OPTION: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_EXTRA_OPTION", "Cellphone Extra Option", "Unknown");
pub const CELLPHONE_SCROLL_FORWARD: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_SCROLL_FORWARD", "Cellphone Scroll Forward", "Unknown");
pub const CELLPHONE_SCROLL_BACKWARD: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_SCROLL_BACKWARD", "Cellphone Scroll Backward", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // CELLPHONE_MISC
	CELLPHONE_SELECT,// INPUT_CELLPHONE_SELECT
	CELLPHONE_CANCEL,// INPUT_CELLPHONE_CANCEL
	CELLPHONE_OPTION,// INPUT_CELLPHONE_OPTION
	CELLPHONE_EXTRA_OPTION,// INPUT_CELLPHONE_EXTRA_OPTION
	CELLPHONE_SCROLL_FORWARD,// INPUT_CELLPHONE_SCROLL_FORWARD
	CELLPHONE_SCROLL_BACKWARD,// INPUT_CELLPHONE_SCROLL_BACKWARD
];
