pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("CREATOR", "Creator", "Creator");
pub const CREATOR_ALT_MENU_ENTRY: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_ALT_MENU_ENTRY", "Creator Alt Menu Entry", "Unknown");
pub const CREATOR_MENU_ACCEPT: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_MENU_ACCEPT", "Creator Menu Accept", "Unknown");
pub const CREATOR_MENU_CANCEL: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_MENU_CANCEL", "Creator Menu Cancel", "Unknown");
pub const CREATOR_MENU_UP: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_MENU_UP", "Creator Menu Up", "Unknown");
pub const CREATOR_MENU_DOWN: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_MENU_DOWN", "Creator Menu Down", "Unknown");
pub const CREATOR_MENU_DELETE: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_MENU_DELETE", "Creator Menu Delete", "Unknown");
pub const CREATOR_EXTRA_OPTIONS1: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_EXTRA_OPTIONS1", "Creator Extra Options1", "Unknown");
pub const CREATOR_TAB_LEFT: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_TAB_LEFT", "Creator Tab Left", "Unknown");
pub const CREATOR_TAB_RIGHT: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_TAB_RIGHT", "Creator Tab Right", "Unknown");
pub const CREATOR_CONTEXT_MENU: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_CONTEXT_MENU", "Creator Context Menu", "Unknown");
pub const CREATOR_TOGGLE_DESCRIPTIONS: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_TOGGLE_DESCRIPTIONS", "Creator Toggle Descriptions", "Unknown");
pub const CREATOR_LIST: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_LIST", "Creator List", "Unknown");
pub const CREATOR_SWITCH_CAMERA: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_SWITCH_CAMERA", "Creator Switch Camera", "Unknown");
pub const CREATOR_TOGGLE_ORBIT_CAMERA: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_TOGGLE_ORBIT_CAMERA", "Creator Toggle Orbit Camera", "Unknown");
pub const CREATOR_FAST_ZOOM: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_FAST_ZOOM", "Creator Fast Zoom", "Unknown");
pub const CREATOR_CAMERA_DECREASE: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_CAMERA_DECREASE", "Creator Camera Decrease", "Unknown");
pub const CREATOR_CAMERA_INCREASE: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_CAMERA_INCREASE", "Creator Camera Increase", "Unknown");
pub const CREATOR_FOV_INCREASE: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_FOV_INCREASE", "Creator Fov Increase", "Unknown");
pub const CREATOR_FOV_DECREASE: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_FOV_DECREASE", "Creator Fov Decrease", "Unknown");
pub const CREATOR_RESET_FOV: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_RESET_FOV", "Creator Reset Fov", "Unknown");
pub const CREATOR_FAST_ADJUST: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_FAST_ADJUST", "Creator Fast Adjust", "Unknown");
pub const CREATOR_FINE_ADJUST: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_FINE_ADJUST", "Creator Fine Adjust", "Unknown");
pub const CREATOR_WARP_AND_EDIT: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_WARP_AND_EDIT", "Creator Warp And Edit", "Unknown");
pub const CREATOR_ROTATE_LEFT: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_ROTATE_LEFT", "Creator Rotate Left", "Unknown");
pub const CREATOR_ROTATE_RIGHT: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_ROTATE_RIGHT", "Creator Rotate Right", "Unknown");
pub const CREATOR_ADJUST_LEFT: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_ADJUST_LEFT", "Creator Adjust Left", "Unknown");
pub const CREATOR_ADJUST_RIGHT: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_ADJUST_RIGHT", "Creator Adjust Right", "Unknown");
pub const CREATOR_ENTITY_RAISE: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_ENTITY_RAISE", "Creator Entity Raise", "Unknown");
pub const CREATOR_ENTITY_LOWER: &'static(&'static str, &'static str, &'static str) = &("INPUT_CREATOR_ENTITY_LOWER", "Creator Entity Lower", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // CREATOR
	CREATOR_ALT_MENU_ENTRY,// INPUT_CREATOR_ALT_MENU_ENTRY
	CREATOR_MENU_ACCEPT,// INPUT_CREATOR_MENU_ACCEPT
	CREATOR_MENU_CANCEL,// INPUT_CREATOR_MENU_CANCEL
	CREATOR_MENU_UP,// INPUT_CREATOR_MENU_UP
	CREATOR_MENU_DOWN,// INPUT_CREATOR_MENU_DOWN
	CREATOR_MENU_DELETE,// INPUT_CREATOR_MENU_DELETE
	CREATOR_EXTRA_OPTIONS1,// INPUT_CREATOR_EXTRA_OPTIONS1
	CREATOR_TAB_LEFT,// INPUT_CREATOR_TAB_LEFT
	CREATOR_TAB_RIGHT,// INPUT_CREATOR_TAB_RIGHT
	CREATOR_CONTEXT_MENU,// INPUT_CREATOR_CONTEXT_MENU
	CREATOR_TOGGLE_DESCRIPTIONS,// INPUT_CREATOR_TOGGLE_DESCRIPTIONS
	CREATOR_LIST,// INPUT_CREATOR_LIST
	CREATOR_SWITCH_CAMERA,// INPUT_CREATOR_SWITCH_CAMERA
	CREATOR_TOGGLE_ORBIT_CAMERA,// INPUT_CREATOR_TOGGLE_ORBIT_CAMERA
	CREATOR_FAST_ZOOM,// INPUT_CREATOR_FAST_ZOOM
	CREATOR_CAMERA_DECREASE,// INPUT_CREATOR_CAMERA_DECREASE
	CREATOR_CAMERA_INCREASE,// INPUT_CREATOR_CAMERA_INCREASE
	CREATOR_FOV_INCREASE,// INPUT_CREATOR_FOV_INCREASE
	CREATOR_FOV_DECREASE,// INPUT_CREATOR_FOV_DECREASE
	CREATOR_RESET_FOV,// INPUT_CREATOR_RESET_FOV
	CREATOR_FAST_ADJUST,// INPUT_CREATOR_FAST_ADJUST
	CREATOR_FINE_ADJUST,// INPUT_CREATOR_FINE_ADJUST
	CREATOR_WARP_AND_EDIT,// INPUT_CREATOR_WARP_AND_EDIT
	CREATOR_ROTATE_LEFT,// INPUT_CREATOR_ROTATE_LEFT
	CREATOR_ROTATE_RIGHT,// INPUT_CREATOR_ROTATE_RIGHT
	CREATOR_ADJUST_LEFT,// INPUT_CREATOR_ADJUST_LEFT
	CREATOR_ADJUST_RIGHT,// INPUT_CREATOR_ADJUST_RIGHT
	CREATOR_ENTITY_RAISE,// INPUT_CREATOR_ENTITY_RAISE
	CREATOR_ENTITY_LOWER,// INPUT_CREATOR_ENTITY_LOWER
];
