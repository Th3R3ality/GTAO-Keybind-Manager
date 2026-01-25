pub const IOMS_MOUSE_NORMALIZED: &'static(&'static str, &'static str, &'static str) = &("IOMS_MOUSE_NORMALIZED", "MOUSE_NORMALIZED", "Mouse axis input (normalized)");
pub const IOM_AXIS_X: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_X", "AXIS_X", "Mouse X Axis");
pub const IOM_AXIS_Y: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_Y", "AXIS_Y", "Mouse Y Axis");
pub const IOM_AXIS_WHEEL: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_WHEEL", "AXIS_WHEEL", "Mouse Wheel Axis");
pub const IOM_AXIS_WHEEL_DELTA: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_WHEEL_DELTA", "AXIS_WHEEL_DELTA", "Mouse Wheel Axis Delta");
pub const IOM_AXIS_WHEEL_RELATIVE: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_WHEEL_RELATIVE", "AXIS_WHEEL_RELATIVE", "Mouse Wheel Axis Relative");
pub const IOM_IAXIS_X: &'static(&'static str, &'static str, &'static str) = &("IOM_IAXIS_X", "IAXIS_X", "Mouse X Axis (Inverted)");
pub const IOM_IAXIS_Y: &'static(&'static str, &'static str, &'static str) = &("IOM_IAXIS_Y", "IAXIS_Y", "Mouse Y Axis (Inverted)");
pub const IOM_IAXIS_WHEEL: &'static(&'static str, &'static str, &'static str) = &("IOM_IAXIS_WHEEL", "IAXIS_WHEEL", "Mouse Wheel Axis (Inverted)");
pub const IOM_IAXIS_WHEEL_DELTA: &'static(&'static str, &'static str, &'static str) = &("IOM_IAXIS_WHEEL_DELTA", "IAXIS_WHEEL_DELTA", "Mouse Wheel Axis Delta (Inverted)");
pub const IOM_IAXIS_WHEEL_RELATIVE: &'static(&'static str, &'static str, &'static str) = &("IOM_IAXIS_WHEEL_RELATIVE", "IAXIS_WHEEL_RELATIVE", "Mouse Wheel Axis Relative (Inverted)");
pub const IOM_AXIS_X_LEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_X_LEFT", "AXIS_X_LEFT", "Mouse X Axis (Left Only)");
pub const IOM_AXIS_X_RIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_X_RIGHT", "AXIS_X_RIGHT", "Mouse X Axis (Right Only)");
pub const IOM_AXIS_Y_UP: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_Y_UP", "AXIS_Y_UP", "Mouse Y Axis (Up Only)");
pub const IOM_AXIS_Y_DOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_Y_DOWN", "AXIS_Y_DOWN", "Mouse Y Axis (Down Only)");
pub const BASIC_MOUSE_AXIS_MAX: &'static(&'static str, &'static str, &'static str) = &("BASIC_MOUSE_AXIS_MAX", "MOUSE_AXIS_MAX", "unknown");
pub const MOUSE_AXIS_MAX: &'static(&'static str, &'static str, &'static str) = &("MOUSE_AXIS_MAX", "AXIS_MAX", "unknown");
pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	IOMS_MOUSE_NORMALIZED, //Mouse axis input (normalized)
	IOM_AXIS_X, //Mouse X Axis
	IOM_AXIS_Y, //Mouse Y Axis
	IOM_AXIS_WHEEL, //Mouse Wheel Axis
	IOM_AXIS_WHEEL_DELTA, //Mouse Wheel Axis Delta
	IOM_AXIS_WHEEL_RELATIVE, //Mouse Wheel Axis Relative
	IOM_IAXIS_X, //Mouse X Axis (Inverted)
	IOM_IAXIS_Y, //Mouse Y Axis (Inverted)
	IOM_IAXIS_WHEEL, //Mouse Wheel Axis (Inverted)
	IOM_IAXIS_WHEEL_DELTA, //Mouse Wheel Axis Delta (Inverted)
	IOM_IAXIS_WHEEL_RELATIVE, //Mouse Wheel Axis Relative (Inverted)
	IOM_AXIS_X_LEFT, //Mouse X Axis (Left Only)
	IOM_AXIS_X_RIGHT, //Mouse X Axis (Right Only)
	IOM_AXIS_Y_UP, //Mouse Y Axis (Up Only)
	IOM_AXIS_Y_DOWN, //Mouse Y Axis (Down Only)
	BASIC_MOUSE_AXIS_MAX, //unknown
	MOUSE_AXIS_MAX, //unknown
];
