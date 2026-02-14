pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &("CONTEXT", "Context", "Context");
pub const CONTEXT: &'static(&'static str, &'static str, &'static str) = &("INPUT_CONTEXT", "Context", "Unknown");

pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[
	CATEGORY, // CONTEXT
	CONTEXT,// INPUT_CONTEXT
];
