pub const IOMS_PAD_AXIS: &'static(&'static str, &'static str, &'static str) = &("IOMS_PAD_AXIS", "Pad Axis", "Controller axis input");
pub const IOM_AXIS_LX: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LX", "Axis Lx", "unknown");
pub const IOM_AXIS_LY: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LY", "Axis Ly", "unknown");
pub const IOM_AXIS_RX: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RX", "Axis Rx", "unknown");
pub const IOM_AXIS_RY: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RY", "Axis Ry", "unknown");
pub const IOM_AXIS_LUP: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LUP", "Axis Lup", "unknown");
pub const IOM_AXIS_LDOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LDOWN", "Axis Ldown", "unknown");
pub const IOM_AXIS_LLEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LLEFT", "Axis Lleft", "unknown");
pub const IOM_AXIS_LRIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LRIGHT", "Axis Lright", "unknown");
pub const IOM_AXIS_LUR: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LUR", "Axis Lur", "unknown");
pub const IOM_AXIS_LUL: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LUL", "Axis Lul", "unknown");
pub const IOM_AXIS_LDR: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LDR", "Axis Ldr", "unknown");
pub const IOM_AXIS_LDL: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LDL", "Axis Ldl", "unknown");
pub const IOM_AXIS_RUP: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RUP", "Axis Rup", "unknown");
pub const IOM_AXIS_RDOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RDOWN", "Axis Rdown", "unknown");
pub const IOM_AXIS_RLEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RLEFT", "Axis Rleft", "unknown");
pub const IOM_AXIS_RRIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RRIGHT", "Axis Rright", "unknown");
pub const IOM_AXIS_RUR: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RUR", "Axis Rur", "unknown");
pub const IOM_AXIS_RUL: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RUL", "Axis Rul", "unknown");
pub const IOM_AXIS_RDR: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RDR", "Axis Rdr", "unknown");
pub const IOM_AXIS_RDL: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RDL", "Axis Rdl", "unknown");
pub const IOM_AXIS_DPADX: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_DPADX", "Axis Dpadx", "unknown");
pub const IOM_AXIS_DPADY: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_DPADY", "Axis Dpady", "unknown");
pub const IOM_AXIS_LY_UP: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LY_UP", "Axis Ly Up", "unknown");
pub const IOM_AXIS_LY_DOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LY_DOWN", "Axis Ly Down", "unknown");
pub const IOM_AXIS_LX_LEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LX_LEFT", "Axis Lx Left", "unknown");
pub const IOM_AXIS_LX_RIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_LX_RIGHT", "Axis Lx Right", "unknown");
pub const IOM_AXIS_RY_UP: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RY_UP", "Axis Ry Up", "unknown");
pub const IOM_AXIS_RY_DOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RY_DOWN", "Axis Ry Down", "unknown");
pub const IOM_AXIS_RX_LEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RX_LEFT", "Axis Rx Left", "unknown");
pub const IOM_AXIS_RX_RIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_RX_RIGHT", "Axis Rx Right", "unknown");
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
