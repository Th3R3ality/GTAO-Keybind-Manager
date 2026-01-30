pub const IOMS_PAD_AXIS: &'static(&'static str, &'static str, &'static str) = &("IOMS_PAD_AXIS", "PAD_AXIS", "Controller axis input");
pub const IOM_AXIS_LX: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LX", "AXIS_LX", "unknown");
pub const IOM_AXIS_LY: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LY", "AXIS_LY", "unknown");
pub const IOM_AXIS_RX: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RX", "AXIS_RX", "unknown");
pub const IOM_AXIS_RY: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RY", "AXIS_RY", "unknown");
pub const IOM_AXIS_LUP: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LUP", "AXIS_LUP", "unknown");
pub const IOM_AXIS_LDOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LDOWN", "AXIS_LDOWN", "unknown");
pub const IOM_AXIS_LLEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LLEFT", "AXIS_LLEFT", "unknown");
pub const IOM_AXIS_LRIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LRIGHT", "AXIS_LRIGHT", "unknown");
pub const IOM_AXIS_LUR: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LUR", "AXIS_LUR", "unknown");
pub const IOM_AXIS_LUL: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LUL", "AXIS_LUL", "unknown");
pub const IOM_AXIS_LDR: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LDR", "AXIS_LDR", "unknown");
pub const IOM_AXIS_LDL: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LDL", "AXIS_LDL", "unknown");
pub const IOM_AXIS_RUP: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RUP", "AXIS_RUP", "unknown");
pub const IOM_AXIS_RDOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RDOWN", "AXIS_RDOWN", "unknown");
pub const IOM_AXIS_RLEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RLEFT", "AXIS_RLEFT", "unknown");
pub const IOM_AXIS_RRIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RRIGHT", "AXIS_RRIGHT", "unknown");
pub const IOM_AXIS_RUR: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RUR", "AXIS_RUR", "unknown");
pub const IOM_AXIS_RUL: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RUL", "AXIS_RUL", "unknown");
pub const IOM_AXIS_RDR: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RDR", "AXIS_RDR", "unknown");
pub const IOM_AXIS_RDL: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RDL", "AXIS_RDL", "unknown");
pub const IOM_AXIS_DPADX: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_DPADX", "AXIS_DPADX", "unknown");
pub const IOM_AXIS_DPADY: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_DPADY", "AXIS_DPADY", "unknown");
pub const IOM_AXIS_LY_UP: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LY_UP", "AXIS_LY_UP", "unknown");
pub const IOM_AXIS_LY_DOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LY_DOWN", "AXIS_LY_DOWN", "unknown");
pub const IOM_AXIS_LX_LEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LX_LEFT", "AXIS_LX_LEFT", "unknown");
pub const IOM_AXIS_LX_RIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LX_RIGHT", "AXIS_LX_RIGHT", "unknown");
pub const IOM_AXIS_RY_UP: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RY_UP", "AXIS_RY_UP", "unknown");
pub const IOM_AXIS_RY_DOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RY_DOWN", "AXIS_RY_DOWN", "unknown");
pub const IOM_AXIS_RX_LEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RX_LEFT", "AXIS_RX_LEFT", "unknown");
pub const IOM_AXIS_RX_RIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RX_RIGHT", "AXIS_RX_RIGHT", "unknown");
pub const NULL: &'static(&'static str, &'static str, &'static str) = &("KEY_NULL", "NULL", "Unbound"); // hardcoded in keycodes_to_rust.py
pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	IOMS_PAD_AXIS, // Controller axis input
	IOM_AXIS_LX, // unknown
	IOM_AXIS_LY, // unknown
	IOM_AXIS_RX, // unknown
	IOM_AXIS_RY, // unknown
	IOM_AXIS_LUP, // unknown
	IOM_AXIS_LDOWN, // unknown
	IOM_AXIS_LLEFT, // unknown
	IOM_AXIS_LRIGHT, // unknown
	IOM_AXIS_LUR, // unknown
	IOM_AXIS_LUL, // unknown
	IOM_AXIS_LDR, // unknown
	IOM_AXIS_LDL, // unknown
	IOM_AXIS_RUP, // unknown
	IOM_AXIS_RDOWN, // unknown
	IOM_AXIS_RLEFT, // unknown
	IOM_AXIS_RRIGHT, // unknown
	IOM_AXIS_RUR, // unknown
	IOM_AXIS_RUL, // unknown
	IOM_AXIS_RDR, // unknown
	IOM_AXIS_RDL, // unknown
	IOM_AXIS_DPADX, // unknown
	IOM_AXIS_DPADY, // unknown
	IOM_AXIS_LY_UP, // unknown
	IOM_AXIS_LY_DOWN, // unknown
	IOM_AXIS_LX_LEFT, // unknown
	IOM_AXIS_LX_RIGHT, // unknown
	IOM_AXIS_RY_UP, // unknown
	IOM_AXIS_RY_DOWN, // unknown
	IOM_AXIS_RX_LEFT, // unknown
	IOM_AXIS_RX_RIGHT, // unknown
	NULL, // hardcoded in keycodes_to_rust.py
];
