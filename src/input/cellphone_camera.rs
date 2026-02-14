pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("CELLPHONE_CAMERA", "Cellphone Camera", "Cellphone Camera");
pub const CELLPHONE_CAMERA_SELFIE : &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_CAMERA_SELFIE ", "Cellphone Camera Selfie ", "Unknown");
pub const CELLPHONE_CAMERA_EXPRESSION: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_CAMERA_EXPRESSION", "Cellphone Camera Expression", "Unknown");
pub const CELLPHONE_CAMERA_GRID: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_CAMERA_GRID", "Cellphone Camera Grid", "Unknown");
pub const CELLPHONE_CAMERA_DOF: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_CAMERA_DOF", "Cellphone Camera Dof", "Unknown");
pub const CELLPHONE_CAMERA_FOCUS_LOCK: &'static(&'static str, &'static str, &'static str) = &("INPUT_CELLPHONE_CAMERA_FOCUS_LOCK", "Cellphone Camera Focus Lock", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // CELLPHONE_CAMERA
	CELLPHONE_CAMERA_SELFIE ,// INPUT_CELLPHONE_CAMERA_SELFIE 
	CELLPHONE_CAMERA_EXPRESSION,// INPUT_CELLPHONE_CAMERA_EXPRESSION
	CELLPHONE_CAMERA_GRID,// INPUT_CELLPHONE_CAMERA_GRID
	CELLPHONE_CAMERA_DOF,// INPUT_CELLPHONE_CAMERA_DOF
	CELLPHONE_CAMERA_FOCUS_LOCK,// INPUT_CELLPHONE_CAMERA_FOCUS_LOCK
];
