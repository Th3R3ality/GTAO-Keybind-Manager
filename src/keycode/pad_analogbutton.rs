pub const IOMS_PAD_ANALOGBUTTON: &'static(&'static str, &'static str, &'static str) = &("IOMS_PAD_ANALOGBUTTON", "PAD_ANALOGBUTTON", "Controller trigger input");
pub const L1_INDEX: &'static(&'static str, &'static str, &'static str) = &("L1_INDEX", "INDEX", "Left Shoulder Button");
pub const R1_INDEX: &'static(&'static str, &'static str, &'static str) = &("R1_INDEX", "INDEX", "Right Shoulder Button");
pub const L2_INDEX: &'static(&'static str, &'static str, &'static str) = &("L2_INDEX", "INDEX", "Left Trigger (considered \"pressed\" half way through the travel of the trigger)");
pub const R2_INDEX: &'static(&'static str, &'static str, &'static str) = &("R2_INDEX", "INDEX", "Right Trigger (considered \"pressed\" half way through the travel of the trigger)");
pub const L3_INDEX: &'static(&'static str, &'static str, &'static str) = &("L3_INDEX", "INDEX", "Left Stick Press");
pub const R3_INDEX: &'static(&'static str, &'static str, &'static str) = &("R3_INDEX", "INDEX", "Right Stick Press");
pub const LUP_INDEX: &'static(&'static str, &'static str, &'static str) = &("LUP_INDEX", "INDEX", "D-pad Up");
pub const LRIGHT_INDEX: &'static(&'static str, &'static str, &'static str) = &("LRIGHT_INDEX", "INDEX", "D-pad Right");
pub const LDOWN_INDEX: &'static(&'static str, &'static str, &'static str) = &("LDOWN_INDEX", "INDEX", "D-pad Down");
pub const LLEFT_INDEX: &'static(&'static str, &'static str, &'static str) = &("LLEFT_INDEX", "INDEX", "D-pad Left");
pub const RUP_INDEX: &'static(&'static str, &'static str, &'static str) = &("RUP_INDEX", "INDEX", "Y / Triangle");
pub const RRIGHT_INDEX: &'static(&'static str, &'static str, &'static str) = &("RRIGHT_INDEX", "INDEX", "B / Circle");
pub const RDOWN_INDEX: &'static(&'static str, &'static str, &'static str) = &("RDOWN_INDEX", "INDEX", "A / Cross");
pub const RLEFT_INDEX: &'static(&'static str, &'static str, &'static str) = &("RLEFT_INDEX", "INDEX", "X / Square");
pub const SELECT_INDEX: &'static(&'static str, &'static str, &'static str) = &("SELECT_INDEX", "INDEX", "Change View / Share");
pub const START_INDEX: &'static(&'static str, &'static str, &'static str) = &("START_INDEX", "INDEX", "Menu / Options");
pub const TOUCH_INDEX: &'static(&'static str, &'static str, &'static str) = &("TOUCH_INDEX", "INDEX", "Touchpad (?)");
pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	IOMS_PAD_ANALOGBUTTON, //Controller trigger input
	L1_INDEX, //Left Shoulder Button
	R1_INDEX, //Right Shoulder Button
	L2_INDEX, //Left Trigger (considered \"pressed\" half way through the travel of the trigger)
	R2_INDEX, //Right Trigger (considered \"pressed\" half way through the travel of the trigger)
	L3_INDEX, //Left Stick Press
	R3_INDEX, //Right Stick Press
	LUP_INDEX, //D-pad Up
	LRIGHT_INDEX, //D-pad Right
	LDOWN_INDEX, //D-pad Down
	LLEFT_INDEX, //D-pad Left
	RUP_INDEX, //Y / Triangle
	RRIGHT_INDEX, //B / Circle
	RDOWN_INDEX, //A / Cross
	RLEFT_INDEX, //X / Square
	SELECT_INDEX, //Change View / Share
	START_INDEX, //Menu / Options
	TOUCH_INDEX, //Touchpad (?)
];
