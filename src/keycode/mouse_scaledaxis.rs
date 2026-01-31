pub const IOMS_MOUSE_SCALEDAXIS: &'static(&'static str, &'static str, &'static str) = &("IOMS_MOUSE_SCALEDAXIS", "Mouse Scaledaxis", "Mouse axis input (scaled)");
pub const IOM_AXIS_X: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_X", "Axis X", "Mouse X Axis");
pub const IOM_AXIS_Y: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_Y", "Axis Y", "Mouse Y Axis");
pub const IOM_AXIS_WHEEL: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_WHEEL", "Axis Wheel", "Mouse Wheel Axis");
pub const IOM_AXIS_WHEEL_DELTA: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_WHEEL_DELTA", "Axis Wheel Delta", "Mouse Wheel Axis Delta");
pub const IOM_AXIS_WHEEL_RELATIVE: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_WHEEL_RELATIVE", "Axis Wheel Relative", "Mouse Wheel Axis Relative");
pub const IOM_IAXIS_X: &'static(&'static str, &'static str, &'static str) = &("IOM_IAXIS_X", "Iaxis X", "Mouse X Axis (Inverted)");
pub const IOM_IAXIS_Y: &'static(&'static str, &'static str, &'static str) = &("IOM_IAXIS_Y", "Iaxis Y", "Mouse Y Axis (Inverted)");
pub const IOM_IAXIS_WHEEL: &'static(&'static str, &'static str, &'static str) = &("IOM_IAXIS_WHEEL", "Iaxis Wheel", "Mouse Wheel Axis (Inverted)");
pub const IOM_IAXIS_WHEEL_DELTA: &'static(&'static str, &'static str, &'static str) = &("IOM_IAXIS_WHEEL_DELTA", "Iaxis Wheel Delta", "Mouse Wheel Axis Delta (Inverted)");
pub const IOM_IAXIS_WHEEL_RELATIVE: &'static(&'static str, &'static str, &'static str) = &("IOM_IAXIS_WHEEL_RELATIVE", "Iaxis Wheel Relative", "Mouse Wheel Axis Relative (Inverted)");
pub const IOM_AXIS_X_LEFT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_X_LEFT", "Axis X Left", "Mouse X Axis (Left Only)");
pub const IOM_AXIS_X_RIGHT: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_X_RIGHT", "Axis X Right", "Mouse X Axis (Right Only)");
pub const IOM_AXIS_Y_UP: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_Y_UP", "Axis Y Up", "Mouse Y Axis (Up Only)");
pub const IOM_AXIS_Y_DOWN: &'static(&'static str, &'static str, &'static str) = &("IOM_AXIS_Y_DOWN", "Axis Y Down", "Mouse Y Axis (Down Only)");
pub const BASIC_MOUSE_AXIS_MAX: &'static(&'static str, &'static str, &'static str) = &("BASIC_MOUSE_AXIS_MAX", "Mouse Axis Max", "unknown");
pub const MOUSE_AXIS_MAX: &'static(&'static str, &'static str, &'static str) = &("MOUSE_AXIS_MAX", "Axis Max", "unknown");
pub const NULL: &'static(&'static str, &'static str, &'static str) = &("KEY_NULL", "NULL", "Unbound"); // hardcoded in keycodes_to_rust.py
pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	IOMS_MOUSE_SCALEDAXIS, // Mouse axis input (scaled)
	IOM_AXIS_X, // Mouse X Axis
	IOM_AXIS_Y, // Mouse Y Axis
	IOM_AXIS_WHEEL, // Mouse Wheel Axis
	IOM_AXIS_WHEEL_DELTA, // Mouse Wheel Axis Delta
	IOM_AXIS_WHEEL_RELATIVE, // Mouse Wheel Axis Relative
	IOM_IAXIS_X, // Mouse X Axis (Inverted)
	IOM_IAXIS_Y, // Mouse Y Axis (Inverted)
	IOM_IAXIS_WHEEL, // Mouse Wheel Axis (Inverted)
	IOM_IAXIS_WHEEL_DELTA, // Mouse Wheel Axis Delta (Inverted)
	IOM_IAXIS_WHEEL_RELATIVE, // Mouse Wheel Axis Relative (Inverted)
	IOM_AXIS_X_LEFT, // Mouse X Axis (Left Only)
	IOM_AXIS_X_RIGHT, // Mouse X Axis (Right Only)
	IOM_AXIS_Y_UP, // Mouse Y Axis (Up Only)
	IOM_AXIS_Y_DOWN, // Mouse Y Axis (Down Only)
	BASIC_MOUSE_AXIS_MAX, // unknown
	MOUSE_AXIS_MAX, // unknown
	NULL, // hardcoded in keycodes_to_rust.py
];
