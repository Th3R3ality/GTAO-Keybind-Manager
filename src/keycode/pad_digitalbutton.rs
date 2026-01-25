pub const CATEGORY: &'static(&'static str, &'static str) = &("IOMS_PAD_DIGITALBUTTON", "Controller button input");
pub const INDEX: &'static(&'static str, &'static str) = &("L1_INDEX", "Left Shoulder Button");
pub const INDEX: &'static(&'static str, &'static str) = &("R1_INDEX", "Right Shoulder Button");
pub const INDEX: &'static(&'static str, &'static str) = &("L2_INDEX", "Left Trigger (considered \"pressed\" half way through the travel of the trigger)");
pub const INDEX: &'static(&'static str, &'static str) = &("R2_INDEX", "Right Trigger (considered \"pressed\" half way through the travel of the trigger)");
pub const INDEX: &'static(&'static str, &'static str) = &("L3_INDEX", "Left Stick Press");
pub const INDEX: &'static(&'static str, &'static str) = &("R3_INDEX", "Right Stick Press");
pub const INDEX: &'static(&'static str, &'static str) = &("LUP_INDEX", "D-pad Up");
pub const INDEX: &'static(&'static str, &'static str) = &("LRIGHT_INDEX", "D-pad Right");
pub const INDEX: &'static(&'static str, &'static str) = &("LDOWN_INDEX", "D-pad Down");
pub const INDEX: &'static(&'static str, &'static str) = &("LLEFT_INDEX", "D-pad Left");
pub const INDEX: &'static(&'static str, &'static str) = &("RUP_INDEX", "Y / Triangle");
pub const INDEX: &'static(&'static str, &'static str) = &("RRIGHT_INDEX", "B / Circle");
pub const INDEX: &'static(&'static str, &'static str) = &("RDOWN_INDEX", "A / Cross");
pub const INDEX: &'static(&'static str, &'static str) = &("RLEFT_INDEX", "X / Square");
pub const INDEX: &'static(&'static str, &'static str) = &("SELECT_INDEX", "Change View / Share");
pub const INDEX: &'static(&'static str, &'static str) = &("START_INDEX", "Menu / Options");
pub const INDEX: &'static(&'static str, &'static str) = &("TOUCH_INDEX", "Touchpad (?)");
pub const ALL: &'static[&'static(&'static str,&'static str)] = &[
	CATEGORY, //Controller button input
	INDEX, //Left Shoulder Button
	INDEX, //Right Shoulder Button
	INDEX, //Left Trigger (considered \"pressed\" half way through the travel of the trigger)
	INDEX, //Right Trigger (considered \"pressed\" half way through the travel of the trigger)
	INDEX, //Left Stick Press
	INDEX, //Right Stick Press
	INDEX, //D-pad Up
	INDEX, //D-pad Right
	INDEX, //D-pad Down
	INDEX, //D-pad Left
	INDEX, //Y / Triangle
	INDEX, //B / Circle
	INDEX, //A / Cross
	INDEX, //X / Square
	INDEX, //Change View / Share
	INDEX, //Menu / Options
	INDEX, //Touchpad (?)
];
