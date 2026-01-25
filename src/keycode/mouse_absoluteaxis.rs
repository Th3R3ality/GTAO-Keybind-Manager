pub const CATEGORY: &'static(&'static str, &'static str) = &("IOMS_MOUSE_ABSOLUTEAXIS", "Mouse axis input (absolute)");
pub const AXIS_X: &'static(&'static str, &'static str) = &("IOM_AXIS_X", "Mouse X Axis");
pub const AXIS_Y: &'static(&'static str, &'static str) = &("IOM_AXIS_Y", "Mouse Y Axis");
pub const AXIS_WHEEL: &'static(&'static str, &'static str) = &("IOM_AXIS_WHEEL", "Mouse Wheel Axis");
pub const AXIS_WHEEL_DELTA: &'static(&'static str, &'static str) = &("IOM_AXIS_WHEEL_DELTA", "Mouse Wheel Axis Delta");
pub const AXIS_WHEEL_RELATIVE: &'static(&'static str, &'static str) = &("IOM_AXIS_WHEEL_RELATIVE", "Mouse Wheel Axis Relative");
pub const IAXIS_X: &'static(&'static str, &'static str) = &("IOM_IAXIS_X", "Mouse X Axis (Inverted)");
pub const IAXIS_Y: &'static(&'static str, &'static str) = &("IOM_IAXIS_Y", "Mouse Y Axis (Inverted)");
pub const IAXIS_WHEEL: &'static(&'static str, &'static str) = &("IOM_IAXIS_WHEEL", "Mouse Wheel Axis (Inverted)");
pub const IAXIS_WHEEL_DELTA: &'static(&'static str, &'static str) = &("IOM_IAXIS_WHEEL_DELTA", "Mouse Wheel Axis Delta (Inverted)");
pub const IAXIS_WHEEL_RELATIVE: &'static(&'static str, &'static str) = &("IOM_IAXIS_WHEEL_RELATIVE", "Mouse Wheel Axis Relative (Inverted)");
pub const AXIS_X_LEFT: &'static(&'static str, &'static str) = &("IOM_AXIS_X_LEFT", "Mouse X Axis (Left Only)");
pub const AXIS_X_RIGHT: &'static(&'static str, &'static str) = &("IOM_AXIS_X_RIGHT", "Mouse X Axis (Right Only)");
pub const AXIS_Y_UP: &'static(&'static str, &'static str) = &("IOM_AXIS_Y_UP", "Mouse Y Axis (Up Only)");
pub const AXIS_Y_DOWN: &'static(&'static str, &'static str) = &("IOM_AXIS_Y_DOWN", "Mouse Y Axis (Down Only)");
pub const MOUSE_AXIS_MAX: &'static(&'static str, &'static str) = &("BASIC_MOUSE_AXIS_MAX", "unknown");
pub const AXIS_MAX: &'static(&'static str, &'static str) = &("MOUSE_AXIS_MAX", "unknown");
pub const ALL: &'static[&'static(&'static str,&'static str)] = &[
	CATEGORY, //Mouse axis input (absolute)
	AXIS_X, //Mouse X Axis
	AXIS_Y, //Mouse Y Axis
	AXIS_WHEEL, //Mouse Wheel Axis
	AXIS_WHEEL_DELTA, //Mouse Wheel Axis Delta
	AXIS_WHEEL_RELATIVE, //Mouse Wheel Axis Relative
	IAXIS_X, //Mouse X Axis (Inverted)
	IAXIS_Y, //Mouse Y Axis (Inverted)
	IAXIS_WHEEL, //Mouse Wheel Axis (Inverted)
	IAXIS_WHEEL_DELTA, //Mouse Wheel Axis Delta (Inverted)
	IAXIS_WHEEL_RELATIVE, //Mouse Wheel Axis Relative (Inverted)
	AXIS_X_LEFT, //Mouse X Axis (Left Only)
	AXIS_X_RIGHT, //Mouse X Axis (Right Only)
	AXIS_Y_UP, //Mouse Y Axis (Up Only)
	AXIS_Y_DOWN, //Mouse Y Axis (Down Only)
	MOUSE_AXIS_MAX, //unknown
	AXIS_MAX, //unknown
];
