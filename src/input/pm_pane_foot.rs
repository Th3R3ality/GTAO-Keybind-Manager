pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("PM_PANE_FOOT", "Pm Pane Foot", "Pm Pane Foot");
pub const JUMP: &'static(&'static str, &'static str, &'static str) = &("INPUT_JUMP", "Jump", "Unknown");
pub const ENTER: &'static(&'static str, &'static str, &'static str) = &("INPUT_ENTER", "Enter", "Unknown");
pub const DUCK: &'static(&'static str, &'static str, &'static str) = &("INPUT_DUCK", "Duck", "Unknown");
pub const LOOK_BEHIND: &'static(&'static str, &'static str, &'static str) = &("INPUT_LOOK_BEHIND", "Look Behind", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // PM_PANE_FOOT
	JUMP,// INPUT_JUMP
	ENTER,// INPUT_ENTER
	DUCK,// INPUT_DUCK
	LOOK_BEHIND,// INPUT_LOOK_BEHIND
];
