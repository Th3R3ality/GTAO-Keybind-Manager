pub const CATEGORY: &'static(&'static str, &'static str) = &("IOMS_PAD_DEBUGBUTTON", "Controller button input (debug interface");
pub const L1: &'static(&'static str, &'static str) = &("L1", "Left Shoulder Button");
pub const R1: &'static(&'static str, &'static str) = &("R1", "Right Shoulder Button");
pub const L2: &'static(&'static str, &'static str) = &("L2", "Left Trigger");
pub const R2: &'static(&'static str, &'static str) = &("R2", "Right Trigger");
pub const L3: &'static(&'static str, &'static str) = &("L3", "Left Stick Press");
pub const R3: &'static(&'static str, &'static str) = &("R3", "Right Stick Press");
pub const LUP: &'static(&'static str, &'static str) = &("LUP", "D-pad Up");
pub const LRIGHT: &'static(&'static str, &'static str) = &("LRIGHT", "D-pad Right");
pub const LDOWN: &'static(&'static str, &'static str) = &("LDOWN", "D-pad Down");
pub const LLEFT: &'static(&'static str, &'static str) = &("LLEFT", "D-pad Left");
pub const RUP: &'static(&'static str, &'static str) = &("RUP", "Y / Triangle");
pub const RRIGHT: &'static(&'static str, &'static str) = &("RRIGHT", "B / Circle");
pub const RDOWN: &'static(&'static str, &'static str) = &("RDOWN", "A / Cross");
pub const RLEFT: &'static(&'static str, &'static str) = &("RLEFT", "X / Square");
pub const SELECT: &'static(&'static str, &'static str) = &("SELECT", "Change View / Share");
pub const START: &'static(&'static str, &'static str) = &("START", "Menu / Options");
pub const TOUCH: &'static(&'static str, &'static str) = &("TOUCH", "Touchpad (?)");
pub const ALL: &'static[&'static(&'static str,&'static str)] = &[
	CATEGORY, //Controller button input (debug interface
	L1, //Left Shoulder Button
	R1, //Right Shoulder Button
	L2, //Left Trigger
	R2, //Right Trigger
	L3, //Left Stick Press
	R3, //Right Stick Press
	LUP, //D-pad Up
	LRIGHT, //D-pad Right
	LDOWN, //D-pad Down
	LLEFT, //D-pad Left
	RUP, //Y / Triangle
	RRIGHT, //B / Circle
	RDOWN, //A / Cross
	RLEFT, //X / Square
	SELECT, //Change View / Share
	START, //Menu / Options
	TOUCH, //Touchpad (?)
];
